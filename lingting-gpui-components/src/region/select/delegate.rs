//! 下拉数据源：一行地区的展示与搜索。

use gpui_kit::component::searchable_list::{SearchableListItem, SearchableVec};
use gpui_kit::*;

use framework_region::regions::REGION_REGIONS;
use framework_region::types::Region;

use crate::region::data::{find_region, region_matches};
use crate::region::flag::RegionFlag;

/// 旗帜与文字的间距，0.25rem（`gap-1`）。
const ITEM_GAP: Rems = rems(0.25);

/// 下拉里的一行地区。
///
/// 只持有 `framework-region` 的静态引用，克隆不复制数据。
#[derive(Clone, Copy, Debug)]
pub struct RegionItem(pub &'static Region);

impl SearchableListItem for RegionItem {
    /// 选项值就是 ISO 代码，与参考实现一致。
    type Value = &'static str;

    fn title(&self) -> SharedString {
        SharedString::new_static(self.0.name.zh)
    }

    fn display_title(&self) -> Option<AnyElement> {
        Some(default_row(self.0))
    }

    fn render(&self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        default_row(self.0)
    }

    fn value(&self) -> &Self::Value {
        &self.0.iso
    }

    fn matches(&self, query: &str) -> bool {
        region_matches(self.0, query)
    }
}

/// 下拉数据源：全部地区，搜索按 [`region_matches`] 过滤。
pub type RegionDelegate = SearchableVec<RegionItem>;

/// 默认数据源，包含 `framework-region` 里的全部地区，首次使用时构建一次。
///
/// 数据本身是静态的，数据源只存下标与筛选结果，因此全库共用一份。
pub fn region_delegate() -> RegionDelegate {
    static DELEGATE: std::sync::LazyLock<RegionDelegate> = std::sync::LazyLock::new(|| {
        SearchableVec::new(REGION_REGIONS.iter().map(RegionItem).collect::<Vec<_>>())
    });

    DELEGATE.clone()
}

/// 把用户传入的地区代码解析成数据源里的选项值，未知代码返回 `None`。
pub fn region_value(value: &str) -> Option<&'static str> {
    find_region(value).map(|region| region.iso)
}

/// 一行地区的默认展示：旗帜 + 区号 + 中文名。
fn default_row(region: &'static Region) -> AnyElement {
    let mut row = div()
        .flex()
        .flex_row()
        .items_center()
        .gap(ITEM_GAP)
        .child(RegionFlag::new(region.iso));

    if let Some(prefix) = region.phone_prefixes.first() {
        row = row.child(format!("+{prefix}"));
    }

    row.child(region.name.zh).into_any_element()
}
