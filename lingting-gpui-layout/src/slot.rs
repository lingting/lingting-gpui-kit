use std::rc::Rc;

use gpui_kit::*;

/// 一段内容的渲染闭包。
///
/// 布局是每帧重渲染的视图，槽位不能只接受 `AnyElement`（它不可克隆），
/// 因此透传内容以闭包形式保存，每帧求值一次。需要保留状态的内容应在闭包内返回
/// 已创建实体的克隆。
pub type SlotRenderer = Rc<dyn Fn(&mut Window, &mut App) -> AnyElement>;
