// 各区域的配置方法与渲染助手分文件实现：本文件是类型本身与整体组装，
// 标题栏、侧栏、菜单、内容区分别在 title_bar、sidebar、menu、content 中。
mod content;
mod menu;
mod route;
mod sidebar;
mod title_bar;

pub use crate::router::RouteDefinition;
pub use route::init_router;

use std::rc::Rc;

use gpui_kit::base::{h_flex, v_flex};
use gpui_kit::component::ActiveTheme as _;
use gpui_kit::*;

use crate::slot::SlotRenderer;
use lingting_gpui_components::menu::MenuSelectedStyle;
use lingting_gpui_components::sidebar::{BOTTOM_MAX_HEIGHT, TOP_MAX_HEIGHT};
use lingting_gpui_nav::Outlet;

/// 桌面侧栏布局。
///
/// 结构：
///
/// ```text
/// ┌──────────────────────────────────────────────┐
/// │ 标题栏                                        │
/// ├───────────────┬──────────────────────────────┤
/// │ 侧栏           │ 内容区                        │
/// │  top           │                              │
/// │  middle（菜单） │                              │
/// │  bottom        │                              │
/// └───────────────┴──────────────────────────────┘
/// ```
///
/// 侧栏的菜单由本布局按 [`RouteDefinition`] 生成，调用方不需要自己拼菜单；
/// 其余部分（标题栏、侧栏顶 / 底段、内容区）的参数按前缀分组透传给对应组件。
///
/// 链式方法按目标组件分组：标题栏的基础参数（[`icon`](Self::icon)、
/// [`title`](Self::title)）不带前缀，标题栏其余参数用 `title_bar_`，
/// 侧栏参数用 `sidebar_`，菜单参数用 `menu_`，内容区内边距（`padding_*`）
/// 不带前缀。
///
/// 布局持有页面出口实体，因此是实体视图：用 `cx.new(|cx| DesktopSidebarLayout::new(cx)…)`
/// 创建，再交给 `open_window`。
pub struct DesktopSidebarLayout {
    outlet: Entity<Outlet>,
    routes: Rc<Vec<RouteDefinition>>,
    title_bar_height: Option<Pixels>,
    title_bar_icon: Option<SlotRenderer>,
    title: Option<SharedString>,
    title_bar_center: Option<SlotRenderer>,
    #[cfg(target_os = "windows")]
    title_bar_windows: lingting_gpui_components::title_bar::WindowsControls,
    sidebar_width: Option<Pixels>,
    sidebar_padding: Option<Pixels>,
    sidebar_top: Option<SlotRenderer>,
    sidebar_top_max_height: Option<Pixels>,
    sidebar_bottom: Option<SlotRenderer>,
    sidebar_bottom_max_height: Option<Pixels>,
    menu_height: Option<Pixels>,
    menu_icon_size: Option<Pixels>,
    menu_selected_style: MenuSelectedStyle,
    padding: Edges<Pixels>,
    content_background: Option<Hsla>,
}

impl DesktopSidebarLayout {
    /// 创建布局：标题栏无图标 / 标题 / 中间内容，侧栏顶 / 底段为空，
    /// 其余参数使用 [`TitleBar`](lingting_gpui_components::title_bar::TitleBar)、
    /// [`Sidebar`](lingting_gpui_components::sidebar::Sidebar)、
    /// [`Menu`](lingting_gpui_components::menu::Menu) 与
    /// [`Content`](crate::content::Content) 的默认值。
    pub fn new(cx: &mut Context<Self>) -> Self {
        Self {
            outlet: cx.new(Outlet::new),
            routes: Rc::new(Vec::new()),
            title_bar_height: None,
            title_bar_icon: None,
            title: None,
            title_bar_center: None,
            #[cfg(target_os = "windows")]
            title_bar_windows: lingting_gpui_components::title_bar::WindowsControls::new(),
            sidebar_width: None,
            sidebar_padding: None,
            sidebar_top: None,
            sidebar_top_max_height: Some(TOP_MAX_HEIGHT),
            sidebar_bottom: None,
            sidebar_bottom_max_height: Some(BOTTOM_MAX_HEIGHT),
            menu_height: None,
            menu_icon_size: None,
            menu_selected_style: MenuSelectedStyle::default(),
            padding: crate::content::default_padding(),
            content_background: None,
        }
    }

    /// 设置路由表。既用于生成侧栏菜单，也决定内容区能渲染哪些页面。
    ///
    /// 与 [`init_router`] 传入的是同一份路由：那里注册路由表，这里生成菜单。
    pub fn routes(mut self, routes: impl IntoIterator<Item = RouteDefinition>) -> Self {
        self.routes = Rc::new(routes.into_iter().collect());
        self
    }
}

impl Render for DesktopSidebarLayout {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .size_full()
            .bg(cx.theme().background)
            .child(self.title_bar(window, cx))
            .child(
                h_flex()
                    .items_stretch()
                    .flex_1()
                    .min_h_0()
                    .w_full()
                    .child(self.sidebar(window, cx))
                    .child(self.content()),
            )
    }
}
