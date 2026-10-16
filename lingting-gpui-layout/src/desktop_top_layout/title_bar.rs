use std::rc::Rc;

use gpui_kit::base::h_flex;
use gpui_kit::prelude::FluentBuilder as _;
use gpui_kit::*;

use lingting_gpui_components::title_bar::TitleBar;

use super::DesktopTopLayout;

/// 菜单项之间的间距。
const MENU_GAP: Pixels = px(4.);

/// 标题栏的配置与渲染。
///
/// 中间区域固定由本布局的菜单填充，因此这里不提供中间区域配置。
impl DesktopTopLayout {
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
    pub fn title_bar_height(mut self, height: impl Into<Pixels>) -> Self {
        self.title_bar_height = Some(height.into());
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

    /// 标题栏，菜单放在中间区域。
    ///
    /// 菜单只使用已注册的路由，未传入路由时中间区域为空，只剩两侧的拖动区。
    pub(super) fn title_bar(&self, window: &mut Window, cx: &mut App) -> TitleBar {
        let menu = self.menu(cx);
        let title_bar = TitleBar::new()
            .when_some(self.title_bar_height, |this, height| this.height(height))
            .when_some(self.title_bar_icon.as_ref(), |this, icon| {
                this.icon(icon(window, cx))
            })
            .when_some(self.title.clone(), |this, title| this.title(title))
            .when(!menu.is_empty(), |this| {
                this.center(h_flex().items_center().gap(MENU_GAP).children(menu))
            });
        #[cfg(target_os = "windows")]
        let title_bar = title_bar.windows(self.title_bar_windows);
        title_bar
    }
}
