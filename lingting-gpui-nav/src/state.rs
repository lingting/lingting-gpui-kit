//! 当前路由状态与页面缓存。

use std::collections::HashMap;

use gpui_kit::*;

use super::route::{RouteFactory, Router};

/// 应用级路由表。必须在创建窗口之前通过 [`init`] 注册。
struct RouteTable(Router);

impl Global for RouteTable {}

/// 全局页面缓存，键为路由路径。
///
/// `AnyView` 内部持有实体的强引用，因此页面一旦创建就会一直存活，
/// 切换页面再切回来时页面内部状态不会丢失。
#[derive(Default)]
struct PageCache(HashMap<SharedString, AnyView>);

impl Global for PageCache {}

/// 当前非缓存路由的页面。
///
/// 只保留正在显示的那一个页面，路径切换时清空，
/// 于是页面离开路由后被丢弃，下次进入重新创建。
#[derive(Default)]
struct ActiveView(Option<(SharedString, AnyView)>);

impl Global for ActiveView {}

/// 当前路由状态。
///
/// 对外可见是为了让 [`super::router::Outlet`] 能订阅它的变化。
pub struct RoutePath {
    path: SharedString,
}

impl Global for RoutePath {}

/// 注册全局路由表。必须在创建窗口之前调用。
pub fn init(cx: &mut App, configure: impl FnOnce(&mut Router)) {
    let mut router = Router::default();
    configure(&mut router);
    cx.set_global(RouteTable(router));
    cx.set_global(PageCache::default());
    cx.set_global(ActiveView::default());
    cx.set_global(RoutePath {
        path: SharedString::default(),
    });
}

/// 当前路径。
pub fn current_path(cx: &App) -> SharedString {
    cx.global::<RoutePath>().path.clone()
}

/// 切换页面。
///
/// 未注册的路径会被忽略，避免渲染出空白内容。
pub fn navigate(cx: &mut App, path: impl Into<SharedString>) {
    let path = path.into();
    if !cx.global::<RouteTable>().0.contains(&path) {
        return;
    }
    if cx.global::<RoutePath>().path == path {
        return;
    }
    cx.global_mut::<RoutePath>().path = path;
    // 非缓存页面只在停留期间存在，路径变化后丢弃。
    *cx.global_mut::<ActiveView>() = ActiveView::default();
    cx.refresh_windows();
}

/// 取得当前路径对应的页面，必要时创建并缓存。
pub(crate) fn page_for_current_path(window: &mut Window, cx: &mut App) -> Option<AnyView> {
    let path = current_path(cx);
    let (is_cached, factory) = {
        let route = cx.global::<RouteTable>().0.find(&path)?;
        (route.is_cached(), route.factory())
    };

    if is_cached {
        return Some(cached_page(&path, factory, window, cx));
    }
    Some(active_page(&path, factory, window, cx))
}

/// 全局缓存页面：命中缓存直接复用，否则创建后写入缓存。
fn cached_page(
    path: &SharedString,
    factory: RouteFactory,
    window: &mut Window,
    cx: &mut App,
) -> AnyView {
    if let Some(view) = cx.global::<PageCache>().0.get(path) {
        return view.clone();
    }

    let view = factory(window, cx);
    cx.global_mut::<PageCache>()
        .0
        .insert(path.clone(), view.clone());
    view
}

/// 非缓存页面：路径不变时复用停留期间的那一份页面。
fn active_page(
    path: &SharedString,
    factory: RouteFactory,
    window: &mut Window,
    cx: &mut App,
) -> AnyView {
    if let Some((active_path, view)) = cx.global::<ActiveView>().0.as_ref()
        && active_path == path
    {
        return view.clone();
    }

    let view = factory(window, cx);
    cx.global_mut::<ActiveView>().0 = Some((path.clone(), view.clone()));
    view
}
