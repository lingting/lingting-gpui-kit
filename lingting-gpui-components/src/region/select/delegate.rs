//! 下拉数据源：一行地区的展示与搜索。

use std::sync::LazyLock;

use gpui_kit::component::searchable_list::{SearchableListItem, SearchableVec};
use gpui_kit::*;

use framework_region::regions::REGION_REGIONS;
use framework_region::types::Region;

use crate::region::data::{find_region, region_matches};
use crate::region::flag::RegionFlag;
use crate::region::name::RegionNameLanguage;

/// 旗帜与文字的间距，0.25rem（`gap-1`）。
const ITEM_GAP: Rems = rems(0.25);

/// 下拉里的一行地区。
///
/// 只持有 `framework_region` 的静态引用，克隆不复制数据。`language` 决定这一行显示哪种语言的
/// 地区名；`flag` 只作用于已选展示（`display_title`），下拉列表行始终带旗帜。
#[derive(Clone, Copy, Debug)]
pub struct RegionItem {
    region: &'static Region,
    language: RegionNameLanguage,
    flag: bool,
}

impl RegionItem {
    /// 按地区创建一行，语言默认中文、已选展示默认带旗帜。
    pub fn new(region: &'static Region) -> Self {
        Self {
            region,
            language: RegionNameLanguage::default(),
            flag: true,
        }
    }

    /// 换掉展示语言。
    pub fn language(mut self, language: RegionNameLanguage) -> Self {
        self.language = language;
        self
    }

    /// 换掉已选展示是否带旗帜。
    pub fn flag(mut self, flag: bool) -> Self {
        self.flag = flag;
        self
    }

    /// 地区静态数据。
    pub fn region(&self) -> &'static Region {
        self.region
    }

    /// 这一行要显示的地区名。
    pub fn name(&self) -> &'static str {
        self.language.name(self.region)
    }

    /// 已选展示里这一项的一行：按自身的语言与旗帜设置渲染。
    ///
    /// 下拉列表行用 [`SearchableListItem::render`]，固定带旗帜。
    pub(crate) fn trigger_row(&self) -> AnyElement {
        region_row(self.region, self.language, self.flag)
    }
}

impl SearchableListItem for RegionItem {
    /// 选项值就是 ISO 代码，与参考实现一致。
    type Value = &'static str;

    fn title(&self) -> SharedString {
        SharedString::new_static(self.name())
    }

    fn display_title(&self) -> Option<AnyElement> {
        Some(region_row(self.region, self.language, self.flag))
    }

    fn render(&self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        region_row(self.region, self.language, true)
    }

    fn value(&self) -> &Self::Value {
        &self.region.iso
    }

    fn matches(&self, query: &str) -> bool {
        region_matches(self.region, query)
    }
}

/// 下拉数据源：全部地区，搜索按 [`region_matches`] 过滤。
pub type RegionDelegate = SearchableVec<RegionItem>;

/// 取指定语言与旗帜设置的数据源，每种组合首次使用时构建一次。
///
/// 数据本身是静态的，数据源只存下标与筛选结果，因此每种组合全库共用一份；各组合的行顺序
/// 完全一致，同一个 ISO 代码在任意组合里都是同一个下标。
pub fn region_delegate(language: RegionNameLanguage, flag: bool) -> RegionDelegate {
    static DELEGATES: LazyLock<[[RegionDelegate; 2]; 2]> = LazyLock::new(|| {
        RegionNameLanguage::ALL.map(|language| {
            [true, false].map(|flag| {
                SearchableVec::new(
                    REGION_REGIONS
                        .iter()
                        .map(|region| RegionItem::new(region).language(language).flag(flag))
                        .collect::<Vec<_>>(),
                )
            })
        })
    });

    DELEGATES[language.index()][usize::from(!flag)].clone()
}

/// 把用户传入的地区代码解析成数据源里的选项值，未知代码返回 `None`。
pub fn region_value(value: &str) -> Option<&'static str> {
    find_region(value).map(|region| region.iso)
}

/// 一行地区的默认展示：旗帜（可选）+ 区号 + 地区名（语言见 `language`）。
pub(crate) fn region_row(
    region: &'static Region,
    language: RegionNameLanguage,
    flag: bool,
) -> AnyElement {
    let mut row = div().flex().flex_row().items_center().gap(ITEM_GAP);

    if flag {
        row = row.child(RegionFlag::new(region.iso));
    }

    if let Some(prefix) = region.phone_prefixes.first() {
        row = row.child(format!("+{prefix}"));
    }

    row.child(language.name(region)).into_any_element()
}
