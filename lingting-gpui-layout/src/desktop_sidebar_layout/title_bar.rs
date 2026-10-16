use std::rc::Rc;

use gpui_kit::prelude::FluentBuilder as _;
use gpui_kit::*;

use lingting_gpui_components::title_bar::TitleBar;

use super::DesktopSidebarLayout;

/// 标题栏的配置与渲染。
impl DesktopSidebarLayout {
    /// 设置标题栏左侧图标，尺寸由调用方决定。
    pub fn icon(mut self, icon: impl Fn(&mut Window, &mut App) -> AnyElement + 'static) -> Self {
        self.title_bar_icon = Some(Rc::new(icon));
        self
    }

    /// 设置标题栏左侧标题。
    pub fn title(mut self, title: impl Into<SharedString>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// 覆盖标题栏高度。
    ///
    /// 标题栏与系统窗口装饰对齐，因此这里收 `Pixels` 而不是 `Rems`。
    pub fn title_bar_height(mut self, height: impl Into<Pixels>) -> Self {
        self.title_bar_height = Some(height.into());
        self
    }

    /// 设置标题栏中间区域的内容。
    pub fn title_bar_center(
        mut self,
        center: impl Fn(&mut Window, &mut App) -> AnyElement + 'static,
    ) -> Self {
        self.title_bar_center = Some(Rc::new(center));
        self
    }

    /// 设置 Windows 窗口控制按钮的尺寸与显示。
    #[cfg(target_os = "windows")]
    pub fn title_bar_windows(
        mut self,
        controls: lingting_gpui_components::title_bar::WindowsControls,
    ) -> Self {
        self.title_bar_windows = controls;
        self
    }

    /// 标题栏。
    pub(super) fn title_bar(&self, window: &mut Window, cx: &mut App) -> TitleBar {
        let title_bar = TitleBar::new()
            .when_some(self.title_bar_height, |this, height| this.height(height))
            .when_some(self.title_bar_icon.as_ref(), |this, icon| {
                this.icon(icon(window, cx))
            })
            .when_some(self.title.clone(), |this, title| this.title(title))
            .when_some(self.title_bar_center.as_ref(), |this, center| {
                this.center(center(window, cx))
            });
        #[cfg(target_os = "windows")]
        let title_bar = title_bar.windows(self.title_bar_windows);
        title_bar
    }
}
