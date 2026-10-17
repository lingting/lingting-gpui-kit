//! 事件订阅的持有者。
//!
//! 元素本身不是实体，而 `cx.subscribe` 需要一个归属实体来存放订阅句柄。
//! 这里把它挂在以组件 id 为键的 keyed state 上，订阅只在首次渲染时建立，
//! 回调则每次渲染替换成最新的闭包。

use std::cell::RefCell;
use std::rc::Rc;

use gpui_kit::*;

/// 一份订阅加上最新的回调。
pub struct EventListener<E> {
    /// 当前回调；`None` 表示调用方没设回调。
    pub handler: Rc<RefCell<Option<E>>>,
    /// 订阅句柄，只在实体销毁时用于退订。
    _subscription: Subscription,
}

impl<E> EventListener<E> {
    /// 用已有订阅建立持有者。
    pub fn new(handler: Rc<RefCell<Option<E>>>, subscription: Subscription) -> Self {
        Self {
            handler,
            _subscription: subscription,
        }
    }

    /// 换成最新的回调。
    pub fn set_handler(&mut self, handler: Rc<RefCell<Option<E>>>) {
        self.handler = handler;
    }
}
