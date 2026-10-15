//! 窗口控制按钮（最小化 / 最大化 / 关闭）。
//!
//! 仅在 Windows 平台编译与渲染。按钮通过 [`WindowControlArea`] 交给系统处理点击，
//! 因此不需要自己绑定点击事件。

use gpui_kit::assets::IconName;
use gpui_kit::component::ActiveTheme as _;
use gpui_kit::*;

/// 未指定宽度时使用的按钮宽度。
const DEFAULT_WIDTH: Pixels = px(46.);

/// 未指定尺寸时使用的图标尺寸。
const DEFAULT_ICON_SIZE: Pixels = px(14.);

/// Windows 窗口控制按钮的尺寸与显示配置。
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WindowsControls {
    width: Pixels,
    icon_size: Pixels,
    visible: bool,
}

impl WindowsControls {
    /// 创建配置：宽度 46、图标 14，并渲染按钮。
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置单个按钮的宽度。
    pub fn width(mut self, width: impl Into<Pixels>) -> Self {
        self.width = width.into();
        self
    }

    /// 设置按钮图标的尺寸。
    pub fn icon_size(mut self, icon_size: impl Into<Pixels>) -> Self {
        self.icon_size = icon_size.into();
        self
    }

    /// 设置是否渲染这三个按钮。
    pub fn visible(mut self, visible: bool) -> Self {
        self.visible = visible;
        self
    }

    /// 是否渲染这三个按钮。
    pub fn is_visible(&self) -> bool {
        self.visible
    }
}

impl Default for WindowsControls {
    fn default() -> Self {
        Self {
            width: DEFAULT_WIDTH,
            icon_size: DEFAULT_ICON_SIZE,
            visible: true,
        }
    }
}

/// 窗口控制按钮组。配置为不渲染时返回 `None`。
pub(crate) fn window_controls(
    window: &Window,
    controls: &WindowsControls,
    cx: &App,
) -> Option<AnyElement> {
    if !controls.is_visible() {
        return None;
    }

    let maximize_icon = if window.is_maximized() {
        IconName::WindowRestore
    } else {
        IconName::WindowMaximize
    };
    let foreground = cx.theme().foreground;
    let hover_bg = cx.theme().secondary_hover;
    let close_bg = cx.theme().danger;

    Some(
        div()
            .flex()
            .flex_row()
            .items_center()
            .h_full()
            .flex_shrink_0()
            .child(button(
                "window-minimize",
                IconName::WindowMinimize,
                WindowControlArea::Min,
                hover_bg,
                foreground,
                controls,
            ))
            .child(button(
                "window-maximize",
                maximize_icon,
                WindowControlArea::Max,
                hover_bg,
                foreground,
                controls,
            ))
            .child(button(
                "window-close",
                IconName::WindowClose,
                WindowControlArea::Close,
                close_bg,
                foreground,
                controls,
            ))
            .into_any_element(),
    )
}

/// 一个窗口控制按钮。
///
/// 颜色必须显式设在 `svg()` 自身：SVG 元素取的是元素自己的文本颜色，
/// 既不继承父元素，缺省时也不会绘制。
fn button(
    id: &'static str,
    icon: IconName,
    area: WindowControlArea,
    hover_bg: Hsla,
    icon_color: Hsla,
    controls: &WindowsControls,
) -> AnyElement {
    div()
        .id(id)
        .flex()
        .items_center()
        .justify_center()
        .w(controls.width)
        .h_full()
        .flex_shrink_0()
        .cursor_pointer()
        .hover(move |this| this.bg(hover_bg))
        .window_control_area(area)
        .child(
            svg()
                .path(icon.path())
                .size(controls.icon_size)
                .text_color(icon_color),
        )
        .into_any_element()
}
