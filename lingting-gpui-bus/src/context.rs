//! 页面级订阅：把总线通知变成前台线程上的 `&mut Self` 回调。

use crate::BusAppExt as _;
use framework_bus::Bus;
use gpui_kit::{Context, Subscription, WeakEntity};

/// 扩展 `Context<T>`：页面订阅总线通知。
///
/// 回调在 GPUI 前台线程执行，因此可以安全地修改页面状态并调用 `cx.notify()`。
/// 同一页面同一订阅的回调不会被并发执行，并且天然合并：两次回调之间到达的多次通知只会触发一次。
///
/// 返回的 [`Subscription`] 必须由调用方保存（例如 `Vec<Subscription>`）：
/// drop 它会在同一时刻取消前台 task 与总线订阅。
pub trait BusContextExt<T: 'static> {
    /// 订阅 `E` 类通知，收到后在前台线程调用 `f`。
    ///
    /// `bus` 通常来自 [`BusAppExt::bus`](crate::BusAppExt::bus)，需要显式传入时用这个方法；
    /// 总线已装在全局状态里时用
    /// [`on_global_bus_notify`](Self::on_global_bus_notify) 更省事。
    ///
    /// 订阅在 `spawn` 之前同步建立，函数返回后立刻发出的通知不会漏。总线关闭或页面销毁后
    /// 前台 task 自行结束，回调不再执行。
    fn on_bus_notify<E: 'static>(
        &mut self,
        bus: &Bus,
        f: impl Fn(&mut T, &mut Context<T>) + 'static,
    ) -> Subscription;

    /// 同上，但从全局状态取总线，需先调用
    /// [`install_bus`](crate::BusAppExt::install_bus)。
    ///
    /// 未安装总线时 panic，信息指向 `install_bus`。
    fn on_global_bus_notify<E: 'static>(
        &mut self,
        f: impl Fn(&mut T, &mut Context<T>) + 'static,
    ) -> Subscription;
}

impl<T: 'static> BusContextExt<T> for Context<'_, T> {
    fn on_bus_notify<E: 'static>(
        &mut self,
        bus: &Bus,
        f: impl Fn(&mut T, &mut Context<T>) + 'static,
    ) -> Subscription {
        // 必须在 spawn 之前同步订阅：函数返回之后发出的通知不能漏
        let mut watcher = bus.watch::<E>();

        let task = self.spawn(async move |this, cx| {
            while watcher.changed().await.is_ok() {
                if !invoke(&this, cx, &f) {
                    // 页面已销毁，退订由 watcher 的 drop 完成
                    break;
                }
            }
        });

        // task 被取消时 watcher 随之 drop，总线上的订阅同时失效
        Subscription::new(move || drop(task))
    }

    fn on_global_bus_notify<E: 'static>(
        &mut self,
        f: impl Fn(&mut T, &mut Context<T>) + 'static,
    ) -> Subscription {
        let bus = self.bus();
        self.on_bus_notify::<E>(&bus, f)
    }
}

/// 在前台线程用当前页面状态执行回调。
///
/// 页面已销毁时返回 `false`，由调用方结束等待循环。
fn invoke<T: 'static>(
    this: &WeakEntity<T>,
    cx: &mut gpui_kit::AsyncApp,
    f: &impl Fn(&mut T, &mut Context<T>),
) -> bool {
    this.update(cx, |this, cx| f(this, cx)).is_ok()
}
