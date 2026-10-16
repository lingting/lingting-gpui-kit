/// 常用 rem 长度的快捷值。
///
/// 全库自绘 UI 的长度单位，GPUI 的 rem 以 `window.rem_size()` 为基准
/// （`gpui-component` 的 Root 把它设为 `theme.font_size`），与 `gap_2`、`px_3`、
/// `text_sm`、`rounded_lg` 等缩放助手同源；主题字号一变，字号、间距、控件尺寸、
/// 图标一起缩放。
///
/// 每档都提供常量与同名函数两种形式：常量适合结构体字段、`const` 声明与默认值，
/// 函数适合直接写在链式调用里，取值完全一致。
pub mod rem;

pub use rem::*;
