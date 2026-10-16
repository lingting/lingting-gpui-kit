use gpui_kit::component::ActiveTheme as _;
use gpui_kit::*;

/// 内容区上下内边距默认值。
pub(crate) const DEFAULT_PADDING_VERTICAL: Pixels = px(12.);

/// 内容区左右内边距默认值。
pub(crate) const DEFAULT_PADDING_HORIZONTAL: Pixels = px(16.);

/// 内容区默认内边距：上下与左右分别取值。
pub(crate) fn default_padding() -> Edges<Pixels> {
    Edges {
        top: DEFAULT_PADDING_VERTICAL,
        right: DEFAULT_PADDING_HORIZONTAL,
        bottom: DEFAULT_PADDING_VERTICAL,
        left: DEFAULT_PADDING_HORIZONTAL,
    }
}

/// 内容区域。
///
/// 布局中承载页面出口的那一块：横向吃掉剩余宽度、纵向吃掉剩余高度，
/// 并给页面留出四个方向的内边距。
///
/// 背景默认取主题的 `muted`，与侧栏同色系，页面卡片会在这层底色上浮起；
/// 需要跟布局主背景一致时用 [`background`](Self::background) 覆盖。
///
/// 内容区自己不管滚动：页面出口是固定尺寸的，超出部分由出口裁剪，
/// 页面需要滚动时在自己的范围内放滚动容器。
#[derive(IntoElement)]
pub struct Content {
    background: Option<Hsla>,
    padding_top: Pixels,
    padding_right: Pixels,
    padding_bottom: Pixels,
    padding_left: Pixels,
    children: Vec<AnyElement>,
}

impl Content {
    /// 创建内容区域：背景为默认值，上下内边距为 [`DEFAULT_PADDING_VERTICAL`]、
    /// 左右内边距为 [`DEFAULT_PADDING_HORIZONTAL`]。
    pub fn new() -> Self {
        let padding = default_padding();
        Self {
            background: None,
            padding_top: padding.top,
            padding_right: padding.right,
            padding_bottom: padding.bottom,
            padding_left: padding.left,
            children: Vec::new(),
        }
    }

    /// 覆盖背景色，默认为主题的 `muted`。
    pub fn background(mut self, background: impl Into<Hsla>) -> Self {
        self.background = Some(background.into());
        self
    }

    /// 覆盖四个方向的内边距。
    pub fn padding(mut self, padding: impl Into<Pixels>) -> Self {
        let padding = padding.into();
        self.padding_top = padding;
        self.padding_right = padding;
        self.padding_bottom = padding;
        self.padding_left = padding;
        self
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
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        // 背景在渲染时取主题色，跟随主题切换；显式设置过就用设置的值。
        div()
            .flex_1()
            .min_w_0()
            .min_h_0()
            .bg(self.background.unwrap_or(cx.theme().muted))
            .pt(self.padding_top)
            .pr(self.padding_right)
            .pb(self.padding_bottom)
            .pl(self.padding_left)
            .children(self.children)
    }
}
