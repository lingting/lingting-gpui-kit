//! 测试共用的通知类型与页面。

#![allow(dead_code, unused_imports)]

mod events;
mod page;

pub use events::{InstallFinished, TaskChanged};
pub use page::{Page, always_foreground, hits, install};
