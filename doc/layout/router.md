# router

布局通用的路由定义与注册。两个桌面布局共用这一份逻辑，各自的 `init_router` 只是转发到这里，
便于后续按布局扩展而不改动调用方。

## RouteDefinition

一条路由 = 路径 + 菜单文案 + 页面工厂，外加可选的图标与显示 / 缓存策略。

```rust
RouteDefinition::new("/home", "首页", |_, cx| Home::new(cx))
    .icon(IconName::Home)
    .cache(true)
```

| 方法 | 默认值 | 说明 |
| --- | --- | --- |
| `new(path, label, create)` | — | `create` 与 `Router::add_cache` 接受的形式一致；`path` 既是跳转地址，也是菜单项的稳定标识 |
| `icon(IconName)` | 无图标 | 菜单项图标 |
| `hidden(bool)` | `false` | 隐藏菜单项；隐藏后仍可跳转进来，只是不出现在菜单里 |
| `cache(bool)` | `true` | 是否全局缓存页面：缓存则离开再进入保留内部状态，不缓存则每次进入都是全新状态 |

读取方法：`path()`、`label()`、`icon_name()`、`is_hidden()`、`is_cached()`、`factory()`。

菜单出现在哪个区域由布局决定：`desktop_sidebar_layout` 放进侧栏，`desktop_top_layout` 放进标题栏中间。

## init_router

```rust
pub fn init_router(cx: &mut App, default_path: impl Into<SharedString>, routes: &[RouteDefinition])
```

各布局从自己的模块再导出一个同名函数（`lingting_gpui_layout::desktop_sidebar_layout::init_router`
与 `lingting_gpui_layout::desktop_top_layout::init_router`），两者都转发到这里。

行为：

1. 遍历 `routes`，用 `Router::add_erased` 逐条注册（`RouteDefinition` 已经擦除过类型，
   因此异构页面可以放进同一个循环）；
2. 调用 `navigate` 进入 `default_path`。

必须在创建窗口之前调用：路由表通过 `lingting_gpui_nav::init` 写入全局状态，
之后再注册不会被已渲染的 `Outlet` 感知。

`default_path` 未出现在 `routes` 中时 `navigate` 静默忽略，此时内容区为空。
