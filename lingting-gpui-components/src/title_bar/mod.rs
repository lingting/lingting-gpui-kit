#[cfg(target_os = "windows")]
mod controls;
mod left;

#[cfg(target_os = "windows")]
pub use controls::WindowsControls;

use gpui_kit::component::ActiveTheme as _;
use gpui_kit::*;

/// 未指定高度时使用的默认标题栏高度。
pub const TITLE_BAR_HEIGHT: Pixels = px(52.);

/// 标题栏。
///
/// 由 [`TitleBar::new`] 建立默认值，再按需覆盖其中的部分。
#[derive(IntoElement)]
pub struct TitleBar {
    height: Pixels,
    icon: Option<AnyElement>,
    title: Option<SharedString>,
    center: Option<AnyElement>,
    #[cfg(target_os = "windows")]
    windows: WindowsControls,
}

impl TitleBar {
    /// 创建标题栏：默认高度，无图标、标题与中间内容，Windows 下渲染默认尺寸的窗口按钮。
    pub fn new() -> Self {
        Self {
            height: TITLE_BAR_HEIGHT,
            icon: None,
            title: None,
            center: None,
            #[cfg(target_os = "windows")]
            windows: WindowsControls::new(),
        }
    }

    /// 设置标题栏高度。
    pub fn height(mut self, height: impl Into<Pixels>) -> Self {
        self.height = height.into();
        self
    }

    /// 设置左侧图标，尺寸由调用方自行决定。
    pub fn icon(mut self, icon: impl IntoElement) -> Self {
        self.icon = Some(icon.into_any_element());
        self
    }

    /// 设置左侧标题。
    pub fn title(mut self, title: impl Into<SharedString>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// 设置标题栏中间区域的内容。
    pub fn center(mut self, center: impl IntoElement) -> Self {
        self.center = Some(center.into_any_element());
        self
    }

    /// 设置 Windows 窗口控制按钮的尺寸与显示。
    #[cfg(target_os = "windows")]
    pub fn windows(mut self, controls: WindowsControls) -> Self {
        self.windows = controls;
        self
    }
}

impl Default for TitleBar {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderOnce for TitleBar {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        #[cfg(target_os = "windows")]
        let controls = controls::window_controls(window, &self.windows, cx);
        #[cfg(not(target_os = "windows"))]
        let controls: Option<AnyElement> = None;

        div()
            .flex()
            .flex_row()
            .items_center()
            .w_full()
            .h(self.height)
            .flex_shrink_0()
            .bg(cx.theme().title_bar)
            .border_b_1()
            .border_color(cx.theme().title_bar_border)
            .children(left::left(self.icon, self.title))
            .child(
                div()
                    .flex_1()
                    .min_w_0()
                    .flex()
                    .items_center()
                    .child(drag_spacer("title-bar-drag-left"))
                    .children(self.center)
                    .child(drag_spacer("title-bar-drag-right")),
            )
            .children(controls)
            .into_any_element()
    }
}

/// 中间内容两侧的空白拖动区。
fn drag_spacer(id: &'static str) -> AnyElement {
    div()
        .id(id)
        .flex_1()
        .h_full()
        .window_control_area(WindowControlArea::Drag)
        .into_any_element()
}
