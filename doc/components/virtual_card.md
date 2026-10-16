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
| `new(impl Into<ElementId>)` | — | `id` 用于列表滚动状态的稳定标识；卡片默认为 `card_size_full()` |
| `card_size_full()` | 默认 | 卡片高度与宽度都铺满可用空间（100%） |
| `card_size_auto()` | — | 卡片高度与宽度都不声明，由内容与父布局决定 |
| `card_size(Rems, Rems)` | — | 同时设置卡片的宽度与高度为固定长度，参数顺序是宽、高 |
| `card_height(Rems)` | `full` | 卡片固定高度 |
| `card_height_full()` | — | 卡片高度铺满可用空间（100%） |
| `card_height_auto()` | — | 卡片高度不声明，由内容与父布局决定 |
| `card_width(Rems)` | `full` | 卡片固定宽度 |
| `card_width_full()` | — | 卡片宽度铺满可用空间（100%） |
| `card_width_auto()` | — | 卡片宽度不声明，由内容与父布局决定 |
| `ghost(bool)` | `false` | 透传给 `Card` |
| `with_ghost()` | — | 等价于 `ghost(true)` |
| `title(impl IntoElement)` | 无 | 卡片标题 |
| `items(Rc<Vec<T>>)` | 空 | 数据；只持有引用，不克隆子项 |
| `item_height(Rems)` | `rems(4.)` | 子项高度 |
| `filter(ItemFilter<T>)` | 无 | 筛选条件 |
| `item_renderer(ItemRenderer<T>)` | 无 | 子项渲染器 |
| `empty(impl IntoElement)` | “暂无数据” | 空状态 |

`VirtualCard<T>` 是 `IntoElement`，也是 `Styled`；`Styled` 作用在内部的虚拟列表上，
因此内边距等样式透传给列表而不是卡片。卡片尺寸用 `card_*` 系列方法设置，
这样不会与 `Styled` 作用在列表上的尺寸方法（`size_full`、`h_full` 等）混淆。

## 注意点

- 卡片本身没有内容区，列表是它唯一的内容；`Card` 的 `content_padding` / `gap` 在 `VirtualCard`
  上没有对应的透传方法。
- 虚拟列表需要确定的高度才能算出可视区，因此卡片默认 `card_size_full()`；
  改成 `card_size_auto()` 或 `card_height_auto()` 后列表会塌成 0 高。
- `item_height`、`id` 的要求与 [`FixedVirtualList`](virtual_list.md) 相同：
  行高固定、`id` 稳定且唯一。
