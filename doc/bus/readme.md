# bus

`lingting-gpui-bus`：`framework-bus` 与 GPUI 之间的桥接层。总线本身与 UI 框架无关（依赖只有
`log` 与只开 `sync` 的 `tokio`），本 crate 只补上 GPUI 需要的两件事：

- **页面级订阅**：把「总线通知」变成「在前台线程调用 `&mut Self` 与 `Context<Self>` 的回调」，
  句柄是 GPUI 的 `Subscription`，页面销毁或句柄 drop 时自动退订。
- **后台分发者**：让业务侧用 `bus.subscribe` 注册的 `Send + Sync` 处理函数在 GPUI 后台 executor 上执行。

依赖只有 `framework-bus` 与 `gpui-kit`（与其余 crate 一致，不单独指定 gpui 版本）。适配的 GPUI
是 `gpui-kit 0.7` 暴露的 API，即 `gpui-fast 0.1.8`：`App`、`Context<T>`、`Entity`、`WeakEntity`、
`Context::spawn(async move |this, cx| ...)`、`Subscription::new`、`BackgroundExecutor`、`Global`。
GPUI 的 API 变动被隔离在本 crate 内，升级 gpui 只需要改这里。

## 设计原则

- **纯通知**：通知不携带数据，页面被唤醒后自己读取最新状态。
- **页面回调不进总线**：页面回调是 GPUI 的非 `Send` 闭包，无法注册进总线的处理函数表。它通过
  `Bus::watch::<E>()` 拿到合并唤醒句柄，由前台 task 等待唤醒后回到前台线程执行回调。
- **无队列**：没有事件队列，也没有积压补偿的概念。多次通知可能只触发一次回调，处理必须幂等。

## 提供的 API

| 项 | 作用 |
| --- | --- |
| `BusContextExt::on_bus_notify::<E>(&Bus, f)` | 订阅 `E`，收到后在前台线程调用 `f(&mut Self, &mut Context<Self>)` |
| `BusContextExt::on_global_bus_notify::<E>(f)` | 同上，总线从全局状态取 |
| `GlobalBus(Bus)` | 全局总线，实现 `Global` |
| `BusAppExt::install_bus` / `bus` / `try_bus` | 在 `App` 上安装与读取总线；`Context<T>` 通过 `Deref<Target = App>` 直接可用 |
| `GpuiBackgroundDispatcher::new(&App)` | 基于 GPUI 后台 executor 的 `Dispatcher` |

订阅生命周期由返回的 `Subscription` 管理：drop 它会在同一时刻取消前台 task 与总线订阅。

## 使用

启动时：

```rust
let bus = Bus::new();
cx.install_bus(bus.clone());
// 业务侧 subscribe 的处理函数走后台 executor
bus.set_dispatcher(Arc::new(GpuiBackgroundDispatcher::new(cx)));

// 把 bus 注入业务模块
task::init(bus.clone());
```

页面中：

```rust
struct ConsolePage {
    _subs: Vec<Subscription>,
    installed: Vec<InstalledItem>,
}

impl ConsolePage {
    fn new(cx: &mut Context<Self>) -> Self {
        // 先订阅，再加载当前状态：二者之间的通知不会漏
        let sub = cx.on_global_bus_notify::<InstallFinished>(|this, cx| {
            this.reload_installed(cx);
            cx.notify();
        });
        let mut this = Self {
            _subs: vec![sub],
            installed: Vec::new(),
        };
        this.reload_installed(cx);
        this
    }
}
```

## 取舍与注意点

- **回调合并**：同一页面同一订阅的回调不会被并发执行；两次回调之间到达的多次通知只触发一次。
  回调必须幂等，不能依赖被调用次数。
- **先订阅再加载**：`on_bus_notify` 在 `spawn` 之前同步调用 `bus.watch`，因此函数返回后立刻发出的
  通知不会漏。页面里应保持「先订阅、再读当前状态」的顺序。
- **页面销毁自动退订**：页面实体销毁后 `this.update` 返回 `Err`，前台 task 退出，`Watcher` 随之
  drop，总线上的订阅一并失效。总线 `close()` 时 `changed()` 返回 `Err(Closed)`，task 同样退出，
  之后再 `notify` 只是打警告日志。
- **后台分发者不能碰 UI**：`GpuiBackgroundDispatcher` 的处理函数在 GPUI 后台线程执行，不能访问
  `App` / `Entity` / `Context`。需要更新 UI 的逻辑一律用 `on_bus_notify`。
- **不可见页面建议只置脏**：页面在后台缓存、不可见时，回调里只置 `dirty = true`，等切回前台再刷新，
  避免对看不见的页面做无用功。这是推荐写法，库不做成代码。
- 业务 crate **不得**依赖本 crate，只有 UI 层依赖；`lingting-gpui-kit` 会再导出本 crate。
