# sidebar

顶 / 中 / 底三段式侧栏。

```text
┌──────────────────┐
│ top（max_h 上限） │
├──────────────────┤
│ middle（flex_1）  │
├──────────────────┤
│ bottom（max_h）   │
└──────────────────┘
```

默认宽度 220px、内边距 8px、段间距 4px，`flex_shrink_0`；背景取主题 `muted`，
右侧一圈 `sidebar_border` 边框。

| 方法 | 默认值 | 说明 |
| --- | --- | --- |
| `new()` | — | 三段皆空 |
| `width(impl Into<Pixels>)` | `px(220.)` | 覆盖侧栏宽度 |
| `padding(impl Into<Pixels>)` | `px(8.)` | 覆盖内边距 |
| `top_max_height(impl Into<Pixels>)` | `TOP_MAX_HEIGHT`（160px） | 覆盖顶部段最大高度 |
| `top_max_height_auto()` | — | 顶部段高度自适应：不设上限，由内容决定 |
| `bottom_max_height(impl Into<Pixels>)` | `BOTTOM_MAX_HEIGHT`（160px） | 覆盖底部段最大高度 |
| `bottom_max_height_auto()` | — | 底部段高度自适应：不设上限，由内容决定 |
| `top(impl IntoIterator<Item = impl IntoElement>)` | 空 | 追加顶部段内容 |
| `middle(impl IntoIterator<Item = impl IntoElement>)` | 空 | 追加中间段内容，占满剩余高度 |
| `bottom(impl IntoIterator<Item = impl IntoElement>)` | 空 | 追加底部段内容 |

三段都是追加语义（可以多次调用累加），中间段是导航列表的位置。

## 段的布局

每段都是一个 `Section`：纵向排列、子项之间留 4px 基准间距、子项撑满侧栏的内容宽度、
两个方向都裁剪超出内容。段被限制了高度上限时，超出的子项不会溢出到相邻段上。

- 顶部与底部段只设最大高度（`max_h`）：内容更矮时按内容高度，更高时被截断并裁剪；
- 中间段是 `flex_1` + `min_h_0`，吃掉剩余高度。

`padding` 是段与侧栏边缘之间的距离，段与段之间的距离（4px）不随它变化。

## 注意点

- 空段不渲染：三段都空时侧栏只剩一块背景，段间距也不会在边缘留下空白。
- 只有最大高度的段不会滚动，超出部分直接裁剪；需要滚动时在段内自己放滚动容器。
- 段与段的间距固定在组件内部，不可配置。
