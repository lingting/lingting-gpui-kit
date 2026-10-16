# card

卡片容器与卡片列表项。

## Card

卡片容器：一个圆角表面，上面是可选的标题区，下面是内容区。

```text
┌──────────────────────────────┐
│ 标题区（p(title_padding)）     │
│            ↕ mb(gap)          │
│ 内容区（p(content_padding)）   │
└──────────────────────────────┘
```

尺寸默认 `auto`：不声明尺寸，由内容与父布局决定。另有 `min_h_0` 与 `overflow_hidden`：
高度被显式限制（`size_full` / `height_full` / `height` / `size`）时，卡片按声明的高度收缩，不被内容撑破。
圆角 `rounded_lg`；非 ghost 模式下绘制背景、一圈边框，主题开启阴影时再加 `shadow_sm`。

`Card` 是 `IntoElement`，同时也是 `ParentElement`：用 `Styled` 与 `child` / `children` 直接写内容。

| 方法 | 默认值 | 说明 |
| --- | --- | --- |
| `new()` | — | 无标题、无内容、非 ghost |
| `ghost(bool)` | `false` | 不绘制卡片表面（背景、边框与投影），标题区仍保留一块卡片底色 |
| `with_ghost()` | — | 等价于 `ghost(true)` |
| `title(impl IntoElement)` | 无 | 标题；标题区高度自适应，等于内边距加标题自身高度 |
| `title_padding(Rems)` | `rems(0.5)` | 标题区内边距 |
| `content_padding(Rems)` | `rems(0.)` | 内容区内边距，默认 0，由内容自己决定 |
| `gap(Rems)` | `rems(0.5)` | 标题区与内容区之间的间隔 |

### 尺寸

| 方法 | 默认值 | 说明 |
| --- | --- | --- |
| `size_full()` | — | 高度与宽度都铺满可用空间（100%） |
| `size_auto()` | — | 高度与宽度都不声明，由内容与父布局决定 |
| `size(Rems, Rems)` | — | 同时设置宽度与高度为固定长度，参数顺序是宽、高 |
| `height(Rems)` | `auto` | 固定高度 |
| `height_full()` | — | 高度铺满可用空间（100%） |
| `height_auto()` | — | 高度不声明，由内容与父布局决定 |
| `width(Rems)` | `auto` | 固定宽度 |
| `width_full()` | — | 宽度铺满可用空间（100%） |
| `width_auto()` | — | 宽度不声明，由内容与父布局决定 |

尺寸在用户样式之后应用，`size_full` / `size_auto` 与 `Styled` 上的同名方法取值一致，
无需再在 `Styled` 上重复声明；但 `Styled` 上的单边样式（如 `h_full()`）仍会被内部尺寸覆盖。

内容区只负责给出剩余高度（`flex_1`）：内容声明 `size_full` 或 `flex_1` 时铺满，
未声明时按自身高度排布，不会被拉伸。

`gap` 用标题区的下外边距实现，而不是留在标题块内部——ghost 模式下标题块有自己的底色，
间隔若算在块内就会被这块底色填满。

## ItemCard

卡片列表项：左 / 中 / 右三栏，中间栏占剩余宽度，可选选中态与行点击。

```text
┌────────────────────────────────────────────────┐
│ [左 left_width]  [中 center flex_1]  [右 right] │
└────────────────────────────────────────────────┘
```

| 方法 | 默认值 | 说明 |
| --- | --- | --- |
| `new(impl Into<ElementId>)` | — | `id` 决定元素身份与焦点归属，必须用稳定的业务标识，不能用列表下标 |
| `left(impl IntoElement)` | 无 | 左侧栏，宽度内容自适应 |
| `left_width(Rems)` | 内容自适应 | 覆盖左侧栏宽度 |
| `center(impl IntoElement)` | 无 | 中间栏，占左右两栏之外的剩余宽度 |
| `with_center_split(top, bottom)` | — | 把中间栏换成上下两块，各占行高的一半 |
| `right(impl IntoElement)` | 无 | 右侧栏，宽度内容自适应 |
| `right_width(Rems)` | 内容自适应 | 覆盖右侧栏宽度 |
| `selected(bool)` | `false` | 选中态 |
| `selected_style(ItemCardSelectedStyle)` | `Tinted` | 选中样式 |
| `hoverable(bool)` | `true` | 是否应用悬浮样式；纯展示的卡片关掉，避免被误读成可点击 |
| `on_click(impl Fn(&mut Window, &mut App) + 'static)` | 无 | 行点击处理 |

行本身可点击，因此同时也是焦点目标：`tab_stop`，Tab 能到达、Enter / Space 能激活，
焦点可见态由本组件绘制（`border_color(theme.ring)`）。

### 选中样式

| 取值 | 表现 |
| --- | --- |
| `Tinted`（默认） | 主题色边框 + 8% 主题色底 |
| `Elevated` | 主题背景色 + 一圈浅边 + 投影，视觉上从列表里浮起来 |
| `Accent` | `Elevated` 的底色、浅边与投影，文字换主题色并加粗 |
| `Filled` | 主题色背景 + 反白文字，选中项整块染色，悬停时用 `primary_hover` |

未选中时用 `background` 底 + `border` 边，悬停取 `secondary_hover`；`hoverable(false)` 时两种
悬停反馈都不加。

`Filled` 与 `Accent` 的 `text_color` 只作为缺省值：调用方在子元素上声明的颜色优先级更高。

`ItemCardSelectedStyle::icon_color(selected, theme)` 给出该样式下图标的取色：未选中取
`muted_foreground`，选中时 `Filled` 取反白、其余取主题色。SVG 不继承文本颜色，图标颜色必须
显式设在图标元素自身，调用方从该方法取值即可，不必自己判断选中态。

## 注意点

- 选中是持久状态，用元素自身的底色与边框表达，不用单侧边条。
- 两者的 `Styled` 都作用在内部容器上，且多数布局属性（尺寸、`rounded_*`、`border_1`、
  选中态的底色与边框等）在用户样式之后应用；用户样式主要用来覆盖内边距、间距，以及 ghost 下的底色。
  尺寸优先用 `Card` 自己的 `size_*` / `height_*` / `width_*` 方法声明。
- `ItemCard` 需要 `id` 稳定；同一个 `id` 复用焦点状态，列表里用下标会让焦点跟着滚动错位。
