//! 卡片列表项：左 / 中 / 右三栏。
//!
//! 中间栏占用左右两栏之外的剩余宽度，左右两栏默认由内容自适应，
//! 需要各行对齐时再分别指定宽度。
//!
//! 与 [`super::Card`] 的区别是它自己绘制卡片表面：选中态需要同时改底色与边框色，
//! 而 [`super::Card`] 的表面不会为单个实例让步；圆角、边框与内边距因此在本组件内保持一致。
//! 行高由外部固定（虚拟列表按固定高度计算可见区间），因此各栏内容按行高居中而不是撑开高度。
//!
//! 三栏只负责布局，不套用任何内容样式：文字尺寸、颜色与截断都由调用方在传入的元素上声明。

use gpui_kit::component::ActiveTheme as _;
use gpui_kit::component::StyledExt as _;
use gpui_kit::prelude::FluentBuilder as _;
use gpui_kit::*;

/// 行点击处理。
type ClickHandler = Box<dyn Fn(&mut Window, &mut App) + 'static>;

/// 卡片列表项。
#[derive(IntoElement)]
pub struct ItemCard {
    id: ElementId,
    left: Option<AnyElement>,
    left_width: Option<Rems>,
    center: Option<AnyElement>,
    right: Option<AnyElement>,
    right_width: Option<Rems>,
    style: StyleRefinement,
    selected: bool,
    on_click: Option<ClickHandler>,
}

impl ItemCard {
    /// 创建列表项。`id` 决定元素身份与焦点归属，必须用稳定的业务标识，不能用列表下标。
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            left: None,
            left_width: None,
            center: None,
            right: None,
            right_width: None,
            style: StyleRefinement::default(),
            selected: false,
            on_click: None,
        }
    }

    /// 设置左侧栏。
    pub fn left(mut self, left: impl IntoElement) -> Self {
        self.left = Some(left.into_any_element());
        self
    }

    /// 覆盖左侧栏宽度，默认为内容自适应。
    pub fn left_width(mut self, width: Rems) -> Self {
        self.left_width = Some(width);
        self
    }

    /// 设置中间栏，占用左右两栏之外的剩余宽度。
    pub fn center(mut self, center: impl IntoElement) -> Self {
        self.center = Some(center.into_any_element());
        self
    }

    /// 把中间栏换成上下两块，各占行高的一半。
    pub fn with_center_split(mut self, top: impl IntoElement, bottom: impl IntoElement) -> Self {
        self.center = Some(
            div()
                .flex()
                .flex_col()
                .size_full()
                .child(div().flex_1().min_h_0().child(top))
                .child(div().flex_1().min_h_0().child(bottom))
                .into_any_element(),
        );
        self
    }

    /// 设置右侧栏。
    pub fn right(mut self, right: impl IntoElement) -> Self {
        self.right = Some(right.into_any_element());
        self
    }

    /// 覆盖右侧栏宽度，默认为内容自适应。
    pub fn right_width(mut self, width: Rems) -> Self {
        self.right_width = Some(width);
        self
    }

    /// 设置选中态。
    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

    /// 设置点击处理。行本身可点击，因此同时也提供键盘激活。
    pub fn on_click(mut self, handler: impl Fn(&mut Window, &mut App) + 'static) -> Self {
        self.on_click = Some(Box::new(handler));
        self
    }
}

impl Styled for ItemCard {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for ItemCard {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let Self {
            id,
            left,
            left_width,
            center,
            right,
            right_width,
            style,
            selected,
            on_click,
        } = self;

        // 行可点击，因此同时也是焦点目标：Tab 能到达，Enter / Space 能激活，
        // 并且焦点可见态由本组件绘制。
        let focus_handle = window
            .use_keyed_state((id.clone(), "focus"), cx, |_, cx| {
                cx.focus_handle().tab_stop(true)
            })
            .read(cx)
            .clone();

        div()
            .id(id)
            .flex()
            .items_center()
            .gap_3()
            .h_full()
            .w_full()
            .px_3()
            .refine_style(&style)
            .rounded_lg()
            .border_1()
            .when(selected, |this| {
                // 选中是持久状态，用元素自身的底色与边框表达，不用单侧边条。
                this.border_color(cx.theme().primary)
                    .bg(cx.theme().primary.opacity(0.08))
            })
            .when(!selected, |this| {
                this.bg(cx.theme().background)
                    .border_color(cx.theme().border)
                    .hover(|this| this.bg(cx.theme().secondary_hover))
            })
            .track_focus(&focus_handle)
            .focus_visible(|this| this.border_color(cx.theme().ring))
            .when_some(on_click, |this, on_click| {
                this.on_click(move |_, window, cx| on_click(window, cx))
            })
            .when_some(left, |this, left| {
                this.child(
                    div()
                        .flex()
                        .flex_shrink_0()
                        .items_center()
                        .when_some(left_width, |this, width| this.w(width))
                        .child(left),
                )
            })
            .child(
                // 中间栏占用剩余宽度：左右两栏固定，剩余空间全部给它。
                div()
                    .flex()
                    .flex_col()
                    .flex_1()
                    .min_w_0()
                    .h_full()
                    .justify_center()
                    .children(center),
            )
            .when_some(right, |this, right| {
                this.child(
                    div()
                        .flex()
                        .flex_shrink_0()
                        .items_center()
                        .when_some(right_width, |this, width| this.w(width))
                        .child(right),
                )
            })
    }
}
