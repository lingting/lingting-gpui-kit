//! 页面订阅的行为：前台回调、合并、互不串扰、退订、关闭、线程。

mod common;

use common::{InstallFinished, Page, TaskChanged, always_foreground, hits, install};
use framework_bus::Bus;
use gpui_kit::{AppContext as _, Subscription, TestAppContext};
use lingting_gpui_bus::BusContextExt as _;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;

#[gpui_kit::test]
fn notify_reaches_page_on_foreground(cx: &mut TestAppContext) {
    let bus = Bus::new();
    cx.update(|cx| install(cx, &bus));

    let page = cx.new(Page::new::<InstallFinished>);
    bus.notify::<InstallFinished>();
    cx.run_until_parked();

    assert_eq!(hits(&page, cx), 1);
    assert!(always_foreground(&page, cx));
}

#[gpui_kit::test]
fn notify_right_after_subscribe_is_not_lost(cx: &mut TestAppContext) {
    let bus = Bus::new();
    cx.update(|cx| install(cx, &bus));

    // 页面构造返回后立刻通知，先 watch 再 spawn 才收得到
    let page = cx.new(Page::new::<InstallFinished>);
    bus.notify::<InstallFinished>();
    cx.run_until_parked();

    assert_eq!(hits(&page, cx), 1);
}

#[gpui_kit::test]
fn bursts_are_coalesced(cx: &mut TestAppContext) {
    let bus = Bus::new();
    cx.update(|cx| install(cx, &bus));

    let page = cx.new(Page::new::<InstallFinished>);
    cx.run_until_parked();

    const TIMES: usize = 8;
    for _ in 0..TIMES {
        bus.notify::<InstallFinished>();
    }
    cx.run_until_parked();

    let hits = hits(&page, cx);
    assert!((1..=TIMES).contains(&hits), "unexpected hits: {hits}");
}

#[gpui_kit::test]
fn pages_only_hear_their_own_event(cx: &mut TestAppContext) {
    let bus = Bus::new();
    cx.update(|cx| install(cx, &bus));

    let install_page = cx.new(Page::new::<InstallFinished>);
    let task_page = cx.new(Page::new::<TaskChanged>);
    cx.run_until_parked();

    bus.notify::<InstallFinished>();
    cx.run_until_parked();

    assert_eq!(hits(&install_page, cx), 1);
    assert_eq!(hits(&task_page, cx), 0);

    bus.notify::<TaskChanged>();
    cx.run_until_parked();

    assert_eq!(hits(&install_page, cx), 1);
    assert_eq!(hits(&task_page, cx), 1);
}

#[gpui_kit::test]
fn two_pages_on_same_event_both_fire(cx: &mut TestAppContext) {
    let bus = Bus::new();
    cx.update(|cx| install(cx, &bus));

    let first = cx.new(Page::new::<InstallFinished>);
    let second = cx.new(Page::new::<InstallFinished>);
    cx.run_until_parked();

    bus.notify::<InstallFinished>();
    cx.run_until_parked();

    assert_eq!(hits(&first, cx), 1);
    assert_eq!(hits(&second, cx), 1);
    assert_eq!(bus.subscriber_count::<InstallFinished>(), 2);
}

#[gpui_kit::test]
fn dropping_subscription_unsubscribes(cx: &mut TestAppContext) {
    let bus = Bus::new();
    cx.update(|cx| install(cx, &bus));

    let page = cx.new(Page::new::<InstallFinished>);
    cx.run_until_parked();
    assert_eq!(bus.subscriber_count::<InstallFinished>(), 1);

    page.update(cx, |page, _| page.sub = None);
    cx.run_until_parked();

    assert_eq!(bus.subscriber_count::<InstallFinished>(), 0);

    bus.notify::<InstallFinished>();
    cx.run_until_parked();
    assert_eq!(hits(&page, cx), 0);
}

#[gpui_kit::test]
fn dropping_page_unsubscribes(cx: &mut TestAppContext) {
    let bus = Bus::new();
    cx.update(|cx| install(cx, &bus));

    let page = cx.new(Page::new::<InstallFinished>);
    cx.run_until_parked();
    assert_eq!(bus.subscriber_count::<InstallFinished>(), 1);

    // 实体在 app 刷新时真正释放，页面里的 Subscription 随之 drop
    drop(page);
    cx.update(|_| {});
    cx.run_until_parked();

    assert_eq!(bus.subscriber_count::<InstallFinished>(), 0);
}

#[gpui_kit::test]
fn notify_from_background_thread_still_hits_foreground(cx: &mut TestAppContext) {
    let bus = Bus::new();
    cx.update(|cx| install(cx, &bus));

    let page = cx.new(Page::new::<InstallFinished>);
    cx.run_until_parked();

    // 测试调度器只允许在主线程上调度，后台线程必须先放行停车
    cx.background_executor.allow_parking();
    let notifier = bus.clone();
    thread::spawn(move || notifier.notify::<InstallFinished>())
        .join()
        .expect("notify thread panicked");
    cx.run_until_parked();

    assert_eq!(hits(&page, cx), 1);
    assert!(always_foreground(&page, cx));
}

#[gpui_kit::test]
fn close_ends_subscription_and_later_notify_is_safe(cx: &mut TestAppContext) {
    let bus = Bus::new();
    cx.update(|cx| install(cx, &bus));

    let page = cx.new(Page::new::<InstallFinished>);
    cx.run_until_parked();

    bus.close();
    cx.run_until_parked();

    // 关闭后总线清空订阅表，前台 task 也随之结束
    assert_eq!(bus.subscriber_count::<InstallFinished>(), 0);

    bus.notify::<InstallFinished>();
    cx.run_until_parked();
    assert_eq!(hits(&page, cx), 0);
}

#[gpui_kit::test]
fn explicit_bus_subscription_works(cx: &mut TestAppContext) {
    let bus = Bus::new();

    // 不安装全局总线，直接传 Bus
    let page = cx.new(|cx| Page::with_bus::<InstallFinished>(&bus, cx));
    cx.run_until_parked();

    bus.notify::<InstallFinished>();
    cx.run_until_parked();

    assert_eq!(hits(&page, cx), 1);
}

/// 只持有一个订阅的页面，用于观察回调副作用。
struct Relay {
    _sub: Subscription,
}

#[gpui_kit::test]
fn callback_may_notify_other_event(cx: &mut TestAppContext) {
    let bus = Bus::new();
    cx.update(|cx| install(cx, &bus));

    let triggered = Arc::new(AtomicUsize::new(0));
    let counter = triggered.clone();
    let notifier = bus.clone();
    let _source = cx.new(|cx| {
        let sub = cx.on_bus_notify::<InstallFinished>(&bus, move |_, _| {
            counter.fetch_add(1, Ordering::SeqCst);
            // 回调内再发另一种通知，不应死锁或 panic
            notifier.notify::<TaskChanged>();
        });
        Relay { _sub: sub }
    });
    let target = cx.new(Page::new::<TaskChanged>);
    cx.run_until_parked();

    bus.notify::<InstallFinished>();
    cx.run_until_parked();

    assert_eq!(triggered.load(Ordering::SeqCst), 1);
    assert_eq!(hits(&target, cx), 1);
}
