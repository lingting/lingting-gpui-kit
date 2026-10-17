//! 地区静态数据：查询与筛选。
//!
//! 数据来自 `framework-region`，全部是构建期生成的静态表，这里只做索引与匹配，
//! 不复制任何数据。

use std::collections::HashMap;
use std::sync::LazyLock;

use framework_region::regions::REGION_REGIONS;
use framework_region::types::Region;

/// ISO 代码到地区的索引，首次查询时构建一次。
static REGION_MAP: LazyLock<HashMap<&'static str, &'static Region>> = LazyLock::new(|| {
    REGION_REGIONS
        .iter()
        .map(|region| (region.iso, region))
        .collect()
});

/// 把地区代码统一成大写，并去掉首尾空白。
pub fn normalize_region_value(value: &str) -> String {
    value.trim().to_uppercase()
}

/// 按 ISO 代码查地区，大小写不敏感；未知代码或空输入返回 `None`。
pub fn find_region(value: &str) -> Option<&'static Region> {
    let iso = normalize_region_value(value);
    REGION_MAP.get(iso.as_str()).copied()
}

/// 默认筛选：输入为空时返回全部地区，否则按 [`region_matches`] 过滤。
pub fn filter_regions(input: &str) -> Vec<&'static Region> {
    let query = input.trim();
    if query.is_empty() {
        return REGION_REGIONS.iter().collect();
    }

    REGION_REGIONS
        .iter()
        .filter(|region| region_matches(region, query))
        .collect()
}

/// 地区是否命中搜索词。
///
/// 命中范围与参考实现一致：ISO / ISO3、国际区号、号码前缀、中英文名；
/// 英文名忽略大小写，其余按包含匹配。
pub fn region_matches(region: &Region, query: &str) -> bool {
    let query = query.trim();
    if query.is_empty() {
        return true;
    }

    let upper = query.to_uppercase();
    let lower = query.to_lowercase();

    region.iso.contains(upper.as_str())
        || region.iso3.contains(upper.as_str())
        || region.calling_codes.iter().any(|code| code.contains(query))
        || region
            .phone_prefixes
            .iter()
            .any(|prefix| prefix.contains(query))
        || region.name.zh.contains(query)
        || region.name.en.to_lowercase().contains(lower.as_str())
}
