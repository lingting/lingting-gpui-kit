//! 在 `Dialog` 之上补一层内边距的对话框。
//!
//! 模块按职责拆分：`basic` 放结构体、常量、与 `Dialog` 的互相转换及元素实现，
//! `builder` 放转发 `Dialog` 的构建方法。

mod basic;
mod builder;

pub use basic::ExDialog;
