use gpui_kit::*;

/// 选中项样式。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum MenuSelectedStyle {
    /// 浮起：主题背景色、一圈浅边与投影，视觉上从标题栏里浮起来。
    #[default]
    Elevated,
    /// 实底：主题色背景配反白文字与图标，选中项整块染色。
    Filled,
}

/// 选中项的投影。
///
/// GPUI 的 `blur_radius` 是高斯标准差，而 CSS 的 blur 是它的两倍，
/// 照搬 Tailwind 的半径会糊成一片灰雾，因此这里取一半的半径。
///
/// `BoxShadow` 的偏移、模糊与扩散半径在 GPUI 里就是 `Pixels`，只能用 `px()`。
pub(super) fn selected_shadow() -> Vec<BoxShadow> {
    let ink = hsla(0., 0., 0., 0.1);
    vec![
        BoxShadow::new(px(0.), px(1.), ink).blur_radius(px(1.5)),
        BoxShadow::new(px(0.), px(2.), ink)
            .blur_radius(px(2.))
            .spread_radius(px(-1.)),
    ]
}
