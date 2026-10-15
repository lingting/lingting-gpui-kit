use std::rc::Rc;

use gpui_kit::*;

/// 创建页面实体的工厂，返回类型擦除后的 [`AnyView`] 便于统一缓存。
///
/// 用 `Rc` 而非裸函数指针，是因为工厂需要捕获页面的构造闭包。
pub(crate) type RouteFactory = Rc<dyn Fn(&mut Window, &mut App) -> AnyView>;

/// 页面的缓存策略。
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum CachePolicy {
    /// 全局缓存：页面创建后一直保留，再次进入时复用同一份页面状态。
    Global,
    /// 不缓存：页面只在停留期间存在，离开路由后丢弃。
    None,
}

/// 一条路由记录。
pub(crate) struct Route {
    path: SharedString,
    factory: RouteFactory,
    policy: CachePolicy,
}

impl Route {
    /// 创建一条路由记录。
    pub(crate) fn new<T, F>(path: impl Into<SharedString>, create: F, policy: CachePolicy) -> Self
    where
        T: Render + 'static,
        F: Fn(&mut Window, &mut App) -> T + 'static,
    {
        Self {
            path: path.into(),
            factory: Rc::new(move |window, cx| cx.new(|cx| create(window, cx)).into()),
            policy,
        }
    }

    pub(crate) fn path(&self) -> &str {
        &self.path
    }

    /// 是否使用全局页面缓存。
    pub(crate) fn is_cached(&self) -> bool {
        self.policy == CachePolicy::Global
    }

    pub(crate) fn factory(&self) -> RouteFactory {
        self.factory.clone()
    }
}

/// 已注册的路由表。
#[derive(Default)]
pub struct Router {
    routes: Vec<Route>,
}

impl Router {
    /// 注册一条全局缓存路由。
    ///
    /// 页面在首次进入时通过 [`App::new`] 创建，之后由页面缓存复用，
    /// 所以再次进入同一路由时页面内部状态仍然保留。
    pub fn add_cache<T, F>(&mut self, path: impl Into<SharedString>, create: F)
    where
        T: Render + 'static,
        F: Fn(&mut Window, &mut App) -> T + 'static,
    {
        self.routes
            .push(Route::new(path, create, CachePolicy::Global));
    }

    /// 注册一条不缓存路由。
    ///
    /// 页面在进入路由时创建，离开路由后丢弃，因此每次进入都是全新状态。
    pub fn add_view<T, F>(&mut self, path: impl Into<SharedString>, create: F)
    where
        T: Render + 'static,
        F: Fn(&mut Window, &mut App) -> T + 'static,
    {
        self.routes
            .push(Route::new(path, create, CachePolicy::None));
    }

    pub(crate) fn find(&self, path: &str) -> Option<&Route> {
        self.routes.iter().find(|route| route.path() == path)
    }

    pub(crate) fn contains(&self, path: &str) -> bool {
        self.find(path).is_some()
    }
}
