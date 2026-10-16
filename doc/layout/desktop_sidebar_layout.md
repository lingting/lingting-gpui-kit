# desktop_sidebar_layout

桌面侧栏布局：标题栏在上，左侧栏与内容区并排在下。

```text
┌──────────────────────────────────────────────┐
│ 标题栏                                        │
├───────────────┬──────────────────────────────┤
│ 侧栏           │ 内容区                        │
│  top           │                              │
│  middle（菜单） │                              │
│  bottom        │                              │
└───────────────┴──────────────────────────────┘
```

侧栏中间段的菜单由本布局按 [`RouteDefinition`](router.md) 生成，调用方不需要自己拼菜单；
其余部分（标题栏、侧栏顶 / 底段、内容区）的参数按前缀透传给对应组件。

## 使用

```rust
use lingting_gpui_layout::desktop_sidebar_layout::{init_router, DesktopSidebarLayout};
use lingting_gpui_layout::router::RouteDefinition;

let routes = vec![
    RouteDefinition::new("/home", "首页", |_, cx| Home::new(cx)),
    RouteDefinition::new("/settings", "设置", |_, cx| Settings::new(cx)),
];

// 必须在创建窗口之前
init_router(cx, "/home", &routes);

let layout = cx.new(|cx| DesktopSidebarLayout::new(cx).routes(routes));
// 再把 layout 交给 open_window
```

布局持有页面出口实体，因此是实体视图：用 `cx.new` 创建后交给 `open_window`。

## 模块导出

- `DesktopSidebarLayout`
- `init_router(cx, default_path, routes)`：转发到布局通用的 `crate::router::init_router`

`RouteDefinition` 不在本模块再导出，从 [`crate::router`](router.md) 引入。

## API

链式方法按目标组件分组：标题栏的基础参数（`icon`、`title`）不带前缀，标题栏其余参数用
`title_bar_`，侧栏参数用 `sidebar_`，菜单参数用 `menu_`，内容区内边距用 `padding_`。

### 基础

| 方法 | 说明 |
| --- | --- |
| `new(cx)` | 标题栏无图标 / 标题 / 中间内容，侧栏顶 / 底段为空，其余取各组件的默认值 |
| `routes(impl IntoIterator<Item = RouteDefinition>)` | 设置路由表；既用于生成侧栏菜单，也决定内容区能渲染哪些页面 |

`routes` 与 `init_router` 传入的是同一份路由：那里注册路由表，这里生成菜单。

### 标题栏

| 方法 | 说明 |
| --- | --- |
| `icon(impl Fn(&mut Window, &mut App) -> AnyElement + 'static)` | 左侧图标，尺寸由调用方决定 |
| `title(impl Into<SharedString>)` | 左侧标题 |
| `title_bar_height(impl Into<Pixels>)` | 覆盖标题栏高度（标题栏属于系统装饰基准层，用固定像素） |
| `title_bar_center(impl Fn(&mut Window, &mut App) -> AnyElement + 'static)` | 标题栏中间区域内容 |
| `title_bar_windows(WindowsControls)` | 仅 Windows：窗口按钮的尺寸与显隐 |

### 侧栏

| 方法 | 说明 |
| --- | --- |
| `sidebar_width(impl Into<Rems>)` | 覆盖侧栏宽度 |
| `sidebar_padding(impl Into<Rems>)` | 覆盖侧栏内边距 |
| `sidebar_top(impl Fn(&mut Window, &mut App) -> AnyElement + 'static)` | 顶部段内容 |
| `sidebar_top_max_height(impl Into<Rems>)` | 覆盖顶部段最大高度 |
| `sidebar_top_max_height_auto()` | 顶部段高度自适应：不设上限，由内容决定 |
| `sidebar_bottom(...)` | 底部段内容 |
| `sidebar_bottom_max_height(impl Into<Rems>)` | 覆盖底部段最大高度 |
| `sidebar_bottom_max_height_auto()` | 底部段高度自适应：不设上限，由内容决定 |

中间段固定由本布局的菜单填充，因此没有 `sidebar_middle`。

### 菜单

| 方法 | 说明 |
| --- | --- |
| `menu_height(impl Into<AbsoluteLength>)` | 覆盖菜单项高度 |
| `menu_icon_size(impl Into<AbsoluteLength>)` | 覆盖菜单项图标尺寸 |
| `menu_text_size(impl Into<AbsoluteLength>)` | 覆盖菜单项字号 |
| `menu_selected_style(MenuSelectedStyle)` | 设置菜单选中样式 |

菜单逐项参数（路径、文案、图标、选中态）都来自路由与当前路径，这里只提供对所有菜单项生效的公共参数。

侧栏菜单用 [`Menu`](../components/menu.md) 的默认值（rem），跟随主题字号缩放；需要固定像素时
显式传值，三个参数都收 `AbsoluteLength`。

### 内容区

| 方法 | 说明 |
| --- | --- |
| `padding(impl Into<Rems>)` | 覆盖四个方向的内边距 |
| `padding_top` / `padding_right` / `padding_bottom` / `padding_left`（`impl Into<Rems>`） | 覆盖单边内边距 |
| `content_background(impl Into<Hsla>)` | 覆盖内容区背景色，默认为主题的 `muted` |

内容区只承载页面出口，因此没有内容槽位。长度一律用 rem（`Rems`）。

## 注意点

- 侧栏槽位是每帧求值的闭包（见 [layout/readme](readme.md)），需要保留状态时在闭包里返回已有实体的克隆。
- 隐藏的路由不进入菜单，但仍可通过 `navigate` 跳转进去。
- 菜单选中态按当前路径判定，`navigate` 引起的重绘会让选中态跟着路由走。
