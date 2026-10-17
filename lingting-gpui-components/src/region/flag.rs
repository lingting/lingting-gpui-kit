//! 区域旗帜元素。
//!
//! GPUI 的 `svg()` 把 SVG 当单色遮罩绘制（只保留 alpha，用文字颜色填充），彩色旗帜会被涂成
//! 一整块色块，因此这里改用 `img()`：SVG 由 GPUI 的图片管线按原色栅格化，并按资源路径缓存
//! 在进程内，同一面旗帜只解析一次。
//!
//! 前提是应用把 [`RegionAssets`] 合并进了自己的 `AssetSource`（见该类型的说明），
//! 否则图片加载失败，旗帜位置留空。

use gpui_kit::component::StyledExt as _;
use gpui_kit::*;

use crate::region::assets::RegionAssets;
use crate::region::data::find_region;

/// 旗帜宽度默认值，1.25rem（`w-5`），与参考实现一致。
const FLAG_WIDTH: Rems = rems(1.25);

/// 旗帜高度默认值，1rem（`h-4`），与参考实现一致。
const FLAG_HEIGHT: Rems = rems(1.);

/// 圆角默认值，0.125rem，让旗帜不至于那么方正。
const FLAG_RADIUS: Rems = rems(0.125);

/// 区域旗帜元素。
///
/// 由 [`RegionFlag::new`] 指定地区代码（ISO 3166-1 alpha-2，大小写不敏感）；
/// 未知代码或资源缺失时不渲染任何内容。
#[derive(IntoElement)]
pub struct RegionFlag {
    value: String,
    width: Rems,
    height: Rems,
    style: StyleRefinement,
}

impl RegionFlag {
    /// 按地区代码创建旗帜。
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            width: FLAG_WIDTH,
            height: FLAG_HEIGHT,
            style: StyleRefinement::default(),
        }
    }

    /// 设置旗帜宽度。
    pub fn width(mut self, width: impl Into<Rems>) -> Self {
        self.width = width.into();
        self
    }

    /// 设置旗帜高度。
    pub fn height(mut self, height: impl Into<Rems>) -> Self {
        self.height = height.into();
        self
    }
}

impl Styled for RegionFlag {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for RegionFlag {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let Some(region) = find_region(&self.value) else {
            return div().into_any_element();
        };

        div()
            .w(self.width)
            .h(self.height)
            .flex_shrink_0()
            .overflow_hidden()
            .rounded(FLAG_RADIUS)
            .refine_style(&self.style)
            .child(
                img(RegionAssets::flag_path(region.flag))
                    .w_full()
                    .h_full()
                    // 旗帜原始比例 4:3，元素 5:4，用 cover 裁掉左右多余部分。
                    .object_fit(ObjectFit::Cover),
            )
            .into_any_element()
    }
}
