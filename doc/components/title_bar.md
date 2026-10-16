# title_bar

自绘标题栏。用于隐藏系统标题栏后，在窗口顶部提供拖动区域、中间内容与窗口控制按钮。

## 布局

```text
┌──────────────────────────────────────────────────────────┐
│ 基座拖动层（absolute + inset_0 + Drag，铺满整条）           │
│ ┌──────────────────────────────────────────────────────┐ │
│ │ [图标 标题]   [中间内容]   [_ □ ×]                     │ │
│ └──────────────────────────────────────────────────────┘ │
└──────────────────────────────────────────────────────────┘
   内容层：flex_row + gap，背景与下边框画在这一层
     左侧          中间（flex_1）            右侧
```

- 整行高度默认 `TITLE_BAR_HEIGHT`（52px），`flex_shrink_0`。
- 分两层：底层是**基座拖动层**（`absolute` + `inset_0` + `WindowControlArea::Drag`），铺满整条；
  上层是**内容层**（`relative`，与基座同尺寸并重叠，`flex_row` + `gap`），背景 `title_bar`
  与下边框 `title_bar_border` 画在内容层。
- 内容层的左侧、中间、右侧三个区域之间用 `gap` 隔开，默认 `TITLE_BAR_GAP`（4px）。
- 左侧只渲染已传入的部分；图标与标题都未传入时整个左侧区域不渲染。左侧自身不带拖动区：
  它没有命中盒，鼠标落在图标与标题上时命中的是基座拖动层。
- 中间是 `flex_1` 容器（`min_w_0`），传入内容时内容整体居中，传入多个元素时由调用方自行排列；
  内容外层带 `occlude`，在内容处截断命中测试，使内容自己的命中盒能收到鼠标事件，
  内容之外的空白仍由基座拖动层接管。
- 右侧为 Windows 窗口按钮，非 Windows 平台不编译该模块，也始终不渲染；按钮组同样带 `occlude`，
  并用 `items_stretch` 撑满标题栏高度。

## API

`TitleBar` 是 `IntoElement`，链式构造，`new()` 提供默认值。

| 方法 | 说明 |
| --- | --- |
| `new()` | 默认高度与间距、无图标 / 标题 / 中间内容、Windows 下渲染默认尺寸的窗口按钮 |
| `gap(impl Into<Pixels>)` | 覆盖内容层三个区域之间的间距，默认 `TITLE_BAR_GAP`（4px） |
| `height(impl Into<Pixels>)` | 覆盖高度，默认 `TITLE_BAR_HEIGHT`（52px） |
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
高度、间距、按钮宽度与图标尺寸都属于「系统窗口装饰基准层」，用固定像素（`px()` / `Pixels`）而不是
rem：它们要和系统摆放的窗口控制按钮对齐，跟随主题字号缩放会错位。

## 拖动区域的约束

`WindowControlArea` 的命中判定按插入顺序取第一个，父元素上的 `Drag` 会盖住子元素。因此：

- `Drag` 只加在铺满整条的基座层上，它排在内容层之前（在下方）；
- 需要接收鼠标事件的内容必须在自身加 `occlude` 截断命中测试，基座层的命中盒才不会先命中它：
  中间内容的外层与窗口按钮组都已经加好；
- `occlude` 不能加在中间区域容器上：那会连内容之外的空白一起挡住，整片区域都无法拖动窗口。

新增中间区域内容时，内容外层会自动带上 `occlude`；若内容内部还有需要点击的区域，注意不要再
套一层覆盖整块中间区域的容器，否则该区域会失去拖动能力。

## 其他注意点

- SVG 图标必须把颜色显式设在 `svg()` 自身（`text_color`）：SVG 取的是元素自己的文本颜色，既不继承父元素，
  缺省时也不会绘制。
- 标题栏依赖宿主窗口已设置为无系统装饰，本模块不负责窗口装饰的切换。
