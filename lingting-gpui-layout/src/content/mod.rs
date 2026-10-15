use gpui_kit::*;

/// 内容区内边距默认值。
pub(crate) const DEFAULT_PADDING: Pixels = px(16.);

/// 内容区域。
///
/// 布局中承载页面出口的那一块：横向吃掉剩余宽度、纵向吃掉剩余高度，
/// 并给页面留出四个方向的内边距。
///
/// 内容区自己不管滚动：页面出口是固定尺寸的，超出部分由出口裁剪，
/// 页面需要滚动时在自己的范围内放滚动容器。
#[derive(IntoElement)]
pub struct Content {
    padding_top: Pixels,
    padding_right: Pixels,
    padding_bottom: Pixels,
    padding_left: Pixels,
    children: Vec<AnyElement>,
}

impl Content {
    /// 创建内容区域：四个方向的内边距均为默认值 16px。
    pub fn new() -> Self {
        Self {
            padding_top: DEFAULT_PADDING,
            padding_right: DEFAULT_PADDING,
            padding_bottom: DEFAULT_PADDING,
            padding_left: DEFAULT_PADDING,
            children: Vec::new(),
        }
    }

    /// 覆盖上内边距。
    pub fn padding_top(mut self, padding: impl Into<Pixels>) -> Self {
        self.padding_top = padding.into();
        self
    }

    /// 覆盖右内边距。
    pub fn padding_right(mut self, padding: impl Into<Pixels>) -> Self {
        self.padding_right = padding.into();
        self
    }

    /// 覆盖下内边距。
    pub fn padding_bottom(mut self, padding: impl Into<Pixels>) -> Self {
        self.padding_bottom = padding.into();
        self
    }

    /// 覆盖左内边距。
    pub fn padding_left(mut self, padding: impl Into<Pixels>) -> Self {
        self.padding_left = padding.into();
        self
    }

    /// 追加内容。
    pub fn children(mut self, children: impl IntoIterator<Item = impl IntoElement>) -> Self {
        self.children
            .extend(children.into_iter().map(IntoElement::into_any_element));
        self
    }
}

impl Default for Content {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderOnce for Content {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        div()
            .flex_1()
            .min_w_0()
            .min_h_0()
            .pt(self.padding_top)
            .pr(self.padding_right)
            .pb(self.padding_bottom)
            .pl(self.padding_left)
            .children(self.children)
    }
}
