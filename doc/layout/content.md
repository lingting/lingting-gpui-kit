# content

内容区域，布局中承载页面出口的那一块。

## 行为

- 横向吃掉剩余宽度、纵向吃掉剩余高度（`flex_1` + `min_w_0` + `min_h_0`）；
- 四个方向都有内边距：上下 `DEFAULT_PADDING_VERTICAL`（0.75rem）、左右 `DEFAULT_PADDING_HORIZONTAL`（1rem）；
- 背景默认取主题的 `muted`，与侧栏同色系，页面卡片会在这层底色上浮起；需要跟布局主背景一致时覆盖它；
- 自己不提供滚动：页面出口是固定尺寸的，超出部分由出口裁剪，
  页面需要滚动时在自己的范围内放滚动容器。

两个桌面布局都用它包住 `Outlet`，并把内容区内边距透传成布局上的 `padding` /
`padding_top` / `padding_right` / `padding_bottom` / `padding_left`（不带前缀），背景用
`content_background` 覆盖。

## API

`Content` 是 `IntoElement`。

| 方法 | 说明 |
| --- | --- |
| `new()` | 上下内边距 0.75rem、左右 1rem，背景取主题 `muted` |
| `background(impl Into<Hsla>)` | 覆盖背景色 |
| `padding(impl Into<Rems>)` | 覆盖四个方向的内边距 |
| `padding_top(impl Into<Rems>)` | 覆盖上内边距 |
| `padding_right(impl Into<Rems>)` | 覆盖右内边距 |
| `padding_bottom(impl Into<Rems>)` | 覆盖下内边距 |
| `padding_left(impl Into<Rems>)` | 覆盖左内边距 |
| `children(impl IntoIterator<Item = impl IntoElement>)` | 追加内容 |

## 注意点

- 长度一律用 rem（`Rems`），随主题字号缩放；`DEFAULT_PADDING_VERTICAL` / `DEFAULT_PADDING_HORIZONTAL`
  是 `pub(crate)`，外部只能通过布局的 `padding_*` 或自己给 `Content` 传值。
- 内容区只负责给出剩余空间，内容未声明 `size_full` / `flex_1` 时按自身高度排布。
