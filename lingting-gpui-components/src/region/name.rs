//! 地区名的展示语言。

use framework_region::types::Region;

/// 地区名展示用的文本语言。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum RegionNameLanguage {
    /// 中文名。
    #[default]
    Zh,
    /// 英文名。
    En,
}

impl RegionNameLanguage {
    /// 全部取值，用于按语言预建数据源。
    pub(crate) const ALL: [Self; 2] = [Self::Zh, Self::En];

    /// 取地区名。
    pub fn name(self, region: &Region) -> &'static str {
        match self {
            Self::Zh => region.name.zh,
            Self::En => region.name.en,
        }
    }

    /// 短标识，用于拼元素状态与缓存的下标。
    pub(crate) fn key(self) -> &'static str {
        match self {
            Self::Zh => "zh",
            Self::En => "en",
        }
    }

    /// 在 [`RegionNameLanguage::ALL`] 里的下标。
    pub(crate) fn index(self) -> usize {
        match self {
            Self::Zh => 0,
            Self::En => 1,
        }
    }
}
