//! 区域旗帜资源。
//!
//! 246 个区域旗帜 SVG 内嵌在本 crate 的 `assets/flags` 下，通过 [`RegionAssets`] 交给 GPUI
//! 的 `AssetSource` 读取；资源路径固定为 `flags/<flag>.svg`，`<flag>` 取自
//! `framework-region` 的 `Region::flag`。

use std::borrow::Cow;

use gpui_kit::*;
use rust_embed::RustEmbed;

/// 区域旗帜资源目录：`Region::flag` 加上扩展名就是完整路径。
pub const FLAG_DIR: &str = "flags";

/// 区域旗帜 SVG 的内嵌资源源。
///
/// 应用需要把它合并进自己的 `AssetSource`，否则旗帜画不出来：
///
/// ```rust,no_run
/// use std::borrow::Cow;
/// use lingting_gpui_components::region::RegionAssets;
///
/// struct AppAssets;
///
/// impl gpui::AssetSource for AppAssets {
///     fn load(&self, path: &str) -> gpui::Result<Option<Cow<'static, [u8]>>> {
///         if let Some(bytes) = RegionAssets::load(path)? {
///             return Ok(Some(bytes));
///         }
///         gpui_kit::assets::Assets.load(path)
///     }
///
///     fn list(&self, path: &str) -> gpui::Result<Vec<gpui::SharedString>> {
///         let mut paths = gpui_kit::assets::Assets.list(path)?;
///         paths.extend(RegionAssets::list(path)?);
///         paths.sort();
///         paths.dedup();
///         Ok(paths)
///     }
/// }
/// ```
#[derive(RustEmbed)]
#[folder = "assets/flags"]
#[include = "*.svg"]
pub struct RegionAssets;

impl RegionAssets {
    /// 拼接某个地区旗帜的资源路径，例如 `AD` → `flags/ad.svg`。
    pub fn flag_path(flag: &str) -> SharedString {
        SharedString::from(format!("{FLAG_DIR}/{flag}.svg"))
    }

    /// 读取区域旗帜 SVG 源码，找不到时返回 `None`。
    pub fn flag_svg(flag: &str) -> Option<Cow<'static, [u8]>> {
        Self::get(&format!("{flag}.svg")).map(|file| file.data)
    }
}

impl AssetSource for RegionAssets {
    fn load(&self, path: &str) -> Result<Option<Cow<'static, [u8]>>> {
        let Some(relative) = path.strip_prefix(FLAG_DIR) else {
            return Ok(None);
        };
        let relative = relative.trim_start_matches('/');
        if relative.is_empty() {
            return Ok(None);
        }

        Ok(Self::get(relative).map(|file| file.data))
    }

    fn list(&self, path: &str) -> Result<Vec<SharedString>> {
        let prefix = if path.is_empty() {
            String::new()
        } else {
            format!("{}/", path.trim_end_matches('/'))
        };

        Ok(Self::iter()
            .filter(|name| name.starts_with(&prefix))
            .map(|name| SharedString::from(format!("{FLAG_DIR}/{name}")))
            .collect())
    }
}
