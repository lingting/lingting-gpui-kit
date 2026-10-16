use gpui_kit::{Pixels, Rems, px};
use lingting_gpui_core::{REM_2, REM_SM};

/// 菜单项高度默认值。
///
/// 默认值与下面三项一样是 rem：GPUI 的 rem 跟随主题字号缩放，适合侧栏这类自绘容器。
pub(crate) const ITEM_HEIGHT: Rems = REM_2;

/// 图标尺寸默认值。
pub(crate) const ICON_SIZE: Rems = REM_SM;

/// 菜单项字号默认值。
pub(crate) const TEXT_SIZE: Rems = REM_SM;

/// 放进标题栏时的菜单项高度。
///
/// 菜单摆在标题栏这一层，而标题栏与系统窗口装饰对齐、本身用固定像素
/// （[`TITLE_BAR_HEIGHT`](crate::title_bar::TITLE_BAR_HEIGHT)）；这里跟随主题字号缩放会和
/// 系统摆放的窗口按钮错开，因此同样用固定像素。
pub const TITLE_BAR_ITEM_HEIGHT: Pixels = px(32.);

/// 放进标题栏时的图标尺寸，理由同 [`TITLE_BAR_ITEM_HEIGHT`]。
pub const TITLE_BAR_ICON_SIZE: Pixels = px(14.);

/// 放进标题栏时的菜单项字号，理由同 [`TITLE_BAR_ITEM_HEIGHT`]。
pub const TITLE_BAR_TEXT_SIZE: Pixels = px(14.);
