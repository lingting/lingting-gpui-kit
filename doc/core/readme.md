# core

`lingting-gpui-core`：全库自绘 UI 的长度基准。它只有 `rem` 一个模块，不依赖其余 crate
（`lingting-gpui-components` / `lingting-gpui-layout` / `lingting-gpui-nav` / `lingting-gpui-kit`
都依赖它），由 `lingting-gpui-kit` 再导出。

## 为什么用 rem

GPUI 的 rem 以 `window.rem_size()` 为基准，`gpui-component` 的 Root 把它设为 `theme.font_size`，
与 `gap_2`、`px_3`、`text_sm`、`rounded_lg` 等缩放助手同源。主题字号一变，字号、间距、控件尺寸、
图标一起缩放。因此自绘 UI 的长度一律用 `rems()`，只有系统窗口装饰基准层与平台物理量用 `px()`。

## 提供的取值

每档都提供常量与同名函数两种形式，取值完全一致：常量适合结构体字段、`const` 声明与默认值，
函数适合直接写在链式调用里。

| 常量 | 函数 | 值 | 对应 |
| --- | --- | --- | --- |
| `REM_XS` | `rem_xs()` | 0.75rem | `text_xs` |
| `REM_SM` | `rem_sm()` | 0.875rem | `text_sm` |
| `REM_BASE` | `rem_base()` | 1rem | `text_base` |
| `REM_LG` | `rem_lg()` | 1.125rem | `text_lg` |
| `REM_XL` | `rem_xl()` | 1.25rem | `text_xl` |
| `REM_2XL` | `rem_2xl()` | 1.5rem | `text_2xl` |
| `REM_3XL` | `rem_3xl()` | 1.875rem | `text_3xl` |
| `REM_0P5` | `rem_0p5()` | 0.5rem | 小内边距与紧凑间隔 |
| `REM_1P5` | `rem_1p5()` | 1.5rem | 中等内边距 |
| `REM_2` | `rem_2()` | 2rem | 控件高度与段间距 |
| `REM_3` | `rem_3()` | 3rem | 大块间隔 |
| `REM_4` | `rem_4()` | 4rem | 列表项高度与页面级留白 |

```rust
use lingting_gpui_core::{REM_2, rem_sm};

const ITEM_HEIGHT: Rems = REM_2;
let icon_size = rem_sm();
```

## 注意点

- 只在「常用档位」内取值；需要别的数值时直接写 `rems(0.75)` 这类字面量，不要为此新增常量。
- 常量声明为 `Rems` 而不是 `Pixels`，用 `px()` 的地方不要引用这里的档位。
