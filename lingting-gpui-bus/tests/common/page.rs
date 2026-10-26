//! 测试共用的页面与断言辅助。

use framework_bus::Bus;
use gpui_kit::{Context, Subscription, TestAppContext};
use lingting_gpui_bus::{BusAppExt as _, BusContextExt as _};

/// 记录通知次数与回调线程的页面。
pub struct Page {
    /// 回调被执行的次数。
    pub hits: usize,
    /// 每次回调是否都在 GPUI 前台线程。
    pub always_foreground: bool,
    /// 页面持有订阅，drop 即退订。
    pub sub: Option<Subscription>,
}

impl Page {
    /// 从全局总线订阅 `E`，回调里计数并记录线程位置。
    pub fn new<E: 'static>(cx: &mut Context<Self>) -> Self {
        let sub = cx.on_global_bus_notify::<E>(|this, cx| this.hit(cx));
        Self::holding(sub)
    }

    /// 从显式传入的总线订阅 `E`。
    pub fn with_bus<E: 'static>(bus: &Bus, cx: &mut Context<Self>) -> Self {
        let sub = cx.on_bus_notify::<E>(bus, |this, cx| this.hit(cx));
        Self::holding(sub)
    }

    fn holding(sub: Subscription) -> Self {
        Self {
            hits: 0,
            always_foreground: true,
            sub: Some(sub),
        }
    }

    /// 回调内容：计数并记录是否在前台线程。
    pub fn hit(&mut self, cx: &mut Context<Self>) {
        self.hits += 1;
        self.always_foreground &= cx.background_executor().is_main_thread();
    }
}

/// 安装总线到全局状态。
pub fn install(cx: &mut gpui_kit::App, bus: &Bus) {
    cx.install_bus(bus.clone());
}

/// 读取页面的回调次数。
pub fn hits(page: &gpui_kit::Entity<Page>, cx: &TestAppContext) -> usize {
    page.read_with(cx, |page, _| page.hits)
}

/// 页面是否每次回调都在前台线程。
pub fn always_foreground(page: &gpui_kit::Entity<Page>, cx: &TestAppContext) -> bool {
    page.read_with(cx, |page, _| page.always_foreground)
}
