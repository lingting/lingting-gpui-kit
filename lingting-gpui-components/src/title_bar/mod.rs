mod center;
#[cfg(target_os = "windows")]
mod controls;
mod left;
#[cfg(target_os = "windows")]
pub use controls::WindowsControls;

use gpui_kit::component::ActiveTheme as _;
use gpui_kit::*;

/// 未指定高度时使用的默认标题栏高度。
pub const TITLE_BAR_HEIGHT: Pixels = px(52.);

/// 未指定间距时使用的默认间距，用于隔开内容层的左侧、中间、右侧三个区域。
pub const TITLE_BAR_GAP: Pixels = px(4.);

/// 标题栏。
///
/// 由 [`TitleBar::new`] 建立默认值，再按需覆盖其中的部分。
#[derive(IntoElement)]
pub struct TitleBar {
    height: Pixels,
    gap: Pixels,
    icon: Option<AnyElement>,
    title: Option<SharedString>,
    center: Option<AnyElement>,
    #[cfg(target_os = "windows")]
    windows: WindowsControls,
}

impl TitleBar {
    /// 创建标题栏：默认高度与间距，无图标、标题与中间内容，Windows 下渲染默认尺寸的窗口按钮。
    pub fn new() -> Self {
        Self {
            height: TITLE_BAR_HEIGHT,
            gap: TITLE_BAR_GAP,
            icon: None,
            title: None,
            center: None,
            #[cfg(target_os = "windows")]
            windows: WindowsControls::new(),
        }
    }

    /// 设置内容层三个区域之间的间距。
    pub fn gap(mut self, gap: impl Into<Pixels>) -> Self {
        self.gap = gap.into();
        self
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
            .relative()
            .w_full()
            .h(self.height)
            .flex_shrink_0()
            // 基座拖动层铺满整条：左侧图标与标题、中间内容之外的空白、右侧按钮之外的空档
            // 都落在它上面。拖动与双击最大化由平台按 `HTCAPTION` 处理，不需要自己绑定事件。
            .child(
                div()
                    .id("title-bar-drag-base")
                    .absolute()
                    .inset_0()
                    .window_control_area(WindowControlArea::Drag),
            )
            // 内容层与基座同尺寸并重叠，自身是固定高度，因此内部可以直接用 `h_full()`。
            // `gap` 隔开左侧、中间、右侧三个区域。
            .child(
                div()
                    .relative()
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(self.gap)
                    .w_full()
                    .h(self.height)
                    .bg(cx.theme().title_bar)
                    .border_b_1()
                    .border_color(cx.theme().title_bar_border)
                    .children(left::left(self.icon, self.title))
                    .child(center::center(self.center))
                    .children(controls),
            )
            .into_any_element()
    }
}
