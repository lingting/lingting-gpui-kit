//! 在 `Dialog` 之上补一层内边距的对话框。

use gpui_kit::component::dialog::{Dialog, DialogButtonProps, DialogContent, DialogEntrance};
use gpui_kit::*;

/// 对话框。
///
/// API 与 [`Dialog`](gpui_kit::component::dialog::Dialog) 完全一致，差别只有一处：
/// 用户传入的内容与底部会各自被包一层 `p_1`（0.25rem）内边距，避免内容完全贴到弹窗边缘，
/// 导致投影、焦点环等需要溢出空间的样式被裁掉。未传入内容或底部时不加包装，行为与 `Dialog` 一致。
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
    dialog: Dialog,
}

impl ExDialog {
    /// 创建对话框。
    pub fn new(cx: &mut App) -> Self {
        Self {
            dialog: Dialog::new(cx),
        }
    }

    /// 转成内层的 [`Dialog`](gpui_kit::component::dialog::Dialog)。
    pub fn into_dialog(self) -> Dialog {
        self.dialog
    }

    /// 设置触发元素，点击后打开对话框。
    pub fn trigger(mut self, trigger: impl IntoElement) -> Self {
        self.dialog = self.dialog.trigger(trigger);
        self
    }

    /// 设置内容。
    ///
    /// 内边距在内容容器上补一层 `p_1`，其余布局与 `Dialog` 一致。
    pub fn content<F>(mut self, builder: F) -> Self
    where
        F: Fn(DialogContent, &mut Window, &mut App) -> DialogContent + 'static,
    {
        self.dialog = self
            .dialog
            .content(move |content, window, cx| builder(content.p_1(), window, cx));
        self
    }

    /// 设置标题。
    pub fn title(mut self, title: impl IntoElement) -> Self {
        self.dialog = self.dialog.title(title);
        self
    }

    /// 设置底部区域，通常放操作按钮。
    ///
    /// 用户构建出的底部会被包一层 `p_1`。
    pub fn footer(mut self, footer: impl IntoElement) -> Self {
        self.dialog = self.dialog.footer(div().p_1().child(footer));
        self
    }

    /// 设置按钮配置。
    pub fn button_props(mut self, button_props: DialogButtonProps) -> Self {
        self.dialog = self.dialog.button_props(button_props);
        self
    }

    /// 设置关闭后的回调。
    pub fn on_close(
        mut self,
        on_close: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.dialog = self.dialog.on_close(on_close);
        self
    }

    /// 设置确认回调，返回 `true` 关闭对话框。
    pub fn on_ok(
        mut self,
        on_ok: impl Fn(&ClickEvent, &mut Window, &mut App) -> bool + 'static,
    ) -> Self {
        self.dialog = self.dialog.on_ok(on_ok);
        self
    }

    /// 设置取消回调，返回 `true` 关闭对话框。
    pub fn on_cancel(
        mut self,
        on_cancel: impl Fn(&ClickEvent, &mut Window, &mut App) -> bool + 'static,
    ) -> Self {
        self.dialog = self.dialog.on_cancel(on_cancel);
        self
    }

    /// 是否显示右上角关闭按钮，默认 `true`。
    pub fn close_button(mut self, close_button: bool) -> Self {
        self.dialog = self.dialog.close_button(close_button);
        self
    }

    /// 设置顶部偏移，默认是视口高度的十分之一。
    pub fn margin_top(mut self, margin_top: impl Into<Pixels>) -> Self {
        self.dialog = self.dialog.margin_top(margin_top);
        self
    }

    /// 设置宽度，默认 448px。
    pub fn w(mut self, width: impl Into<Pixels>) -> Self {
        self.dialog = self.dialog.w(width);
        self
    }

    /// 设置宽度，默认 448px。
    pub fn width(mut self, width: impl Into<Pixels>) -> Self {
        self.dialog = self.dialog.width(width);
        self
    }

    /// 设置最大宽度，默认 `None`。
    pub fn max_w(mut self, max_width: impl Into<Pixels>) -> Self {
        self.dialog = self.dialog.max_w(max_width);
        self
    }

    /// 是否绘制遮罩，默认 `true`。
    pub fn overlay(mut self, overlay: bool) -> Self {
        self.dialog = self.dialog.overlay(overlay);
        self
    }

    /// 点击遮罩是否关闭，默认 `true`。
    pub fn overlay_closable(mut self, overlay_closable: bool) -> Self {
        self.dialog = self.dialog.overlay_closable(overlay_closable);
        self
    }

    /// 是否支持 `Esc` 关闭，默认 `true`。
    pub fn keyboard(mut self, keyboard: bool) -> Self {
        self.dialog = self.dialog.keyboard(keyboard);
        self
    }

    /// 设置入场动画，默认 [`DialogEntrance::SlideDown`]。
    pub fn entrance(mut self, entrance: DialogEntrance) -> Self {
        self.dialog = self.dialog.entrance(entrance);
        self
    }
}

impl From<Dialog> for ExDialog {
    fn from(dialog: Dialog) -> Self {
        Self { dialog }
    }
}

impl ParentElement for ExDialog {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.dialog.extend(elements);
    }
}

impl Styled for ExDialog {
    fn style(&mut self) -> &mut StyleRefinement {
        self.dialog.style()
    }
}

impl RenderOnce for ExDialog {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        self.dialog
    }
}
