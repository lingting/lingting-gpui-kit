# menu

菜单项：点击即跳转路由的一行。

```
┌──────────────────┐
│ [图标] 文案       │
└──────────────────┘
```

由 `Menu::new` 建立默认值：不显示图标、高度 32px、未选中、`Elevated` 选中样式。

| 方法 | 默认值 | 说明 |
| --- | --- | --- |
| `new(path, label)` | — | `path` 既是点击后跳转的路由，也是元素 id |
| `icon(IconName)` | 无图标 | 指定图标，未指定时不渲染 |
| `icon_size(impl Into<Pixels>)` | `px(14.)` | 覆盖图标尺寸 |
| `height(impl Into<Pixels>)` | `px(32.)` | 覆盖菜单项高度 |
| `selected(bool)` | `false` | 选中状态，通常由调用方与当前路由比较后传入 |
| `selected_style(MenuSelectedStyle)` | `Elevated` | 选中样式 |

样式：`px_4` + `gap_2` + `rounded_md` + `text_sm` + `cursor_pointer`，未选中时悬停取
`secondary_hover`。

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
