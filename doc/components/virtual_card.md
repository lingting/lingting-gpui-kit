# virtual_card

卡片包一层固定高度虚拟列表：`Card` + `FixedVirtualList`。

```text
┌──────────────────────────────┐
│ 卡片标题区                     │
├──────────────────────────────┤
│ 虚拟列表（固定行高）            │
└──────────────────────────────┘
```

它把 [`Card`](card.md) 的标题 / ghost 与 [`FixedVirtualList`](virtual_list.md) 的
数据 / 筛选 / 渲染器合成一个组件，省去调用方自己嵌套。

| 方法 | 默认值 | 说明 |
| --- | --- | --- |
| `new(impl Into<ElementId>)` | — | `id` 用于列表滚动状态的稳定标识 |
| `ghost(bool)` | `false` | 透传给 `Card` |
| `with_ghost()` | — | 等价于 `ghost(true)` |
| `title(impl IntoElement)` | 无 | 卡片标题 |
| `items(Rc<Vec<T>>)` | 空 | 数据；只持有引用，不克隆子项 |
| `item_height(Rems)` | `rems(4.)` | 子项高度 |
| `filter(ItemFilter<T>)` | 无 | 筛选条件 |
| `item_renderer(ItemRenderer<T>)` | 无 | 子项渲染器 |
| `empty(impl IntoElement)` | “暂无数据” | 空状态 |

`VirtualCard<T>` 是 `IntoElement`，也是 `Styled`；`Styled` 作用在内部的虚拟列表上，
因此内边距、尺寸等样式透传给列表而不是卡片。

## 注意点

- 卡片本身没有内容区，列表是它唯一的内容；`Card` 的 `content_padding` / `gap` 在 `VirtualCard`
  上没有对应的透传方法。
- `item_height`、`id` 的要求与 [`FixedVirtualList`](virtual_list.md) 相同：
  行高固定、`id` 稳定且唯一。
