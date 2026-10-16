use gpui_kit::*;

use crate::router::{RouteDefinition, init_router as register};

/// 注册路由表并进入默认路由。必须在创建窗口之前调用。
///
/// 转发到布局通用的 `crate::router::init_router`；后续本布局需要扩展路由注册时
/// 只改这里，不用修改调用方。
pub fn init_router(
    cx: &mut App,
    default_path: impl Into<SharedString>,
    routes: &[RouteDefinition],
) {
    register(cx, default_path, routes);
}
