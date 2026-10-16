# virtual_list

固定行高的虚拟列表。

每个子项占据的高度由 `item_height` 指定，**包含子项自身的内外边距**；子项渲染结果必须在该高度内完成
布局，超出部分会被裁掉（每行内部 `overflow_hidden`，并留 0.5rem（`pb_2`）下内边距作为行间距）。

## 类型别名

| 别名 | 定义 |
| --- | --- |
| `ItemFilter<T>` | `Rc<dyn Fn(&T) -> bool>`，返回 `true` 表示保留 |
| `ItemRenderer<T>` | `Rc<dyn Fn(&T, &mut Window, &mut App) -> AnyElement>` |

```rust
FixedVirtualList::new("device-list")
    .items(items)              // Rc<Vec<Device>>
    .item_height(rems(3.5))
    .item_renderer(Rc::new(|device, _, _| ItemCard::new(device.id).into_any_element()))
    .empty("暂无设备")
```

## API

`FixedVirtualList<T>` 是 `IntoElement`，也是 `Styled`。

| 方法 | 默认值 | 说明 |
| --- | --- | --- |
| `new(impl Into<ElementId>)` | — | `id` 同时用于滚动状态的稳定标识，同一页面内的多个列表必须不同 |
| `items(Rc<Vec<T>>)` | 空 | 设置数据；列表只持有引用，不会克隆每一条子项 |
| `item_height(Rems)` | `rems(4.)` | 设置子项高度 |
| `filter(ItemFilter<T>)` | 无 | 筛选条件，未命中任何子项时显示空状态 |
| `item_renderer(ItemRenderer<T>)` | 无 | 子项渲染器；未设置时不渲染列表 |
| `empty(impl IntoElement)` | “暂无数据” | 空状态 |

## 行为

- 筛选只记录原始下标（`Rc<Vec<usize>>`），渲染时按下标取数据，因此不需要克隆数据，
  也不需要把筛选结果收集成新的元素列表；
- 滚动位置必须跨帧保留，句柄存在以列表 `id` 为键的元素状态里（`UniformListScrollHandle`）；
- 数量为 0 时渲染空状态，居中、`text_sm`、颜色取 `muted_foreground`；
- 数量大于 0 时渲染 `uniform_list`，并在其上叠一个纵向 `Scrollbar`。滚动条浮在内容之上、贴列表右边缘：
  它不占布局宽度，子项仍铺满列表宽度，因此只会盖住内容最右侧的一小条。

## 注意点

- `id` 必须稳定且唯一：它同时决定滚动状态与元素身份。
- `item_height` 是固定值，变高内容会被裁掉；需要变高时不能用这个列表。
- `item_renderer` 未设置时列表区域为空（没有 `uniform_list` 也没有滚动条），但仍会按数量判断是否显示空状态。
