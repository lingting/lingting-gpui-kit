# nav

`lingting-gpui-nav`：领域无关的路由与页面出口。它只维护「路径 → 页面」的映射和页面缓存，
不关心页面内容，也不提供具体布局。

## 使用流程

1. 创建窗口之前调用 `init`，在回调里注册路由表；
2. 在需要显示页面的位置放一个 `Outlet`；
3. 调用 `navigate` 切换路径。

```rust
init(cx, |router| {
    router.add_cache("/home", |_, cx| Home::new(cx));
    router.add_view("/settings", |_, cx| Settings::new(cx));
});

// 布局中
Outlet::new(cx)

// 切换
navigate(cx, "/settings");
```

`init` 必须在创建窗口之前调用：它通过 `cx.set_global` 写入路由表与缓存，窗口创建后再注册的路由不会被
已渲染的 `Outlet` 感知。

## 路由注册与缓存策略

`Router` 提供两种注册方式，区别只在页面生命周期：

| 方法 | 策略 | 行为 |
| --- | --- | --- |
| `add_cache(path, create)` | `Global` | 首次进入创建并写入全局缓存，之后复用同一份页面，**页面内部状态保留** |
| `add_view(path, create)` | `None` | 只在停留期间存在，离开路由后丢弃，**每次进入都是全新状态** |

两者都接受 `T: Render + 'static` 与 `F: Fn(&mut Window, &mut App) -> T + 'static`；工厂最终被擦除为
`Rc<dyn Fn(&mut Window, &mut App) -> AnyView>`，用 `Rc` 是因为工厂需要捕获页面的构造闭包。

选择原则：需要保留用户输入、滚动位置、已加载数据等状态的页面用 `add_cache`；每次进入都应重置的页面
（详情页、向导页等）用 `add_view`。

## 状态与缓存

三个全局状态（均实现 `Global`）：

- `RouteTable(Router)`：路由表，由 `init` 写入。
- `PageCache(HashMap<SharedString, AnyView>)`：全局页面缓存。`AnyView` 内部持有实体的强引用，
  因此页面一旦创建就一直存活，切走再切回来状态不丢。
- `ActiveView(Option<(SharedString, AnyView)>)`：当前非缓存页面，只保留正在显示的那一个。

对外 API：

| 函数 | 说明 |
| --- | --- |
| `current_path(cx) -> SharedString` | 当前路径 |
| `navigate(cx, path)` | 切换路径 |
| `Outlet::new(cx)` | 页面出口实体 |

`navigate` 的两条静默返回：路径未注册（避免渲染空白内容）、路径与当前相同（避免无意义的刷新）。
实际切换时会清空 `ActiveView` 并调用 `cx.refresh_windows()`，从而触发所有窗口重绘。

## Outlet

`Outlet` 是一个实体，内部持有 `cx.observe_global::<RoutePath>` 的 `Subscription`。该订阅必须保持存活，
否则路径变化时不会重新渲染；因此它是结构体字段而不是临时变量。

渲染时取当前路径对应的页面：命中则渲染（`size_full`），未命中（路径为空或未注册）渲染空 `div`。

`page_for_current_path` 按路由的缓存策略分流：

- 缓存路由：查 `PageCache`，命中直接复用，否则创建后写入；
- 非缓存路由：`ActiveView` 中的路径与当前一致则复用，否则重新创建并替换。

## 注意点

- 路径用字符串精确匹配，没有嵌套路由、参数解析与通配符；需要参数时由调用方在路径里编码。
- 缓存路由一旦创建就不会被回收，注册过多会造成页面长期驻留内存。
- 路由表在 `init` 之后不可变；动态增删路由需要先扩展这一层。
