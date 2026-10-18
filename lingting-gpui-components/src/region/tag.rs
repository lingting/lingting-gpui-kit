//! 地区标签：旗帜 + 地区名，可换成自定义内容。

use std::rc::Rc;

use gpui_kit::component::ActiveTheme as _;
use gpui_kit::component::StyledExt as _;
use gpui_kit::prelude::FluentBuilder as _;
use gpui_kit::*;

use framework_region::types::Region;

use crate::region::data::find_region;
use crate::region::flag::RegionFlag;
use crate::region::name::RegionNameLanguage;

/// 自定义渲染：给一个地区，返回要显示的元素。
pub type RegionRenderer = Rc<dyn Fn(&Region) -> AnyElement>;

/// 旗帜与文字的间距，0.25rem（`gap-1`）。
const TAG_GAP: Rems = rems(0.25);

/// 地区标签。
///
/// 默认内容是「旗帜 + 中文名」，用 [`RegionTag::language`] 换成英文名，
/// 用 [`RegionTag::flag`] 关掉旗帜；需要区号、英文名之外的其他排版时用
/// [`RegionTag::render_with`] 换成自己的渲染。地区代码未知时不渲染任何内容。
#[derive(IntoElement)]
pub struct RegionTag {
    value: String,
    language: RegionNameLanguage,
    flag: bool,
    renderer: Option<RegionRenderer>,
    style: StyleRefinement,
}

impl RegionTag {
    /// 按地区代码创建标签（ISO 3166-1 alpha-2，大小写不敏感）。
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            language: RegionNameLanguage::default(),
            flag: true,
            renderer: None,
            style: StyleRefinement::default(),
        }
    }

    /// 设置地区名的文本语言，默认中文。
    pub fn language(mut self, language: RegionNameLanguage) -> Self {
        self.language = language;
        self
    }

    /// 是否在地区名前面显示旗帜，默认 `true`。
    pub fn flag(mut self, flag: bool) -> Self {
        self.flag = flag;
        self
    }

    /// 换掉默认渲染。
    pub fn render_with(mut self, renderer: impl Fn(&Region) -> AnyElement + 'static) -> Self {
        self.renderer = Some(Rc::new(renderer));
        self
    }
}

impl Styled for RegionTag {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for RegionTag {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let Some(region) = find_region(&self.value) else {
            return div().into_any_element();
        };

        let content = match &self.renderer {
            Some(renderer) => renderer(region),
            None => default_render(region, self.language, self.flag, cx),
        };

        div()
            .flex()
            .flex_row()
            .items_center()
            .gap(TAG_GAP)
            .refine_style(&self.style)
            .child(content)
            .into_any_element()
    }
}

/// 默认内容：旗帜（可选）+ 地区名。
fn default_render(
    region: &Region,
    language: RegionNameLanguage,
    flag: bool,
    cx: &mut App,
) -> AnyElement {
    div()
        .flex()
        .flex_row()
        .items_center()
        .gap(TAG_GAP)
        .when(flag, |this| this.child(RegionFlag::new(region.iso)))
        .child(
            div()
                .text_color(cx.theme().foreground)
                .child(language.name(region)),
        )
        .into_any_element()
}
