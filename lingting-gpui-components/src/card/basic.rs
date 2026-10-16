use gpui_kit::component::ActiveTheme as _;
use gpui_kit::prelude::FluentBuilder as _;
use gpui_kit::*;
/// 标题区默认内边距。
const TITLE_PADDING: Rems = rems(0.5);

/// 内容区默认内边距（0，即不设内边距，由内容自己决定）。
const CONTENT_PADDING: Rems = rems(0.);

/// 标题区与内容区之间的默认间隔。
const TITLE_GAP: Rems = rems(0.5);

/// 卡片。
#[derive(IntoElement)]
pub struct Card {
    base: Div,
    ghost: bool,
    height: Length,
    width: Length,
    title_padding: Rems,
    content_padding: Rems,
    gap: Rems,
    title: Option<AnyElement>,
    children: Vec<AnyElement>,
}

impl Card {
    pub fn new() -> Self {
        Self {
            base: div(),
            ghost: false,
            height: Length::Auto,
            width: Length::Auto,
            title_padding: TITLE_PADDING,
            content_padding: CONTENT_PADDING,
            gap: TITLE_GAP,
            title: None,
            children: Vec::new(),
        }
    }

    /// 高度与宽度都铺满可用空间（100%）。
    pub fn size_full(self) -> Self {
        self.height_full().width_full()
    }

    /// 高度与宽度都不声明，由内容与父布局决定。默认值。
    pub fn size_auto(self) -> Self {
        self.height_auto().width_auto()
    }

    /// 同时设置高度与宽度为固定长度。
    pub fn size(mut self, width: Rems, height: Rems) -> Self {
        self.width = width.into();
        self.height = height.into();
        self
    }

    /// 设置高度为固定长度。
    pub fn height(mut self, height: Rems) -> Self {
        self.height = height.into();
        self
    }

    /// 设置高度铺满可用空间（100%）。
    pub fn height_full(mut self) -> Self {
        self.height = relative(1.).into();
        self
    }

    /// 高度不声明，由内容与父布局决定。默认值。
    pub fn height_auto(mut self) -> Self {
        self.height = Length::Auto;
        self
    }

    /// 设置宽度为固定长度。
    pub fn width(mut self, width: Rems) -> Self {
        self.width = width.into();
        self
    }

    /// 设置宽度铺满可用空间（100%）。
    pub fn width_full(mut self) -> Self {
        self.width = relative(1.).into();
        self
    }

    /// 宽度不声明，由内容与父布局决定。默认值。
    pub fn width_auto(mut self) -> Self {
        self.width = Length::Auto;
        self
    }

    /// 设置 ghost 模式：不绘制卡片表面（背景、边框与投影），标题区仍保留卡片底色。
    pub fn ghost(mut self, ghost: bool) -> Self {
        self.ghost = ghost;
        self
    }

    /// 启用 ghost 模式，等价于 `ghost(true)`。
    pub fn with_ghost(self) -> Self {
        self.ghost(true)
    }

    /// 设置标题。标题区高度自适应：内边距加标题自身高度。
    pub fn title(mut self, title: impl IntoElement) -> Self {
        self.title = Some(title.into_any_element());
        self
    }

    /// 覆盖标题区内边距，默认为 `TITLE_PADDING`。
    pub fn title_padding(mut self, padding: Rems) -> Self {
        self.title_padding = padding;
        self
    }

    /// 覆盖内容区内边距，默认为 `CONTENT_PADDING`（0）。
    pub fn content_padding(mut self, padding: Rems) -> Self {
        self.content_padding = padding;
        self
    }

    /// 覆盖标题区与内容区之间的间隔，默认为 `TITLE_GAP`。
    pub fn gap(mut self, gap: Rems) -> Self {
        self.gap = gap;
        self
    }
}

impl Default for Card {
    fn default() -> Self {
        Self::new()
    }
}

impl Styled for Card {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl ParentElement for Card {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements)
    }
}

impl RenderOnce for Card {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let Self {
            base,
            ghost,
            height,
            width,
            title_padding,
            content_padding,
            gap,
            title,
            children,
        } = self;

        // 尺寸默认 Auto：不声明尺寸，由内容与父布局决定。
        // min_h_0 只在高度被显式限制（`height_full` / `height`）时生效：让卡片按声明的高度收缩，不被内容撑破。
        base.flex()
            .flex_col()
            .h(height)
            .w(width)
            .min_h_0()
            .overflow_hidden()
            .rounded_lg()
            .when(!ghost, |this| {
                this.bg(cx.theme().background)
                    .border_1()
                    .border_color(cx.theme().border)
                    .when(cx.theme().shadow, |this| this.shadow_sm())
            })
            .when_some(title, |this, title| {
                this.child(
                    div()
                        .flex()
                        .flex_col()
                        .flex_shrink_0()
                        .w_full()
                        // 标题区高度自适应：只由内边距与标题自身高度决定。
                        .p(title_padding)
                        // 间隔用外边距而不是留在标题块内部：ghost 下标题块有自己的底色，
                        // 间隔若算在块内，就会被这块底色填满。
                        .mb(gap)
                        // ghost 不绘制卡片表面，标题区仍绘制一块底色：
                        // 标题在页面底色上需要一块自己的表面，圆角与卡片保持一致。
                        .when(ghost, |this| this.bg(cx.theme().background).rounded_lg())
                        .child(title),
                )
            })
            .child(
                // 内容区只负责给出剩余高度：内容自己声明 `size_full` 或 `flex_1` 时铺满，
                // 未声明时按内容高度排布，不会为了填满空间被拉伸。
                div()
                    .flex()
                    .flex_col()
                    .flex_1()
                    .min_h_0()
                    .w_full()
                    .p(content_padding)
                    .children(children),
            )
    }
}
