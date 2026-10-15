//! 路由与页面出口。
//!
//! 使用方式：
//!
//! 1. 创建窗口之前调用 [`init`] 注册路由表，用 [`Router::add_cache`] 注册需要
//!    保留状态的页面，用 [`Router::add_view`] 注册每次进入都重新创建的页面；
//! 2. 需要显示页面的位置放置 [`Outlet`]；
//! 3. 通过 [`navigate`] 切换路径。

mod route;
mod router;
mod state;

pub use route::Router;
pub use router::{Outlet, init};
pub use state::{current_path, navigate};
