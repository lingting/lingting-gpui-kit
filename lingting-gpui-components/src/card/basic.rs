//! 卡片容器。
//!
//! 圆角容器，白底细边细投影；标题区高度自适应，内容区占用剩余空间。
//! 标题区内边距、内容区内边距与两者之间的间隔都可分别覆盖。
//! `ghost` 模式下不绘制卡片表面（背景、边框与投影），但标题区仍绘制一块带圆角的底色，
//! 用于把卡片当作纯布局容器使用。

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
            title_padding: TITLE_PADDING,
            content_padding: CONTENT_PADDING,
            gap: TITLE_GAP,
            title: None,
            children: Vec::new(),
        }
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
            title_padding,
            content_padding,
            gap,
            title,
            children,
        } = self;

        base.flex()
            .flex_col()
            .size_full()
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
