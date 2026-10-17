use gpui_kit::assets::IconName;
use gpui_kit::component::ActiveTheme as _;
use gpui_kit::prelude::FluentBuilder as _;
use gpui_kit::*;

use lingting_gpui_nav::navigate;

use crate::card::ItemCard;

use super::constants::{ICON_SIZE, ITEM_HEIGHT, TEXT_SIZE};
use super::selected_style::MenuSelectedStyle;

/// 标题栏菜单项。
///
/// 行本身由 [`ItemCard`] 渲染：三栏布局、选中样式、悬浮与焦点反馈都来自卡片，
/// 这里只负责把图标、文案与跳转行为填进去。
///
/// 由 [`Menu::new`] 建立默认值：不显示图标、高度 2rem、字号 `text_sm`、未选中、
/// [`MenuSelectedStyle::Elevated`]，再按需覆盖其中部分。
///
/// 长度参数收 [`AbsoluteLength`]，rem 与像素都能传，默认用 rem；放进标题栏时改传
/// `px()`，见 [`TITLE_BAR_ITEM_HEIGHT`](super::constants::TITLE_BAR_ITEM_HEIGHT)。
#[derive(IntoElement)]
pub struct Menu {
    path: SharedString,
    label: SharedString,
    icon: Option<IconName>,
    icon_size: AbsoluteLength,
    height: AbsoluteLength,
    text_size: AbsoluteLength,
    selected: bool,
    selected_style: MenuSelectedStyle,
}

impl Menu {
    /// 创建菜单项。
    ///
    /// `path` 既是点击后跳转的路由，也是元素 id。
    pub fn new(path: impl Into<SharedString>, label: impl Into<SharedString>) -> Self {
        Self {
            path: path.into(),
            label: label.into(),
            icon: None,
            icon_size: ICON_SIZE.into(),
            height: ITEM_HEIGHT.into(),
            text_size: TEXT_SIZE.into(),
            selected: false,
            selected_style: MenuSelectedStyle::default(),
        }
    }

    /// 指定图标，未指定时不渲染图标。
    pub fn icon(mut self, icon: IconName) -> Self {
        self.icon = Some(icon);
        self
    }

    /// 覆盖图标尺寸。
    pub fn icon_size(mut self, size: impl Into<AbsoluteLength>) -> Self {
        self.icon_size = size.into();
        self
    }

    /// 覆盖菜单项高度。
    pub fn height(mut self, height: impl Into<AbsoluteLength>) -> Self {
        self.height = height.into();
        self
    }

    /// 覆盖菜单项字号，默认为 `text_sm`。
    pub fn text_size(mut self, size: impl Into<AbsoluteLength>) -> Self {
        self.text_size = size.into();
        self
    }

    /// 设置选中状态，通常由调用方与当前路由比较后传入。
    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

    /// 设置选中样式。
    pub fn selected_style(mut self, style: MenuSelectedStyle) -> Self {
        self.selected_style = style;
        self
    }
}

impl RenderOnce for Menu {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        let path = self.path.clone();
        let selected_style = self.selected_style;
        // SVG 不继承文本颜色，必须显式给值，否则图标不绘制。
        let icon_color = selected_style
            .card()
            .icon_color(self.selected, theme.foreground, theme);

        ItemCard::new(self.path)
            .selected(self.selected)
            .selected_style(selected_style.card())
            .h(self.height)
            .text_size(self.text_size)
            // 还原菜单项原来的行样式：`px_4`、`gap_2`、`rounded_md`，未选中时不画边框。
            .padding_x(rems(1.))
            .padding_y(rems(0.))
            .gap(rems(0.5))
            .radius(rems(0.375))
            .border_unselected(false)
            // 菜单项自身的底色交给外层容器，未选中时不铺 `background`，否则会把标题栏 / 侧栏的底色盖住。
            .bg_unselected(transparent_black())
            .cursor_pointer()
            .on_click(move |_, cx| navigate(cx, &path))
            .when_some(self.icon, |this, icon| {
                this.left(
                    svg()
                        .path(icon.path())
                        .size(self.icon_size)
                        .text_color(icon_color),
                )
            })
            .center(self.label)
    }
}
