use gpui_kit::component::ActiveTheme as _;
use gpui_kit::component::StyledExt as _;
use gpui_kit::component::Theme;
use gpui_kit::prelude::FluentBuilder as _;
use gpui_kit::*;

/// 选中样式。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ItemCardSelectedStyle {
    /// 浅染：主题色边框配一层很淡的主题色底，默认样式。
    #[default]
    Tinted,
    /// 浮起：主题背景色、一圈浅边与投影，视觉上从列表里浮起来。
    Elevated,
    /// 强调：浮起的底色与投影之上，再把文字设为主题色并加粗。
    Accent,
    /// 实底：主题色背景配反白文字，选中项整块染色。
    Filled,
}

impl ItemCardSelectedStyle {
    /// 图标颜色。
    ///
    /// SVG 不继承文本颜色，图标必须由调用方显式设色；这里按样式给出取值，免得每个调用点
    /// 自己判断选中态：未选中取 `unselected`（卡片自己不渲染图标，缺省色由调用方决定），
    /// 选中时 `Filled` 取反白、其余取主题色。
    pub fn icon_color(self, selected: bool, unselected: Hsla, theme: &Theme) -> Hsla {
        if !selected {
            return unselected;
        }
        match self {
            Self::Filled => theme.primary_foreground,
            Self::Tinted | Self::Elevated | Self::Accent => theme.primary,
        }
    }
}

/// 选中项的投影。
///
/// GPUI 的 `blur_radius` 是高斯标准差，而 CSS 的 blur 是它的两倍，
/// 照搬 Tailwind 的半径会糊成一片灰雾，因此这里取一半的半径。
///
/// `BoxShadow` 的偏移、模糊与扩散半径在 GPUI 里就是 `Pixels`，只能用 `px()`。
fn selected_shadow() -> Vec<BoxShadow> {
    let ink = hsla(0., 0., 0., 0.1);
    vec![
        BoxShadow::new(px(0.), px(1.), ink).blur_radius(px(1.5)),
        BoxShadow::new(px(0.), px(2.), ink)
            .blur_radius(px(2.))
            .spread_radius(px(-1.)),
    ]
}

/// 水平内边距默认值，0.75rem（`px_3`）。
const HORIZONTAL_PADDING: Rems = rems(0.75);

/// 圆角默认值，0.5rem（`rounded_lg`）。
const RADIUS: Rems = rems(0.5);

/// 栏间距默认值，0.75rem（`gap_3`）。
const GAP: Rems = rems(0.75);

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
    /// 四边内边距，默认水平 `rems(0.75)`、垂直 `rems(0.)`（与原先的 `px_3` 一致）。
    padding: Edges<Rems>,
    radius: Rems,
    gap: Rems,
    /// 未选中时是否绘制边框，默认 `true`。
    border_unselected: bool,
    /// 未选中时的底色，默认取主题 `background`。
    bg_unselected: Option<Hsla>,
    style: StyleRefinement,
    selected: bool,
    selected_style: ItemCardSelectedStyle,
    hoverable: bool,
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
            padding: Edges {
                top: rems(0.),
                right: HORIZONTAL_PADDING,
                bottom: rems(0.),
                left: HORIZONTAL_PADDING,
            },
            radius: RADIUS,
            gap: GAP,
            border_unselected: true,
            bg_unselected: None,
            style: StyleRefinement::default(),
            selected: false,
            selected_style: ItemCardSelectedStyle::default(),
            hoverable: true,
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

    /// 覆盖四边内边距，默认垂直 `rems(0.)`、水平 `rems(0.75)`。
    ///
    /// 行高由 `h` 决定，上下内边距会把内容区压小，因此默认只在水平方向留白；
    /// 与 [`Styled::p`](gpui_kit::Styled::p) 不同，这里在用户样式之后应用，不会被 `refine_style` 覆盖。
    pub fn padding(mut self, padding: impl Into<Rems>) -> Self {
        let padding = padding.into();
        self.padding = Edges::all(padding);
        self
    }

    /// 只覆盖水平内边距。
    pub fn padding_x(mut self, padding: impl Into<Rems>) -> Self {
        let padding = padding.into();
        self.padding.left = padding;
        self.padding.right = padding;
        self
    }

    /// 只覆盖垂直内边距。
    pub fn padding_y(mut self, padding: impl Into<Rems>) -> Self {
        let padding = padding.into();
        self.padding.top = padding;
        self.padding.bottom = padding;
        self
    }

    /// 覆盖圆角，默认为 `rems(0.5)`（`rounded_lg`）。
    pub fn radius(mut self, radius: impl Into<Rems>) -> Self {
        self.radius = radius.into();
        self
    }

    /// 覆盖栏间距，默认为 `rems(0.75)`（`gap_3`）。
    pub fn gap(mut self, gap: impl Into<Rems>) -> Self {
        self.gap = gap.into();
        self
    }

    /// 设置未选中时是否绘制边框，默认为 `true`。
    ///
    /// 关掉后未选中态只剩底色与悬停反馈，选中态仍然按 [`ItemCardSelectedStyle`] 画边框。
    pub fn border_unselected(mut self, border: bool) -> Self {
        self.border_unselected = border;
        self
    }

    /// 设置未选中时的底色，默认为主题 `background`。
    ///
    /// 放在标题栏、侧栏这类已有底色的容器里时，卡片可以传 `transparent_black()` 让容器底色透出。
    /// 悬停底色仍然按 `secondary_hover` 叠加在上层。
    pub fn bg_unselected(mut self, color: Hsla) -> Self {
        self.bg_unselected = Some(color);
        self
    }

    /// 设置选中态。
    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }
    /// 设置选中样式，默认为 [`ItemCardSelectedStyle::Tinted`]。
    pub fn selected_style(mut self, style: ItemCardSelectedStyle) -> Self {
        self.selected_style = style;
        self
    }
    /// 设置是否应用悬浮样式，默认为 `true`。
    ///
    /// 纯展示的卡片没有可点击行为，悬浮反馈会把它误读成可点击，此时关掉即可。
    pub fn hoverable(mut self, hoverable: bool) -> Self {
        self.hoverable = hoverable;
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
            padding,
            radius,
            gap,
            border_unselected,
            bg_unselected,
            style,
            selected,
            selected_style,
            hoverable,
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
            .gap(gap)
            .h_full()
            .w_full()
            .pt(padding.top)
            .pb(padding.bottom)
            .pl(padding.left)
            .pr(padding.right)
            .refine_style(&style)
            .rounded(radius)
            .when(!selected || border_unselected, |this| this.border_1())
            .when(selected, |this| match selected_style {
                // 选中是持久状态，用元素自身的底色与边框表达，不用单侧边条。
                ItemCardSelectedStyle::Tinted => this
                    .border_color(cx.theme().primary)
                    .bg(cx.theme().primary.opacity(0.08)),
                ItemCardSelectedStyle::Elevated => this
                    .bg(cx.theme().background)
                    .border_color(cx.theme().border)
                    .shadow(selected_shadow()),
                ItemCardSelectedStyle::Accent => this
                    .bg(cx.theme().background)
                    .border_color(cx.theme().border)
                    .shadow(selected_shadow())
                    // 调用方在子元素上声明的颜色优先级更高，这里的文字颜色只作为缺省值。
                    .text_color(cx.theme().primary)
                    .font_semibold(),
                ItemCardSelectedStyle::Filled => this
                    .bg(cx.theme().primary)
                    .border_color(cx.theme().primary)
                    // 调用方在子元素上声明的颜色优先级更高，这里的文字颜色只作为缺省值。
                    .text_color(cx.theme().primary_foreground)
                    .when(hoverable, |this| {
                        this.hover(|this| this.bg(cx.theme().primary_hover))
                    }),
            })
            .when(!selected, |this| {
                this.bg(bg_unselected.unwrap_or(cx.theme().background))
                    .when(border_unselected, |this| {
                        this.border_color(cx.theme().border)
                    })
                    .when(hoverable, |this| {
                        this.hover(|this| this.bg(cx.theme().secondary_hover))
                    })
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
