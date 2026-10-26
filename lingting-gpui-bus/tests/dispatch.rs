//! 全局总线安装与后台分发者。

mod common;

use common::InstallFinished;
use framework_bus::Bus;
use gpui_kit::TestAppContext;
use lingting_gpui_bus::{BusAppExt as _, GpuiBackgroundDispatcher};
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

#[gpui_kit::test]
fn install_and_read_bus(cx: &mut TestAppContext) {
    let bus = Bus::new();
    cx.update(|cx| cx.install_bus(bus.clone()));

    cx.update(|cx| {
        // 浅克隆：同一份订阅表
        assert_eq!(cx.bus().subscriber_count::<InstallFinished>(), 0);
        bus.notify::<InstallFinished>();
        assert!(cx.try_bus().is_some());
    });
}

#[test]
fn try_bus_is_none_before_install() {
    let mut app = gpui_kit::TestApp::new();
    app.update(|cx| assert!(cx.try_bus().is_none()));
}

#[gpui_kit::test]
fn background_dispatcher_runs_handler_off_main_thread(cx: &mut TestAppContext) {
    let runs = Arc::new(AtomicUsize::new(0));
    let on_main_thread = Arc::new(AtomicUsize::new(0));
    let background = cx.update(|cx| {
        let bus = Bus::new();
        bus.set_dispatcher(Arc::new(GpuiBackgroundDispatcher::new(cx)));
        bus
    });
    let foreground_executor = cx.background_executor.clone();

    let counter = runs.clone();
    let main_flag = on_main_thread.clone();
    let _sub = background.subscribe::<InstallFinished>(move || {
        counter.fetch_add(1, Ordering::SeqCst);
        if foreground_executor.is_main_thread() {
            main_flag.fetch_add(1, Ordering::SeqCst);
        }
    });

    background.notify::<InstallFinished>();
    cx.run_until_parked();

    assert_eq!(runs.load(Ordering::SeqCst), 1);
    assert_eq!(
        on_main_thread.load(Ordering::SeqCst),
        0,
        "handler should not run on the GPUI foreground thread"
    );
}

#[gpui_kit::test]
fn background_dispatcher_coalesces_bursts(cx: &mut TestAppContext) {
    let runs = Arc::new(AtomicUsize::new(0));
    let background = cx.update(|cx| {
        let bus = Bus::new();
        bus.set_dispatcher(Arc::new(GpuiBackgroundDispatcher::new(cx)));
        bus
    });

    let counter = runs.clone();
    let _sub = background.subscribe::<InstallFinished>(move || {
        counter.fetch_add(1, Ordering::SeqCst);
    });

    const TIMES: usize = 8;
    for _ in 0..TIMES {
        background.notify::<InstallFinished>();
    }
    cx.run_until_parked();

    let runs = runs.load(Ordering::SeqCst);
    assert!((1..=TIMES).contains(&runs), "unexpected runs: {runs}");
}
