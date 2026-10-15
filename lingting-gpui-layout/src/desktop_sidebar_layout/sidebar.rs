use std::rc::Rc;

use gpui_kit::prelude::FluentBuilder as _;
use gpui_kit::*;

use lingting_gpui_components::sidebar::Sidebar;

use super::DesktopSidebarLayout;

/// 侧栏的配置与渲染。
///
/// 中间段固定由本布局的菜单填充，因此这里不提供 middle 配置。
impl DesktopSidebarLayout {
    /// 覆盖侧栏宽度。
    pub fn sidebar_width(mut self, width: impl Into<Pixels>) -> Self {
        self.sidebar_width = Some(width.into());
        self
    }

    /// 覆盖侧栏内边距。
    pub fn sidebar_padding(mut self, padding: impl Into<Pixels>) -> Self {
        self.sidebar_padding = Some(padding.into());
        self
    }

    /// 设置侧栏顶部段的内容。
    pub fn sidebar_top(
        mut self,
        top: impl Fn(&mut Window, &mut App) -> AnyElement + 'static,
    ) -> Self {
        self.sidebar_top = Some(Rc::new(top));
        self
    }

    /// 覆盖侧栏顶部段的最大高度。
    pub fn sidebar_top_max_height(mut self, height: impl Into<Pixels>) -> Self {
        self.sidebar_top_max_height = Some(height.into());
        self
    }

    /// 侧栏顶部段高度自适应：不设上限，由内容决定。
    pub fn sidebar_top_max_height_auto(mut self) -> Self {
        self.sidebar_top_max_height = None;
        self
    }

    /// 设置侧栏底部段的内容。
    pub fn sidebar_bottom(
        mut self,
        bottom: impl Fn(&mut Window, &mut App) -> AnyElement + 'static,
    ) -> Self {
        self.sidebar_bottom = Some(Rc::new(bottom));
        self
    }

    /// 覆盖侧栏底部段的最大高度。
    pub fn sidebar_bottom_max_height(mut self, height: impl Into<Pixels>) -> Self {
        self.sidebar_bottom_max_height = Some(height.into());
        self
    }

    /// 侧栏底部段高度自适应：不设上限，由内容决定。
    pub fn sidebar_bottom_max_height_auto(mut self) -> Self {
        self.sidebar_bottom_max_height = None;
        self
    }

    /// 侧栏，菜单放在中间段。
    pub(super) fn sidebar(&self, window: &mut Window, cx: &mut App) -> Sidebar {
        let menu = self.menu(cx);
        self.sidebar_style(Sidebar::new())
            .when_some(self.sidebar_top.as_ref(), |this, top| {
                this.top([top(window, cx)])
            })
            .when(!menu.is_empty(), |this| this.middle(menu))
            .when_some(self.sidebar_bottom.as_ref(), |this, bottom| {
                this.bottom([bottom(window, cx)])
            })
    }

    /// 应用侧栏的尺寸参数。
    fn sidebar_style(&self, sidebar: Sidebar) -> Sidebar {
        sidebar
            .when_some(self.sidebar_width, |this, width| this.width(width))
            .when_some(self.sidebar_padding, |this, padding| this.padding(padding))
            .map(|this| match self.sidebar_top_max_height {
                Some(height) => this.top_max_height(height),
                None => this.top_max_height_auto(),
            })
            .map(|this| match self.sidebar_bottom_max_height {
                Some(height) => this.bottom_max_height(height),
                None => this.bottom_max_height_auto(),
            })
    }
}
