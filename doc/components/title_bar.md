# title_bar

自绘标题栏。用于隐藏系统标题栏后，在窗口顶部提供拖动区域、中间内容与窗口控制按钮。

## 布局

```
┌──────────────────────────────────────────────────────────┐
│ [图标 标题]  ──拖动──  [中间内容]  ──拖动──  [_ □ ×]      │
└──────────────────────────────────────────────────────────┘
     左侧                中间（flex_1）              右侧
```

- 整行高度默认 `TITLE_BAR_HEIGHT`（52px），`flex_shrink_0`，背景与下边框取主题的
  `title_bar` / `title_bar_border`。
- 左侧只渲染已传入的部分；图标与标题都未传入时整个左侧区域不渲染。
- 中间是 `flex_1` 容器，内部依次是左侧拖动占位、`center` 传入的内容、右侧拖动占位；两侧占位都是
  `flex_1`，因此传入内容默认居中，传入多个元素时由调用方自行排列。
- 右侧为 Windows 窗口按钮，非 Windows 平台不编译该模块，也始终不渲染。

## API

`TitleBar` 是 `IntoElement`，链式构造，`new()` 提供默认值。

| 方法 | 说明 |
| --- | --- |
| `new()` | 默认高度、无图标 / 标题 / 中间内容、Windows 下渲染默认尺寸的窗口按钮 |
| `height(impl Into<Pixels>)` | 覆盖高度 |
| `icon(impl IntoElement)` | 左侧图标，尺寸由调用方决定 |
| `title(impl Into<SharedString>)` | 左侧标题，`text_sm` + `font_semibold` |
| `center(impl IntoElement)` | 中间区域内容 |
| `windows(WindowsControls)` | Windows 下窗口按钮的尺寸与显隐 |

```rust
TitleBar::new()
    .title("我的应用")
    .center(menu)
```

## 窗口控制按钮

`WindowsControls`（仅 `target_os = "windows"`）配置右侧最小化 / 最大化 / 关闭三个按钮：

| 方法 | 默认值 | 说明 |
| --- | --- | --- |
| `width(impl Into<Pixels>)` | `px(46.)` | 单个按钮宽度 |
| `icon_size(impl Into<Pixels>)` | `px(14.)` | 图标尺寸 |
| `visible(bool)` | `true` | 是否渲染这三个按钮 |

按钮通过 `WindowControlArea`（`Min` / `Max` / `Close`）交给系统处理点击，因此不需要自己绑定点击事件；
最大化按钮的图标按 `Window::is_maximized()` 在 `WindowMaximize` 与 `WindowRestore` 之间切换。
悬停背景取主题 `secondary_hover`，关闭按钮取 `danger`。

## 拖动区域的约束

`WindowControlArea` 的命中判定按插入顺序取第一个，父元素上的 `Drag` 会盖住子元素。因此：

- `Drag` 只加在左侧区域，以及中间内容左右两侧的空白占位（`drag_spacer`）上；
- 中间内容与窗口按钮位于这些拖动区之间或之后，不会被覆盖。

新增中间区域内容时，注意不要把它放进带 `Drag` 的容器里，否则该内容无法接收鼠标事件。

## 其他注意点

- SVG 图标必须把颜色显式设在 `svg()` 自身（`text_color`）：SVG 取的是元素自己的文本颜色，既不继承父元素，
  缺省时也不会绘制。
- 标题栏依赖宿主窗口已设置为无系统装饰，本模块不负责窗口装饰的切换。
