use gpui_kit::{
    Bounds, DevicePixels, Pixels, Point, SharedString, Size, TitlebarOptions,
    WindowBackgroundAppearance, WindowBounds, WindowOptions, point, px, size,
};

use crate::window_options_builder::screen::{PrimaryScreen, primary_screen};
#[cfg(target_os = "macos")]
use crate::window_options_builder::traffic_light;

/// 未指定窗口尺寸时使用的物理尺寸。
const DEFAULT_SIZE: Size<DevicePixels> = size(DevicePixels(1600), DevicePixels(1024));

/// 窗口选项构建器。
///
/// 尺寸按物理像素传入，[`build`](Self::build) 时统一换算成 GPUI 使用的逻辑像素：
///
/// - 屏幕缩放因子由平台查询，逻辑尺寸 = 物理尺寸 / 缩放因子；
/// - 换算后的尺寸超出屏幕可见区域时（任意一边超出，或屏幕更小），窗口以最大化状态创建，
///   除非用 [`clamp_to_screen`](Self::clamp_to_screen) 关掉这一行为；
/// - 未超出时窗口按可见区域居中；最小尺寸默认与初始尺寸相同，避免最小值大于初始值。
///
/// ```text
/// let options = WindowOptionsBuilder::new()
///     .title("我的应用")
///     .size(size(DevicePixels(1600), DevicePixels(1024)))
///     .build();
/// ```
#[derive(Clone, Debug)]
pub struct WindowOptionsBuilder {
    bounds: Option<WindowBounds>,
    transparent: bool,
    title: Option<SharedString>,
    size: Size<DevicePixels>,
    /// 最小尺寸；未设置时与 [`size`](Self::size) 相同。
    size_min: Option<Size<DevicePixels>>,
    /// macOS 红绿灯位置；`None` 表示用 [`traffic_light::default_position`]。
    #[cfg(target_os = "macos")]
    traffic_light_position: Option<Point<Pixels>>,
    clamp_to_screen: bool,
}

impl WindowOptionsBuilder {
    /// 创建构建器：默认尺寸 [`DEFAULT_SIZE`]、最小尺寸与初始尺寸相同、透明、
    /// 不设置标题、按屏幕钳制。
    pub fn new() -> Self {
        Self {
            bounds: None,
            transparent: true,
            title: None,
            size: DEFAULT_SIZE,
            size_min: None,
            #[cfg(target_os = "macos")]
            traffic_light_position: None,
            clamp_to_screen: true,
        }
    }

    /// 设置窗口状态与位置，覆盖 [`size`](Self::size) 决定的位置。
    ///
    /// 传入 [`WindowBounds`] 可以让窗口以最大化或全屏状态创建；其中携带的尺寸与位置都是
    /// 逻辑像素，不再经过屏幕换算。
    pub fn bounds(mut self, bounds: WindowBounds) -> Self {
        self.bounds = Some(bounds);
        self
    }

    /// 设置窗口是否透明，默认 `true`。
    ///
    /// 为 `true` 时窗口背景取 [`WindowBackgroundAppearance::Transparent`]，
    /// `titlebar.appears_transparent` 随之为 `true`（隐藏系统标题栏交给自绘标题栏），
    /// macOS 上还会把红绿灯按钮压进自绘标题栏。
    pub fn transparent(mut self, transparent: bool) -> Self {
        self.transparent = transparent;
        self
    }

    /// 设置窗口标题，默认不设置。
    ///
    /// 标题同时用于系统任务栏等外部位置，与自绘标题栏上显示的标题无关。
    pub fn title(mut self, title: impl Into<SharedString>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// 设置初始尺寸，传入物理像素。
    pub fn size(mut self, size: Size<DevicePixels>) -> Self {
        self.size = size;
        self
    }

    /// 设置最小尺寸，传入物理像素；未设置时与 [`size`](Self::size) 相同。
    pub fn size_min(mut self, size_min: Size<DevicePixels>) -> Self {
        self.size_min = Some(size_min);
        self
    }

    /// 设置 macOS 红绿灯按钮位置，坐标以窗口左下角为原点、逻辑像素。
    ///
    /// 默认把红绿灯垂直居中于自绘标题栏内；只在
    /// [`transparent`](Self::transparent) 为 `true` 时生效。
    #[cfg(target_os = "macos")]
    pub fn traffic_light_position(mut self, position: Point<Pixels>) -> Self {
        self.traffic_light_position = Some(position);
        self
    }

    /// 设置换算后的尺寸超出屏幕时是否转为最大化创建，默认 `true`。
    ///
    /// 关闭后超出屏幕的尺寸照常以窗口化状态创建，由窗口系统自行裁剪。
    pub fn clamp_to_screen(mut self, clamp_to_screen: bool) -> Self {
        self.clamp_to_screen = clamp_to_screen;
        self
    }

    /// 构造 [`WindowOptions`]。
    pub fn build(&self) -> WindowOptions {
        let screen = primary_screen();

        // 屏幕缩放因子在窗口创建前只能由平台查询，查询失败时按 1.0 处理。
        let scale_factor = screen.map_or(1.0, |screen| screen.scale_factor);
        let size = self.size.to_pixels(scale_factor);
        // 未设置最小尺寸时与初始尺寸一致，保证窗口不会被系统放大。
        let size_min = self.size_min.unwrap_or(self.size).to_pixels(scale_factor);

        let window_bounds = match (self.bounds, screen) {
            (Some(bounds), _) => bounds,
            (None, Some(screen)) => self.centered_bounds(size, screen),
            (None, None) => WindowBounds::Windowed(Bounds::new(point(px(0.), px(0.)), size)),
        };

        WindowOptions {
            window_bounds: Some(window_bounds),
            titlebar: Some(TitlebarOptions {
                title: self.title.clone(),
                appears_transparent: self.transparent,
                traffic_light_position: self.traffic_light_position(),
            }),
            window_background: if self.transparent {
                WindowBackgroundAppearance::Transparent
            } else {
                WindowBackgroundAppearance::Opaque
            },
            // 最小值不超过初始值，否则窗口创建时会被系统放大。
            window_min_size: Some(size_min.min(&size)),
            ..Default::default()
        }
    }

    /// 按屏幕可见区域居中；换算后的尺寸超出可见区域时转为最大化创建。
    fn centered_bounds(&self, size: Size<Pixels>, screen: PrimaryScreen) -> WindowBounds {
        let visible_bounds = screen.visible_bounds.to_pixels(screen.scale_factor);
        let fits =
            size.width <= visible_bounds.size.width && size.height <= visible_bounds.size.height;

        if !fits && self.clamp_to_screen {
            // 最大化状态仍需要还原尺寸，这里给出屏幕可见区域的尺寸。
            return WindowBounds::Maximized(Bounds::new(
                visible_bounds.origin,
                visible_bounds.size,
            ));
        }

        // 居中时按可见区域裁剪，避免窗口覆盖任务栏。
        let clamped_size = size.min(&visible_bounds.size);
        WindowBounds::Windowed(Bounds::centered_at(visible_bounds.center(), clamped_size))
    }

    /// 红绿灯位置：不透明窗口没有自绘标题栏可压，交给窗口系统默认摆放。
    fn traffic_light_position(&self) -> Option<Point<Pixels>> {
        // 只在透明窗口上生效：不透明窗口没有自绘标题栏可压。
        #[cfg(target_os = "macos")]
        if self.transparent {
            return Some(
                self.traffic_light_position
                    .unwrap_or_else(traffic_light::default_position),
            );
        }

        None
    }
}

impl Default for WindowOptionsBuilder {
    fn default() -> Self {
        Self::new()
    }
}
