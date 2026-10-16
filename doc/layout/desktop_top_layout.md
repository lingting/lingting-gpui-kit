# desktop_top_layout

桌面顶栏布局：标题栏在上，内容区在下。

```text
┌──────────────────────────────────────────────┐
│ 标题栏（中间是菜单）                           │
├──────────────────────────────────────────────┤
│ 内容区                                        │
└──────────────────────────────────────────────┘
```

标题栏中间的菜单由本布局按 [`RouteDefinition`](router.md) 生成，调用方不需要自己拼菜单；
标题栏其余参数按前缀透传给 `TitleBar`。

## 使用

```rust
use lingting_gpui_layout::desktop_top_layout::{init_router, DesktopTopLayout, RouteDefinition};

let routes = vec![
    RouteDefinition::new("/home", "首页", |_, cx| Home::new(cx)),
    RouteDefinition::new("/settings", "设置", |_, cx| Settings::new(cx)),
];

// 必须在创建窗口之前
init_router(cx, "/home", &routes);

let layout = cx.new(|cx| DesktopTopLayout::new(cx).routes(routes));
// 再把 layout 交给 open_window
```

布局持有页面出口实体，因此是实体视图：用 `cx.new` 创建后交给 `open_window`。

## 模块导出

- `DesktopTopLayout`
- `init_router(cx, default_path, routes)`：转发到布局通用的 `crate::router::init_router`

## API

链式方法按目标组件分组：标题栏的基础参数（`icon`、`title`）不带前缀，标题栏其余参数用
`title_bar_`，菜单参数用 `menu_`，内容区内边距用 `padding_`。

### 基础

| 方法 | 说明 |
| --- | --- |
| `new(cx)` | 标题栏无图标与标题，其余取各组件的默认值 |
| `routes(impl IntoIterator<Item = RouteDefinition>)` | 设置路由表；既用于生成标题栏菜单，也决定内容区能渲染哪些页面 |

`routes` 与 `init_router` 传入的是同一份路由：那里注册路由表，这里生成菜单。

### 标题栏

| 方法 | 说明 |
| --- | --- |
| `icon(impl Fn(&mut Window, &mut App) -> AnyElement + 'static)` | 左侧图标，尺寸由调用方决定 |
| `title(impl Into<SharedString>)` | 左侧标题 |
| `title_bar_height(impl Into<Pixels>)` | 覆盖标题栏高度 |
| `title_bar_windows(WindowsControls)` | 仅 Windows：窗口按钮的尺寸与显隐 |

标题栏中间区域固定由菜单占用，因此不提供 `title_bar_center`。

### 菜单

| 方法 | 说明 |
| --- | --- |
| `menu_height(impl Into<Pixels>)` | 覆盖菜单项高度 |
| `menu_icon_size(impl Into<Pixels>)` | 覆盖菜单项图标尺寸 |
| `menu_selected_style(MenuSelectedStyle)` | 设置菜单选中样式 |

菜单逐项参数都来自路由与当前路径，这里只提供对所有菜单项生效的公共参数。

### 内容区

`padding_top` / `padding_right` / `padding_bottom` / `padding_left`，各自接受 `impl Into<Pixels>`。
内容区只承载页面出口，因此没有内容槽位。

## 注意点

- 菜单只使用已注册的路由，未传入路由时中间区域为空，标题栏只剩两侧的拖动区。
- 菜单项之间用固定间距（4px）横向排列。
- 隐藏的路由不进入菜单，但仍可通过 `navigate` 跳转进去。
- 菜单选中态按当前路径判定，`navigate` 引起的重绘会让选中态跟着路由走。
