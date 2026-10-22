use std::rc::Rc;

use gpui_kit::base::Scrollbar;
use gpui_kit::component::ActiveTheme as _;
use gpui_kit::component::StyledExt as _;
use gpui_kit::prelude::FluentBuilder as _;
use gpui_kit::*;

/// 筛选子项：返回 `true` 表示保留。
pub type ItemFilter<T> = Rc<dyn Fn(&T) -> bool>;

/// 渲染子项。
pub type ItemRenderer<T> = Rc<dyn Fn(&T, &mut Window, &mut App) -> AnyElement>;

/// 固定高度虚拟列表。
///
/// 每个子项占据的高度由 [`FixedVirtualList::item_height`] 指定，包含子项自身的内外边距；
/// 子项渲染结果必须在该高度内完成布局，超出部分会被裁掉。
#[derive(IntoElement)]
pub struct FixedVirtualList<T: 'static> {
    id: ElementId,
    items: Rc<Vec<T>>,
    item_height: Rems,
    filter: Option<ItemFilter<T>>,
    item_renderer: Option<ItemRenderer<T>>,
    empty: Option<AnyElement>,
    /// 调用方注入的滚动句柄，未注入时列表自建一个。
    scroll_handle: Option<UniformListScrollHandle>,
    style: StyleRefinement,
}

impl<T: 'static> FixedVirtualList<T> {
    /// 创建列表。`id` 同时用于滚动状态的稳定标识，同一页面内的多个列表必须不同。
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            items: Rc::new(Vec::new()),
            item_height: rems(4.),
            filter: None,
            item_renderer: None,
            empty: None,
            scroll_handle: None,
            style: StyleRefinement::default(),
        }
    }

    /// 注入外部滚动句柄，列表改用它的滚动位置。
    ///
    /// 默认由列表按 `id` 自行保存滚动状态；调用方需要主动滚动（例如保持在底部）时，
    /// 自己持有一个句柄并在这里注入，之后可以对它调用 `scroll_to_item`。
    pub fn scroll_handle(mut self, handle: UniformListScrollHandle) -> Self {
        self.scroll_handle = Some(handle);
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

impl<T: 'static> Styled for FixedVirtualList<T> {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl<T: 'static> RenderOnce for FixedVirtualList<T> {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let Self {
            id,
            items,
            item_height,
            filter,
            item_renderer,
            empty,
            scroll_handle,
            style,
        } = self;

        // 筛选项只记录原始下标，这样渲染时仍能拿到数据本身，也不需要克隆数据。
        let visible: Rc<Vec<usize>> = Rc::new(match &filter {
            Some(matches) => (0..items.len()).filter(|ix| matches(&items[*ix])).collect(),
            None => (0..items.len()).collect(),
        });
        let count = visible.len();
        let item_height = item_height.to_pixels(window.rem_size());

        // 滚动位置必须跨帧保留：调用方注入句柄时用它，否则存放在以列表 id 为键的元素状态里。
        let scroll_handle = match scroll_handle {
            Some(handle) => handle,
            None => window
                .use_keyed_state((id.clone(), "scroll"), cx, |_, _| {
                    UniformListScrollHandle::new()
                })
                .read(cx)
                .clone(),
        };
        let list = item_renderer.map(|item_renderer| {
            let items = items.clone();
            let visible = visible.clone();

            uniform_list(id, count, move |range, window, cx| {
                range
                    .map(|ix| {
                        let item = &items[visible[ix]];

                        div()
                            .w_full()
                            .h(item_height)
                            .flex_shrink_0()
                            .overflow_hidden()
                            // 行间距：子项高度包含它，去掉后带边框的列表项会贴在一起。
                            .pb_2()
                            .child(item_renderer(item, window, cx))
                    })
                    .collect::<Vec<_>>()
            })
            .track_scroll(&scroll_handle)
            .size_full()
        });

        div()
            .relative()
            .flex()
            .flex_col()
            .size_full()
            .min_h_0()
            .refine_style(&style)
            .when(count == 0, |this| {
                this.items_center()
                    .justify_center()
                    .text_sm()
                    .text_color(cx.theme().muted_foreground)
                    .child(empty.unwrap_or_else(|| "暂无数据".into_any_element()))
            })
            .when(count > 0, |this| {
                this.children(list)
                    // 滚动条浮在内容之上，贴列表右边缘：它不占布局宽度，
                    // 子项仍铺满列表宽度，因此滚动条只会盖住内容最右侧的一小条。
                    .child(Scrollbar::vertical(&scroll_handle))
            })
    }
}
