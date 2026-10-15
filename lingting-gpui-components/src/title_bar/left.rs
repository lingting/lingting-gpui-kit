use gpui_kit::base::StyledExt as _;
use gpui_kit::*;

/// 左侧区域。整体作为窗口拖动区。
///
/// 图标与标题均由调用方传入，只渲染已传入的部分；两者都未传入时不渲染该区域。
pub(crate) fn left(icon: Option<AnyElement>, title: Option<SharedString>) -> Option<AnyElement> {
    if icon.is_none() && title.is_none() {
        return None;
    }

    Some(
        div()
            .id("title-bar-left")
            .flex()
            .flex_row()
            .items_center()
            .gap_2()
            .pl_3()
            .h_full()
            .flex_shrink_0()
            .window_control_area(WindowControlArea::Drag)
            .children(icon)
            .children(title.map(|title| div().text_sm().font_semibold().child(title)))
            .into_any_element(),
    )
}
