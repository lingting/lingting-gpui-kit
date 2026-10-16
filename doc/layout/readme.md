# layout

`lingting-gpui-layout`：把 `lingting-gpui-components` 的组件与 `lingting-gpui-nav` 的路由拼成完整窗口。
长度单位统一用 `lingting-gpui-core` 的 rem 档位（[core/readme](../core/readme.md)），只有标题栏那一层
（`title_bar_height`、`desktop_top_layout` 的 `MENU_GAP`）用固定像素。

它提供两个开箱即用的桌面布局，两者都由本 crate 按路由表自动生成菜单，调用方只需注册路由、
提供标题栏图标 / 标题与侧栏顶 / 底段内容：

| 布局 | 结构 | 菜单位置 |
| --- | --- | --- |
| [`DesktopSidebarLayout`](desktop_sidebar_layout.md) | 标题栏 + 左侧栏 + 内容区 | 侧栏中间段 |
| [`DesktopTopLayout`](desktop_top_layout.md) | 标题栏 + 内容区 | 标题栏中间 |

两个布局共用 [`RouteDefinition`](router.md)（定义在 `crate::router`，不在布局模块再导出）与
[`Content`](content.md)，页面出口由 `lingting-gpui-nav` 的 `Outlet` 提供，布局只负责把它放进内容区。

## 槽位渲染器

`SlotRenderer` 是布局透传外部内容的统一形式：

```rust
pub type SlotRenderer = Rc<dyn Fn(&mut Window, &mut App) -> AnyElement>;
```

布局是每帧重渲染的实体视图，`AnyElement` 不可克隆，因此槽位存的是闭包、每帧求值一次。
需要保留状态的内容（例如自己的实体视图）应在闭包里返回已创建实体的克隆，而不是在闭包内重新创建。
