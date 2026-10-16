use gpui_kit::*;

use lingting_gpui_components::menu::{Menu, MenuSelectedStyle, MenuTextSize};
use lingting_gpui_nav::current_path;

use super::DesktopSidebarLayout;

/// 菜单的配置与生成。
///
/// 菜单项由 [`RouteDefinition`](super::RouteDefinition) 生成，逐项参数（路径、文案、图标、选中态）来自
/// 路由与当前路径，这里只提供对所有菜单项生效的公共参数。
impl DesktopSidebarLayout {
    /// 覆盖菜单项高度。
    pub fn menu_height(mut self, height: impl Into<Pixels>) -> Self {
        self.menu_height = Some(height.into());
        self
    }

    /// 覆盖菜单项图标尺寸。
    pub fn menu_icon_size(mut self, size: impl Into<Pixels>) -> Self {
        self.menu_icon_size = Some(size.into());
        self
    }

    /// 覆盖菜单项字号。
    ///
    /// 传 [`MenuTextSize`] 档位（跟随主题字号缩放），或直接传 `Pixels` 用固定像素值。
    pub fn menu_text_size(mut self, size: impl Into<MenuTextSize>) -> Self {
        self.menu_text_size = Some(size.into());
        self
    }

    /// 设置菜单选中样式。
    pub fn menu_selected_style(mut self, style: MenuSelectedStyle) -> Self {
        self.menu_selected_style = style;
        self
    }

    /// 侧栏菜单。
    ///
    /// 隐藏的路由不进入菜单；选中项按当前路径判定，因此 `navigate` 引起的重绘
    /// 会让选中态跟着路由走。
    pub(super) fn menu(&self, cx: &App) -> Vec<Menu> {
        let current = current_path(cx);
        self.routes
            .iter()
            .filter(|route| !route.is_hidden())
            .map(|route| {
                let mut menu = Menu::new(route.path(), route.label())
                    .selected(route.path() == current)
                    .selected_style(self.menu_selected_style);
                if let Some(icon) = route.icon_name() {
                    menu = menu.icon(icon);
                }
                if let Some(height) = self.menu_height {
                    menu = menu.height(height);
                }
                if let Some(size) = self.menu_icon_size {
                    menu = menu.icon_size(size);
                }
                if let Some(size) = self.menu_text_size {
                    menu = menu.text_size(size);
                }
                menu
            })
            .collect()
    }
}
