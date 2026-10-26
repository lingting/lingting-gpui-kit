//! GPUI 与 `framework-bus` 之间的桥接层。
//!
//! [`framework_bus`] 与 UI 框架无关：它只保存订阅关系，通知不携带数据，处理函数在哪里执行由
//! [`Dispatcher`](framework_bus::Dispatcher) 决定。本 crate 补上 GPUI 需要的两件事：
//!
//! - 页面级订阅：[`BusContextExt::on_bus_notify`] 把「总线通知」变成「在前台线程调用
//!   `&mut Self` 与 `Context<Self>` 的回调」，句柄是 GPUI 的 [`Subscription`](gpui_kit::Subscription)，
//!   页面销毁或句柄 drop 时自动退订。
//! - 后台分发者：[`GpuiBackgroundDispatcher`] 让业务侧用
//!   [`Bus::subscribe`](framework_bus::Bus::subscribe) 注册的 `Send + Sync` 处理函数在 GPUI
//!   后台 executor 上执行。
//!
//! GPUI 的 API 变动被隔离在本 crate 内，升级 gpui 只需要改这里。
//!
//! # 适配的 GPUI 版本
//!
//! 依赖的是 `gpui-kit 0.7`（`gpui-fast` 0.1.8）暴露的 GPUI API：`App`、`Context<T>`、`Entity`、
//! `WeakEntity`、`Context::spawn(async move |this, cx| ...)`、`Subscription::new`、
//! `BackgroundExecutor`、`Global`。
//!
//! # 设计原则
//!
//! - 通知是纯通知，不带数据。页面被唤醒后自己读取最新状态。
//! - 页面级回调是 GPUI 的非 `Send` 闭包，不注册进总线的处理函数表；它通过
//!   [`Bus::watch`](framework_bus::Bus::watch) 拿到合并唤醒句柄，由前台 task 等待唤醒后回到前台
//!   线程执行回调。
//! - 没有事件队列，也没有积压补偿的概念：多次通知可能只触发一次回调，处理必须幂等。
//!
//! # 使用
//!
//! 启动时安装总线与分发者：
//!
//! ```ignore
//! use lingting_gpui_bus::{BusAppExt as _, GpuiBackgroundDispatcher};
//!
//! let bus = Bus::new();
//! cx.install_bus(bus.clone());
//! // 业务侧 subscribe 的处理函数走后台 executor
//! bus.set_dispatcher(Arc::new(GpuiBackgroundDispatcher::new(cx)));
//! ```
//!
//! 页面里订阅：
//!
//! ```ignore
//! struct ConsolePage {
//!     subs: Vec<Subscription>,
//!     installed: Vec<InstalledItem>,
//! }
//!
//! impl ConsolePage {
//!     fn new(cx: &mut Context<Self>) -> Self {
//!         // 先订阅，再加载当前状态，二者之间的通知不会漏
//!         let sub = cx.on_global_bus_notify::<InstallFinished>(|this, cx| {
//!             this.reload_installed(cx);
//!             cx.notify();
//!         });
//!         let mut this = Self {
//!             subs: vec![sub],
//!             installed: Vec::new(),
//!         };
//!         this.reload_installed(cx);
//!         this
//!     }
//! }
//! ```

#![deny(missing_docs)]

mod context;
mod dispatcher;
mod global;

pub use context::BusContextExt;
pub use dispatcher::GpuiBackgroundDispatcher;
pub use global::{BusAppExt, GlobalBus};
