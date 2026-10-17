//! 地区选择：单选走 `Select`，多选走 `Combobox`。
//!
//! 两者都是 GPUI Component 的搜索型下拉，数据统一由 [`RegionDelegate`] 提供：
//! 一行是「旗帜 + 区号 + 中文名」，搜索走 [`region_matches`](crate::region::data::region_matches)。
//!
//! 参考实现用一个 `multiple` 开关切换单选与多选，这里拆成两个类型：`Select` 只支持单选，
//! 多选必须走 `Combobox`，把两种语义塞进同一个状态反而更难用。

mod delegate;
mod listener;
mod multi;
mod single;

pub use delegate::{RegionDelegate, RegionItem, region_delegate, region_value};
pub use multi::{RegionMultiSelect, RegionMultiSelectHandler};
pub use single::{RegionSelect, RegionSelectHandler};
