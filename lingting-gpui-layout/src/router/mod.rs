// 布局通用的路由定义与注册。两个布局共用同一份路由逻辑，各自的 `init_router`
// 只是转发到这里的 `init_router`，便于后续按布局扩展而不用修改调用方。
mod route;

pub use route::RouteDefinition;
pub(crate) use route::init_router;
