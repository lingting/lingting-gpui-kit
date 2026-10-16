# menu

菜单项：点击即跳转路由的一行。

```
┌──────────────────┐
│ [图标] 文案       │
└──────────────────┘
```

由 `Menu::new` 建立默认值：不显示图标、高度 2rem、图标 0.875rem、字号 0.875rem、未选中、
`Elevated` 选中样式。

| 方法 | 默认值 | 说明 |
| --- | --- | --- |
| `new(path, label)` | — | `path` 既是点击后跳转的路由，也是元素 id |
| `icon(IconName)` | 无图标 | 指定图标，未指定时不渲染 |
| `icon_size(impl Into<AbsoluteLength>)` | `ICON_SIZE`（0.875rem） | 覆盖图标尺寸 |
| `height(impl Into<AbsoluteLength>)` | `ITEM_HEIGHT`（2rem） | 覆盖菜单项高度 |
| `text_size(impl Into<AbsoluteLength>)` | `TEXT_SIZE`（0.875rem） | 覆盖菜单项字号 |
| `selected(bool)` | `false` | 选中状态，通常由调用方与当前路由比较后传入 |
| `selected_style(MenuSelectedStyle)` | `Elevated` | 选中样式 |

样式：`px_4` + `gap_2` + `rounded_md` + 上面三项长度（高度、图标尺寸、字号）+ `cursor_pointer`，
未选中时悬停取 `secondary_hover`。

## 长度单位

三个长度参数都收 `AbsoluteLength`，rem 与 px 都能传，默认值是 rem。同一组件因此覆盖两种场景：

| 场景 | 取值 |
| --- | --- |
| 侧栏等自绘容器 | 默认值：`ITEM_HEIGHT`（2rem）、`ICON_SIZE`（0.875rem）、`TEXT_SIZE`（0.875rem），随主题字号缩放 |
| 标题栏 | `constants::TITLE_BAR_ITEM_HEIGHT`（32px）、`TITLE_BAR_ICON_SIZE`（14px）、`TITLE_BAR_TEXT_SIZE`（14px） |

标题栏与系统摆放的窗口控制按钮、红绿灯对齐，必须用固定像素；`desktop_top_layout` 已经按这三个
常量给出默认值，调用方显式传 `menu_height` / `menu_icon_size` / `menu_text_size` 时以调用方为准。

## 选中样式

| 取值 | 表现 |
| --- | --- |
| `Elevated`（默认） | 主题背景色 + 一圈浅边 + 投影，视觉上从标题栏 / 侧栏里浮起来 |
| `Filled` | 主题色背景 + 反白文字与图标，悬停时用 `primary_hover` |

## 点击行为

菜单项自己调用 `lingting_gpui_nav::navigate`，因此：

- 路径必须是已注册的路由，未注册的路径会被静默忽略；
- 点击当前已选中的项不会产生切换（`navigate` 会跳过同路径）；
- 选中态不由本组件维护，需要在渲染时按当前路径传入 `selected`。

`Menu` 依赖 `lingting-gpui-nav`，是 `lingting-gpui-components` 里唯一带导航语义的组件；
布局层（`lingting-gpui-layout`）按路由表批量生成菜单并负责选中态判定。

## 注意点

- 图标颜色必须显式设在 `svg()` 自身：SVG 取的是元素自己的文本颜色，既不继承父元素，
  缺省时也不会绘制。
