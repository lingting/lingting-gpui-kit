//! 基于 GPUI 后台 executor 的分发者。

use framework_bus::{Dispatcher, Job};
use gpui_kit::{App, BackgroundExecutor};

/// 把总线任务投递到 GPUI 后台 executor 的分发者。
///
/// 供业务侧通过 [`Bus::subscribe`](framework_bus::Bus::subscribe) 注册的
/// `Send + Sync` 处理函数使用。处理函数在 GPUI 后台线程执行，**不能**访问
/// `App` / `Entity` / `Context`；只适合刷新业务缓存、投递后台任务这类轻量逻辑。
/// 需要更新 UI 的逻辑一律用
/// [`BusContextExt::on_bus_notify`](crate::BusContextExt::on_bus_notify)。
///
/// 合并语义由总线保证：任务开始执行之前到达的多次通知只触发一次。
pub struct GpuiBackgroundDispatcher {
    executor: BackgroundExecutor,
}

impl GpuiBackgroundDispatcher {
    /// 从当前 `App` 取后台 executor。
    ///
    /// executor 是句柄，克隆代价很低，构造完成后不再需要持有 `App`。
    pub fn new(cx: &App) -> Self {
        Self {
            executor: cx.background_executor().clone(),
        }
    }
}

impl Dispatcher for GpuiBackgroundDispatcher {
    fn dispatch(&self, job: Job) {
        // detach：任务不返回结果，也不由本 crate 保存句柄
        self.executor.spawn(async move { job.run() }).detach();
    }
}
