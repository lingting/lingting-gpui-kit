use crate::card::ItemCardSelectedStyle;

/// 选中项样式。
///
/// 取值与 [`ItemCardSelectedStyle`] 一一对应：菜单项复用 `ItemCard` 渲染，
/// 这里只是菜单这一层的样式名，渲染时映射过去，因此菜单项也能用 `Tinted` 与 `Accent`。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum MenuSelectedStyle {
    /// 浅染：主题色边框配一层很淡的主题色底。
    Tinted,
    /// 浮起：主题背景色、一圈浅边与投影，视觉上从标题栏 / 侧栏里浮起来。
    #[default]
    Elevated,
    /// 强调：浮起的底色与投影之上，再把文字设为主题色并加粗。
    Accent,
    /// 实底：主题色背景配反白文字与图标，选中项整块染色。
    Filled,
}

impl MenuSelectedStyle {
    /// 对应的卡片选中样式。
    pub(crate) fn card(self) -> ItemCardSelectedStyle {
        match self {
            Self::Tinted => ItemCardSelectedStyle::Tinted,
            Self::Elevated => ItemCardSelectedStyle::Elevated,
            Self::Accent => ItemCardSelectedStyle::Accent,
            Self::Filled => ItemCardSelectedStyle::Filled,
        }
    }
}
