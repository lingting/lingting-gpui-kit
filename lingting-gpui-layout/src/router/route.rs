use std::rc::Rc;

use gpui_kit::assets::IconName;
use gpui_kit::*;

use lingting_gpui_nav::{RouteFactory, init, navigate};

/// 一条路由：路径、菜单文案、页面工厂，以及可选的图标与显示 / 缓存策略。
///
/// 由 [`RouteDefinition::new`] 建立默认值：无图标、菜单可见、使用全局缓存，
/// 再按需覆盖其中部分。
///
/// 菜单出现在哪个区域由布局决定：`desktop_sidebar_layout` 放进侧栏，
/// `desktop_top_layout` 放进标题栏中间。
#[derive(Clone)]
pub struct RouteDefinition {
    path: SharedString,
    label: SharedString,
    icon: Option<IconName>,
    hidden: bool,
    cache: bool,
    create: RouteFactory,
}

impl RouteDefinition {
    /// 创建一条路由。
    ///
    /// `create` 是页面的构造闭包，与 [`lingting_gpui_nav::Router::add_cache`] 接受的形式一致；
    /// `path` 既是跳转地址，也是菜单项的稳定标识。
    pub fn new<T, F>(
        path: impl Into<SharedString>,
        label: impl Into<SharedString>,
        create: F,
    ) -> Self
    where
        T: Render + 'static,
        F: Fn(&mut Window, &mut App) -> T + 'static,
    {
        Self {
            path: path.into(),
            label: label.into(),
            icon: None,
            hidden: false,
            cache: true,
            create: Rc::new(move |window, cx| cx.new(|cx| create(window, cx)).into()),
        }
    }

    /// 指定菜单项图标，未指定时不渲染图标。
    pub fn icon(mut self, icon: IconName) -> Self {
        self.icon = Some(icon);
        self
    }

    /// 设置是否隐藏菜单项。隐藏后仍然可以跳转进来，只是不出现在菜单里。
    pub fn hidden(mut self, hidden: bool) -> Self {
        self.hidden = hidden;
        self
    }

    /// 设置是否全局缓存页面。
    ///
    /// 缓存时页面创建后一直保留，离开再进入仍然保留内部状态；
    /// 不缓存时页面只在停留期间存在，每次进入都是全新状态。
    pub fn cache(mut self, cache: bool) -> Self {
        self.cache = cache;
        self
    }

    /// 路由路径。
    pub fn path(&self) -> SharedString {
        self.path.clone()
    }

    /// 菜单文案。
    pub fn label(&self) -> SharedString {
        self.label.clone()
    }

    /// 菜单项图标。
    pub fn icon_name(&self) -> Option<IconName> {
        self.icon
    }

    /// 是否隐藏菜单项。
    pub fn is_hidden(&self) -> bool {
        self.hidden
    }

    /// 是否全局缓存页面。
    pub fn is_cached(&self) -> bool {
        self.cache
    }

    /// 页面工厂。每次调用返回同一个工厂的引用计数副本。
    pub fn factory(&self) -> RouteFactory {
        self.create.clone()
    }
}

/// 注册路由表并进入默认路由。必须在创建窗口之前调用。
///
/// 由各布局的 `init_router` 转发调用：使用者调用布局的 `init_router`，
/// 布局需要按自身需要扩展路由注册时只改这一层，不用修改使用者。
///
/// 路由表通过 [`lingting_gpui_nav::init`] 写入全局状态，之后再注册不会被已渲染的
/// [`lingting_gpui_nav::Outlet`] 感知。默认路径未出现在 `routes` 中时，
/// `navigate` 会静默忽略，此时内容区为空。
pub(crate) fn init_router(
    cx: &mut App,
    default_path: impl Into<SharedString>,
    routes: &[RouteDefinition],
) {
    let default_path = default_path.into();
    init(cx, |router| {
        for route in routes {
            router.add_erased(route.path(), route.factory(), route.is_cached());
        }
    });
    navigate(cx, default_path);
}
