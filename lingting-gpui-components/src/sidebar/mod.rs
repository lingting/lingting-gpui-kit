//! 侧栏。
//!
//! 分顶部、中间、底部三段：中间段吃掉剩余高度，因此底部段始终贴在侧栏底边，
//! 不会随着中间内容增减而上下浮动。
//! 侧栏不渲染任何内容，只决定三段的位置与内外边距；每段的子项被撑满内容宽度，
//! 与侧栏边缘之间的距离来自侧栏的 padding。
//!
//! 宽度与内边距可由调用方覆盖；顶部与底部段各自带一个最大高度，超出上限的内容被裁剪，
//! 也可以设为自适应，让内容有多高就多高。
mod section;

use section::Section;

use gpui_kit::base::v_flex;
use gpui_kit::component::ActiveTheme as _;
use gpui_kit::prelude::FluentBuilder as _;
use gpui_kit::*;

/// 侧栏宽度。
const SIDEBAR_WIDTH: Pixels = px(220.);

/// 侧栏内边距。
const SIDEBAR_PADDING: Pixels = px(8.);

/// 顶部段最大高度。
const TOP_MAX_HEIGHT: Pixels = px(160.);

/// 底部段最大高度。
const BOTTOM_MAX_HEIGHT: Pixels = px(160.);

/// 侧栏。
///
/// 由 [`Sidebar::new`] 建立默认值，再按需覆盖宽度、内边距与顶部 / 底部段的最大高度。
#[derive(IntoElement)]
pub struct Sidebar {
    width: Pixels,
    padding: Pixels,
    /// 顶部段最大高度，`None` 表示自适应。
    top_max_height: Option<Pixels>,
    /// 底部段最大高度，`None` 表示自适应。
    bottom_max_height: Option<Pixels>,
    top: Vec<AnyElement>,
    middle: Vec<AnyElement>,
    bottom: Vec<AnyElement>,
}

impl Sidebar {
    /// 创建一个三段皆空的侧栏：默认宽度 [`SIDEBAR_WIDTH`]、默认内边距 [`SIDEBAR_PADDING`]，
    /// 顶部与底部段各自使用默认最大高度。
    pub fn new() -> Self {
        Self {
            width: SIDEBAR_WIDTH,
            padding: SIDEBAR_PADDING,
            top_max_height: Some(TOP_MAX_HEIGHT),
            bottom_max_height: Some(BOTTOM_MAX_HEIGHT),
            top: Vec::new(),
            middle: Vec::new(),
            bottom: Vec::new(),
        }
    }

    /// 覆盖侧栏宽度，默认为 [`SIDEBAR_WIDTH`]。
    pub fn width(mut self, width: impl Into<Pixels>) -> Self {
        self.width = width.into();
        self
    }

    /// 覆盖侧栏内边距，默认为 [`SIDEBAR_PADDING`]。
    ///
    /// 三段与侧栏边缘之间的距离都来自这个值，段与段之间的距离不随它变化。
    pub fn padding(mut self, padding: impl Into<Pixels>) -> Self {
        self.padding = padding.into();
        self
    }

    /// 覆盖顶部段最大高度，默认为 [`TOP_MAX_HEIGHT`]。
    pub fn top_max_height(mut self, height: impl Into<Pixels>) -> Self {
        self.top_max_height = Some(height.into());
        self
    }

    /// 顶部段高度自适应：不设上限，由内容决定。
    pub fn top_max_height_auto(mut self) -> Self {
        self.top_max_height = None;
        self
    }

    /// 覆盖底部段最大高度，默认为 [`BOTTOM_MAX_HEIGHT`]。
    pub fn bottom_max_height(mut self, height: impl Into<Pixels>) -> Self {
        self.bottom_max_height = Some(height.into());
        self
    }

    /// 底部段高度自适应：不设上限，由内容决定。
    pub fn bottom_max_height_auto(mut self) -> Self {
        self.bottom_max_height = None;
        self
    }

    /// 追加顶部段的内容。
    pub fn top(mut self, items: impl IntoIterator<Item = impl IntoElement>) -> Self {
        self.top
            .extend(items.into_iter().map(IntoElement::into_any_element));
        self
    }

    /// 追加中间段的内容。中间段占满剩余高度，是导航列表的位置。
    pub fn middle(mut self, items: impl IntoIterator<Item = impl IntoElement>) -> Self {
        self.middle
            .extend(items.into_iter().map(IntoElement::into_any_element));
        self
    }

    /// 追加底部段的内容。
    pub fn bottom(mut self, items: impl IntoIterator<Item = impl IntoElement>) -> Self {
        self.bottom
            .extend(items.into_iter().map(IntoElement::into_any_element));
        self
    }
}

impl Default for Sidebar {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderOnce for Sidebar {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let mut sidebar = v_flex()
            .w(self.width)
            .h_full()
            .flex_shrink_0()
            .p(self.padding)
            .gap_1()
            .bg(cx.theme().muted)
            .border_r_1()
            .border_color(cx.theme().sidebar_border);

        // 空段不渲染，否则侧栏的段间距会在边缘留下一段空白。
        if !self.top.is_empty() {
            sidebar = sidebar.child(
                Section::new(self.top)
                    // 最大高度只设上限：内容更矮时按内容高度，更高时被截断并裁剪。
                    .when_some(self.top_max_height, |this, height| this.max_h(height)),
            );
        }
        if !self.middle.is_empty() {
            sidebar = sidebar.child(Section::new(self.middle).flex_1().min_h_0());
        }
        if !self.bottom.is_empty() {
            sidebar = sidebar.child(
                Section::new(self.bottom)
                    .when_some(self.bottom_max_height, |this, height| this.max_h(height)),
            );
        }

        sidebar
    }
}
