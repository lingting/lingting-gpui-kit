# window_options_builder

窗口选项构建器。调用方按物理像素描述窗口尺寸，`WindowOptionsBuilder` 负责换算成 GPUI 需要的
逻辑像素，并统一自绘标题栏所需的窗口设置。

## 使用

```rust
use lingting_gpui_kit::WindowOptionsBuilder;
use gpui_kit::{DevicePixels, size};

let options = WindowOptionsBuilder::new()
    .title("我的应用")
    .size(size(DevicePixels(1600), DevicePixels(1024)))
    .size_min(size(DevicePixels(960), DevicePixels(600)))
    .build();
```

## API

| 方法 | 默认值 | 说明 |
| --- | --- | --- |
| `new()` | — | 默认尺寸 1600x1024、最小尺寸与初始尺寸相同、透明、无标题、按屏幕钳制 |
| `bounds(WindowBounds)` | 未设置 | 直接指定状态与位置；设置后覆盖按屏幕计算的位置，携带的尺寸与位置按逻辑像素处理 |
| `transparent(bool)` | `true` | 为 `true` 时背景取 `WindowBackgroundAppearance::Transparent`，`titlebar.appears_transparent` 随之为 `true`，macOS 上把红绿灯压进自绘标题栏 |
| `title(impl Into<SharedString>)` | 未设置 | 系统窗口标题，与自绘标题栏显示的标题无关 |
| `size(Size<DevicePixels>)` | 1600x1024 | 初始尺寸，物理像素 |
| `size_min(Size<DevicePixels>)` | 与 `size` 相同 | 最小尺寸，物理像素 |
| `traffic_light_position(Point<Pixels>)` | 标题栏内居中 | 仅 macOS：覆盖红绿灯位置（窗口左下角为原点，逻辑像素） |
| `clamp_to_screen(bool)` | `true` | 换算后尺寸超出屏幕时是否转为最大化创建 |
| `build()` | — | 构造 `WindowOptions` |

`build()` 是纯计算，不需要 `App`；屏幕信息由平台查询（Windows 下为主显示器 +
`GetDpiForMonitor(MDT_EFFECTIVE_DPI)`，其他平台返回 `None`）。

## 尺寸换算与屏幕适配

GPUI 的 `WindowBounds` 与 `window_min_size` 都是逻辑像素，而窗口实际占用物理像素，两者相差
屏幕缩放因子。缩放因子在窗口创建前只能由平台查询，因此 `build()` 内统一换算：

```
逻辑尺寸 = 物理尺寸 / 缩放因子
```

换算之后：

- 尺寸任意一边超出屏幕可见区域（或屏幕本身更小）且 `clamp_to_screen` 为 `true` 时，窗口以
  `WindowBounds::Maximized` 创建，还原尺寸取屏幕可见区域大小；
- 未超出时窗口按屏幕可见区域居中（`WindowBounds::Windowed`），尺寸仍按可见区域裁剪，避免
  覆盖任务栏；
- 最小尺寸未设置时取初始尺寸，并最终不超过初始值，避免最小值大于初始值导致窗口创建时被系统放大；
- 平台查询失败（非 Windows，或调用返回错误）时缩放因子按 `1.0` 处理，位置回退到原点，不做
  最大化转换。

## 标题栏与透明度

- `titlebar.appears_transparent` 跟随 `transparent`：透明窗口隐藏系统标题栏，交给自绘标题栏。
- `traffic_light_position` 只在 macOS 且 `transparent` 为 `true` 时给出：默认把红绿灯在
  `TITLE_BAR_HEIGHT` 内垂直居中（`x = 9`，`y = (52 - 12) / 2 = 20`），可用
  `traffic_light_position` 覆盖；不透明窗口与其余平台为 `None`，由窗口系统默认摆放。
- 红绿灯的直径与左侧留白是系统装饰基准层，用固定像素（`px()`）；高度、位置与它们一起算，
  因此 `TITLE_BAR_HEIGHT` 也是固定像素，这一层不随主题字号缩放。

## 注意点

- `size` / `size_min` 只接受 `Size<DevicePixels>`，传入逻辑像素会得到放大的窗口。
- 屏幕信息在 `build()` 调用时查询一次；窗口创建后显示器切换或缩放变化不会回溯修正。
- 只查询主显示器；需要指定显示器时用 `bounds` 自行传入逻辑像素矩形。
- Windows 下由 `windows-sys` 查询主显示器（`MonitorFromPoint` + `GetMonitorInfoW` 取工作区，
  `GetDpiForMonitor(MDT_EFFECTIVE_DPI)` 取缩放）；`visible_bounds` 为空区域时同样视作查询失败。
- 本 crate 同时再导出另外四个 crate 的全部内容（含 `lingting-gpui-core` 的 rem 档位），
  但自身只实现这一个模块。
