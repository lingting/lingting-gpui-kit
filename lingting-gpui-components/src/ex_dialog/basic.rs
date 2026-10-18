//! `ExDialog` 的结构体定义、与 `Dialog` 的互相转换以及元素实现。

use gpui_kit::component::dialog::Dialog;
use gpui_kit::*;

/// 上下内边距：0.5rem（对应 `py_2`）。
pub(super) const PADDING_Y: Rems = rems(0.5);

/// 左右内边距：0.75rem（对应 `px_3`）。
pub(super) const PADDING_X: Rems = rems(0.75);

/// 一个已带统一内边距的空 `div`，用于包装用户传入的元素。
pub(super) fn padded() -> Div {
    div().py(PADDING_Y).px(PADDING_X)
}

/// 对话框。
///
/// API 与 [`Dialog`](gpui_kit::component::dialog::Dialog) 完全一致，差别只有两处：
///
/// 1. 用户传入的内容与底部会各自被补上内边距（上下 `0.5rem`、左右 `0.75rem`），避免内容完全贴到弹窗边缘，
///    导致投影、焦点环等需要溢出空间的样式被裁掉。未传入内容或底部时不加包装，行为与 `Dialog` 一致。
/// 2. `child` / `children` 追加的元素同样补上内边距：`Dialog` 把它们放进
///    `overflow_hidden` 的滚动容器，输入框这类在边框外绘制焦点环的控件会被容器边缘裁掉。
///
/// 需要交回 `window.open_dialog` 这类只接受 `Dialog` 的命令式 API 时，用 [`Self::into_dialog`]
/// 或 `From<Dialog>` 在两个类型之间转换：
///
/// ```ignore
/// window.open_dialog(cx, |dialog, window, cx| {
///     ExDialog::from(dialog).title("标题").footer(buttons).into_dialog()
/// });
/// ```
#[derive(IntoElement)]
pub struct ExDialog {
    pub(super) dialog: Dialog,
    /// `child` / `children` 追加的内容，渲染时统一补内边距。
    pub(super) children: Vec<AnyElement>,
}

impl ExDialog {
    /// 创建对话框。
    pub fn new(cx: &mut App) -> Self {
        Self {
            dialog: Dialog::new(cx),
            children: Vec::new(),
        }
    }

    /// 转成内层的 [`Dialog`](gpui_kit::component::dialog::Dialog)。
    ///
    /// `child` / `children` 追加的内容会补上内边距后并入，与直接渲染 `ExDialog` 一致。
    pub fn into_dialog(self) -> Dialog {
        let Self { dialog, children } = self;

        if children.is_empty() {
            return dialog;
        }

        dialog.child(padded().children(children))
    }
}

impl From<Dialog> for ExDialog {
    fn from(dialog: Dialog) -> Self {
        Self {
            dialog,
            children: Vec::new(),
        }
    }
}

impl ParentElement for ExDialog {
    /// 追加的元素补上内边距后交给 `Dialog`，让焦点环这类画在控件之外的样式有溢出空间。
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl Styled for ExDialog {
    fn style(&mut self) -> &mut StyleRefinement {
        self.dialog.style()
    }
}

impl RenderOnce for ExDialog {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        let Self { dialog, children } = self;

        if children.is_empty() {
            return dialog.into_any_element();
        }

        dialog.child(padded().children(children)).into_any_element()
    }
}
