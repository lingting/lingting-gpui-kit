use gpui_kit::*;

use crate::content::Content;

use super::DesktopTopLayout;

/// 内容区的配置与渲染。
///
/// 内容区只承载页面出口，因此这里不提供内容槽位，只提供内边距。
impl DesktopTopLayout {
    /// 覆盖内容区上内边距。
    pub fn padding_top(mut self, padding: impl Into<Pixels>) -> Self {
        self.padding.top = padding.into();
        self
    }

    /// 覆盖内容区右内边距。
    pub fn padding_right(mut self, padding: impl Into<Pixels>) -> Self {
        self.padding.right = padding.into();
        self
    }

    /// 覆盖内容区下内边距。
    pub fn padding_bottom(mut self, padding: impl Into<Pixels>) -> Self {
        self.padding.bottom = padding.into();
        self
    }

    /// 覆盖内容区左内边距。
    pub fn padding_left(mut self, padding: impl Into<Pixels>) -> Self {
        self.padding.left = padding.into();
        self
    }

    /// 内容区，页面出口是它唯一的内容。
    pub(super) fn content(&self) -> Content {
        Content::new()
            .padding_top(self.padding.top)
            .padding_right(self.padding.right)
            .padding_bottom(self.padding.bottom)
            .padding_left(self.padding.left)
            .children([self.outlet.clone()])
    }
}
