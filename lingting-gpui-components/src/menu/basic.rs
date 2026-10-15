use gpui_kit::assets::IconName;
use gpui_kit::component::ActiveTheme as _;
use gpui_kit::prelude::FluentBuilder as _;
use gpui_kit::*;

use lingting_gpui_nav::navigate;

/// 图标尺寸默认值。
const ICON_SIZE: Pixels = px(14.);

/// 菜单项高度默认值。
const ITEM_HEIGHT: Pixels = px(32.);

/// 选中项样式。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum MenuSelectedStyle {
    /// 浮起：主题背景色、一圈浅边与投影，视觉上从标题栏里浮起来。
    #[default]
    Elevated,
    /// 实底：主题色背景配反白文字与图标，选中项整块染色。
    Filled,
}

/// 标题栏菜单项。
///
/// 由 [`Menu::new`] 建立默认值：不显示图标、高度 32px、未选中、
/// [`MenuSelectedStyle::Elevated`]，再按需覆盖其中部分。
#[derive(IntoElement)]
pub struct Menu {
    path: &'static str,
    label: &'static str,
    icon: Option<IconName>,
    icon_size: Pixels,
    height: Pixels,
    selected: bool,
    selected_style: MenuSelectedStyle,
}

impl Menu {
    /// 创建菜单项。
    ///
    /// `path` 既是点击后跳转的路由，也是元素 id。
    pub fn new(path: &'static str, label: &'static str) -> Self {
        Self {
            path,
            label,
            icon: None,
            icon_size: ICON_SIZE,
            height: ITEM_HEIGHT,
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
    pub fn icon_size(mut self, size: impl Into<Pixels>) -> Self {
        self.icon_size = size.into();
        self
    }

    /// 覆盖菜单项高度。
    pub fn height(mut self, height: impl Into<Pixels>) -> Self {
        self.height = height.into();
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

/// 选中项的投影。
///
/// GPUI 的 `blur_radius` 是高斯标准差，而 CSS 的 blur 是它的两倍，
/// 照搬 Tailwind 的半径会糊成一片灰雾，因此这里取一半的半径。
fn selected_shadow() -> Vec<BoxShadow> {
    let ink = hsla(0., 0., 0., 0.1);
    vec![
        BoxShadow::new(px(0.), px(1.), ink).blur_radius(px(1.5)),
        BoxShadow::new(px(0.), px(2.), ink)
            .blur_radius(px(2.))
            .spread_radius(px(-1.)),
    ]
}

impl RenderOnce for Menu {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        let path = self.path;
        let filled = self.selected && self.selected_style == MenuSelectedStyle::Filled;
        // SVG 不继承文本颜色，必须显式给值，否则图标不绘制。
        let icon_color = if filled {
            theme.primary_foreground
        } else {
            theme.foreground
        };

        div()
            .id(self.path)
            .flex()
            .items_center()
            .gap_2()
            .px_4()
            .h(self.height)
            .rounded_md()
            .text_sm()
            .cursor_pointer()
            .when(self.selected, |this| match self.selected_style {
                MenuSelectedStyle::Elevated => this
                    .bg(theme.background)
                    .text_color(theme.foreground)
                    .border_1()
                    .border_color(theme.border)
                    .shadow(selected_shadow()),
                MenuSelectedStyle::Filled => this
                    .bg(theme.primary)
                    .text_color(theme.primary_foreground)
                    .hover(|this| this.bg(theme.primary_hover)),
            })
            .when(!self.selected, |this| {
                this.hover(|this| this.bg(theme.secondary_hover))
            })
            .on_click(move |_, _, cx| navigate(cx, path))
            .when_some(self.icon, |this, icon| {
                this.child(
                    svg()
                        .path(icon.path())
                        .size(self.icon_size)
                        .text_color(icon_color),
                )
            })
            .child(self.label)
            .into_any_element()
    }
}
