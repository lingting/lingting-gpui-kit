//! 全局持有总线，并提供 [`BusAppExt`] 的安装与读取入口。

use framework_bus::Bus;
use gpui_kit::{App, Global};

/// 全局总线。
///
/// 由 [`BusAppExt::install_bus`] 写入 `App` 的全局状态，页面用
/// [`BusContextExt::on_global_bus_notify`](crate::BusContextExt::on_global_bus_notify)
/// 或 [`BusAppExt::bus`] 取用。
pub struct GlobalBus(pub Bus);

impl Global for GlobalBus {}

/// 在 `App` 上安装与读取总线。
///
/// `Context<T>` 通过 `Deref<Target = App>` 同样可以直接调用这些方法。
pub trait BusAppExt {
    /// 安装总线，覆盖之前安装的同类型全局状态。
    ///
    /// 总线是浅克隆，页面拿到的 `Bus` 与业务模块发出通知用的是同一份订阅表。
    fn install_bus(&mut self, bus: Bus);

    /// 取出总线。
    ///
    /// 未安装时 panic，错误信息指向 [`install_bus`](Self::install_bus)。
    fn bus(&self) -> Bus;

    /// 取出总线，未安装时返回 `None`。
    fn try_bus(&self) -> Option<Bus>;
}

impl BusAppExt for App {
    fn install_bus(&mut self, bus: Bus) {
        self.set_global(GlobalBus(bus));
    }

    fn bus(&self) -> Bus {
        self.try_bus().unwrap_or_else(|| {
            panic!("lingting-gpui-bus: 尚未安装总线，请在启动阶段调用 cx.install_bus(bus.clone())")
        })
    }

    fn try_bus(&self) -> Option<Bus> {
        self.try_global::<GlobalBus>()
            .map(|global| global.0.clone())
    }
}
