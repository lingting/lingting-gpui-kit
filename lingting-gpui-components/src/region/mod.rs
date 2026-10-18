//! 国家与地区：静态数据查询、区域旗帜、标签、单选与多选。

mod assets;
mod data;
mod flag;
mod name;
mod select;
mod tag;

pub use assets::{FLAG_DIR, RegionAssets};
pub use data::{filter_regions, find_region, normalize_region_value, region_matches};
pub use flag::RegionFlag;
pub use name::RegionNameLanguage;
pub use select::{
    RegionDelegate, RegionItem, RegionMultiSelect, RegionMultiSelectHandler, RegionSelect,
    RegionSelectHandler, region_delegate, region_value,
};
pub use tag::{RegionRenderer, RegionTag};
