use gpui_kit::*;

use super::route::Router;
use super::state;

/// 页面出口。
///
/// 把它放进布局里需要显示页面的位置即可。它会订阅当前路由状态，
/// 路径变化时自动重新渲染。
pub struct Outlet {
    /// 保持订阅存活，否则路径变化时不会重新渲染。
    _subscription: Subscription,
}

impl Outlet {
    pub fn new(cx: &mut Context<Self>) -> Self {
        Self {
            _subscription: cx.observe_global::<state::RoutePath>(|_, cx| cx.notify()),
        }
    }
}

impl Render for Outlet {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        match state::page_for_current_path(window, cx) {
            // 页面拿到确定的宽高，铺满内容区后自行管理内部滚动；超出部分在此裁剪，
            // 不会溢出到相邻区域上。
            Some(view) => div()
                .size_full()
                .overflow_hidden()
                .child(view)
                .into_any_element(),
            None => div().size_full().overflow_hidden().into_any_element(),
        }
    }
}

/// 注册全局路由表。必须在创建窗口之前调用。
pub fn init(cx: &mut App, configure: impl FnOnce(&mut Router)) {
    state::init(cx, configure);
}
