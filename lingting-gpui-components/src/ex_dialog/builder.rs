//! `ExDialog` 转发 `Dialog` 的全部构建方法，并在内容、底部与追加元素上补内边距。

use gpui_kit::component::dialog::{DialogButtonProps, DialogContent, DialogEntrance};
use gpui_kit::*;

use super::basic::{ExDialog, PADDING_X, PADDING_Y, padded};

impl ExDialog {
    /// 设置触发元素，点击后打开对话框。
    pub fn trigger(mut self, trigger: impl IntoElement) -> Self {
        self.dialog = self.dialog.trigger(trigger);
        self
    }

    /// 设置内容。
    ///
    /// 内边距在内容容器上补一层（上下 `0.5rem`、左右 `0.75rem`），其余布局与 `Dialog` 一致。
    pub fn content<F>(mut self, builder: F) -> Self
    where
        F: Fn(DialogContent, &mut Window, &mut App) -> DialogContent + 'static,
    {
        self.dialog = self.dialog.content(move |content, window, cx| {
            builder(content.py(PADDING_Y).px(PADDING_X), window, cx)
        });
        self
    }

    /// 设置标题。
    pub fn title(mut self, title: impl IntoElement) -> Self {
        self.dialog = self.dialog.title(title);
        self
    }

    /// 设置底部区域，通常放操作按钮。
    ///
    /// 用户构建出的底部会被补上内边距（上下 `0.5rem`、左右 `0.75rem`）。
    pub fn footer(mut self, footer: impl IntoElement) -> Self {
        self.dialog = self.dialog.footer(padded().child(footer));
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
