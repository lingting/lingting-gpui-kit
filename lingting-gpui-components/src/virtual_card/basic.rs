use std::rc::Rc;

use gpui_kit::component::StyledExt as _;
use gpui_kit::prelude::FluentBuilder as _;
use gpui_kit::*;

use crate::card::Card;
use crate::virtual_list::{FixedVirtualList, ItemFilter, ItemRenderer};

/// 虚拟滚动卡片。
#[derive(IntoElement)]
pub struct VirtualCard<T: 'static> {
    card: Card,
    id: ElementId,
    items: Rc<Vec<T>>,
    item_height: Rems,
    filter: Option<ItemFilter<T>>,
    item_renderer: Option<ItemRenderer<T>>,
    empty: Option<AnyElement>,
    style: StyleRefinement,
}

impl<T: 'static> VirtualCard<T> {
    /// 创建卡片。`id` 用于列表滚动状态的稳定标识。
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            card: Card::new(),
            id: id.into(),
            items: Rc::new(Vec::new()),
            item_height: rems(4.),
            filter: None,
            item_renderer: None,
            empty: None,
            style: StyleRefinement::default(),
        }
    }

    /// 设置 ghost 模式，透传给 [`Card`]。
    pub fn ghost(mut self, ghost: bool) -> Self {
        self.card = self.card.ghost(ghost);
        self
    }

    /// 启用 ghost 模式，等价于 `ghost(true)`。
    pub fn with_ghost(self) -> Self {
        self.ghost(true)
    }

    /// 设置标题。
    pub fn title(mut self, title: impl IntoElement) -> Self {
        self.card = self.card.title(title);
        self
    }

    /// 设置数据。列表只持有数据引用，不会克隆每一条子项。
    pub fn items(mut self, items: Rc<Vec<T>>) -> Self {
        self.items = items;
        self
    }

    /// 设置子项高度。
    pub fn item_height(mut self, item_height: Rems) -> Self {
        self.item_height = item_height;
        self
    }

    /// 设置筛选条件，未命中任何子项时显示空状态。
    pub fn filter(mut self, filter: ItemFilter<T>) -> Self {
        self.filter = Some(filter);
        self
    }

    /// 设置子项渲染器。
    pub fn item_renderer(mut self, item_renderer: ItemRenderer<T>) -> Self {
        self.item_renderer = Some(item_renderer);
        self
    }

    /// 设置空状态。
    pub fn empty(mut self, empty: impl IntoElement) -> Self {
        self.empty = Some(empty.into_any_element());
        self
    }
}

impl<T: 'static> Styled for VirtualCard<T> {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl<T: 'static> RenderOnce for VirtualCard<T> {
    fn render(self, _: &mut Window, _cx: &mut App) -> impl IntoElement {
        let Self {
            card,
            id,
            items,
            item_height,
            filter,
            item_renderer,
            empty,
            style,
        } = self;

        card.child(
            FixedVirtualList::new(id)
                .items(items)
                .item_height(item_height)
                .when_some(filter, |this, filter| this.filter(filter))
                .when_some(item_renderer, |this, renderer| this.item_renderer(renderer))
                .when_some(empty, |this, empty| this.empty(empty))
                .refine_style(&style),
        )
    }
}
