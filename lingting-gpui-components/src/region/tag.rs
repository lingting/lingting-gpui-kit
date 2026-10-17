//! 地区标签：旗帜 + 中文名，可换成自定义内容。

use std::rc::Rc;

use gpui_kit::component::ActiveTheme as _;
use gpui_kit::component::StyledExt as _;
use gpui_kit::*;

use framework_region::types::Region;

use crate::region::data::find_region;
use crate::region::flag::RegionFlag;

/// 自定义渲染：给一个地区，返回要显示的元素。
pub type RegionRenderer = Rc<dyn Fn(&Region) -> AnyElement>;

/// 旗帜与文字的间距，0.25rem（`gap-1`）。
const TAG_GAP: Rems = rems(0.25);

/// 地区标签。
///
/// 默认内容是「旗帜 + 中文名」；需要区号、英文名或其他排版时用
/// [`RegionTag::render_with`] 换成自己的渲染。地区代码未知时不渲染任何内容。
#[derive(IntoElement)]
pub struct RegionTag {
    value: String,
    renderer: Option<RegionRenderer>,
    style: StyleRefinement,
}

impl RegionTag {
    /// 按地区代码创建标签（ISO 3166-1 alpha-2，大小写不敏感）。
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            renderer: None,
            style: StyleRefinement::default(),
        }
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
            None => default_render(region, cx),
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

/// 默认内容：旗帜 + 中文名。
fn default_render(region: &Region, cx: &mut App) -> AnyElement {
    div()
        .flex()
        .flex_row()
        .items_center()
        .gap(TAG_GAP)
        .child(RegionFlag::new(region.iso))
        .child(
            div()
                .text_sm()
                .text_color(cx.theme().foreground)
                .child(region.name.zh),
        )
        .into_any_element()
}
