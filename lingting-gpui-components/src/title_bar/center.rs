use gpui_kit::*;

/// 中间区域：`flex_1` 占满左侧与右侧之间的全部空间，传入内容时内容整体居中。
///
/// 区域本身不遮挡命中测试，内容之外的空白仍由标题栏的基座拖动层接管，可以拖动窗口。
pub(crate) fn center(center: Option<AnyElement>) -> AnyElement {
    div()
        .flex_1()
        .min_w_0()
        .flex()
        .items_center()
        .justify_center()
        .children(center.map(content))
        .into_any_element()
}

/// 内容外层，按内容尺寸撑开。
///
/// `occlude` 在内容处截断命中测试，使基座拖动层的命中盒不进 `ids`，该区域回落到
/// `HTCLIENT`，内容自己的命中盒才能收到鼠标事件。遮挡层不能加在中间区域容器上：
/// 那会连内容之外的空白一起挡住，整片区域都无法拖动窗口。
fn content(center: AnyElement) -> AnyElement {
    div()
        .occlude()
        .flex()
        .items_center()
        .child(center)
        .into_any_element()
}
