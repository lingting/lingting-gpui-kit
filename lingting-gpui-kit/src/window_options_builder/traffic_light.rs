use gpui_kit::{Pixels, Point, point, px};

use crate::title_bar::TITLE_BAR_HEIGHT;

/// 红绿灯按钮直径。
///
/// 红绿灯由系统按固定像素摆放，位置要和 [`TITLE_BAR_HEIGHT`]（同为固定像素）一起算，因此这里用 `px()`。
const BUTTON_SIZE: Pixels = px(12.);

/// 红绿灯左侧留白。
///
/// 与 [`BUTTON_SIZE`] 同属系统装饰基准层，用 `px()`。
const LEFT_INSET: Pixels = px(9.);

/// 默认位置：红绿灯在自绘标题栏（高度 [`TITLE_BAR_HEIGHT`]）内垂直居中。
pub(crate) fn default_position() -> Point<Pixels> {
    point(LEFT_INSET, (TITLE_BAR_HEIGHT - BUTTON_SIZE) * 0.5)
}
