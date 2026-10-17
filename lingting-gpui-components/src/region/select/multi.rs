//! 地区多选。

use std::cell::RefCell;
use std::rc::Rc;

use gpui_kit::component::StyledExt as _;
use gpui_kit::component::combobox::{Combobox, ComboboxEvent, ComboboxState};
use gpui_kit::component::searchable_list::SearchableListDelegate as _;
use gpui_kit::component::{IndexPath, Sizable, Size};
use gpui_kit::prelude::FluentBuilder as _;
use gpui_kit::*;

use crate::region::select::delegate::{RegionDelegate, region_delegate, region_value};
use crate::region::select::listener::EventListener;

/// 值变化回调：按选择顺序给出 ISO 代码列表。
pub type RegionMultiSelectHandler = Rc<dyn Fn(&[&'static str], &mut Window, &mut App)>;

type HandlerCell = Rc<RefCell<Option<RegionMultiSelectHandler>>>;

/// 地区多选。
///
/// 走 GPUI Component 的 `Combobox`（多选模式）：点选一项不会关闭下拉，直到按 Esc 或点空白处。
/// 同一个 `id` 在多次渲染之间复用同一份状态，因此 `id` 必须在同一层级唯一。
///
/// `value` 是受控值，每次渲染都会同步进下拉状态；选择变化时通过
/// [`RegionMultiSelect::on_change`] 交回全部已选 ISO 代码。
#[derive(IntoElement)]
pub struct RegionMultiSelect {
    id: ElementId,
    value: Vec<String>,
    placeholder: Option<SharedString>,
    search_placeholder: Option<SharedString>,
    cleanable: bool,
    disabled: bool,
    style: StyleRefinement,
    handler: HandlerCell,
}

impl RegionMultiSelect {
    /// 创建多选。`id` 用于稳定下拉状态。
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            value: Vec::new(),
            placeholder: None,
            search_placeholder: None,
            cleanable: true,
            disabled: false,
            style: StyleRefinement::default(),
            handler: Rc::new(RefCell::new(None)),
        }
    }

    /// 设置受控值（ISO 代码列表，大小写不敏感）。
    pub fn value<I, S>(mut self, value: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.value = value.into_iter().map(Into::into).collect();
        self
    }

    /// 设置未选时的占位文案。
    pub fn placeholder(mut self, placeholder: impl Into<SharedString>) -> Self {
        self.placeholder = Some(placeholder.into());
        self
    }

    /// 设置搜索框占位文案。
    pub fn search_placeholder(mut self, placeholder: impl Into<SharedString>) -> Self {
        self.search_placeholder = Some(placeholder.into());
        self
    }

    /// 是否显示清空按钮，默认 `true`。
    pub fn cleanable(mut self, cleanable: bool) -> Self {
        self.cleanable = cleanable;
        self
    }

    /// 是否禁用。
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// 值变化回调。
    pub fn on_change(
        self,
        handler: impl Fn(&[&'static str], &mut Window, &mut App) + 'static,
    ) -> Self {
        *self.handler.borrow_mut() = Some(Rc::new(handler));
        self
    }
}

impl Styled for RegionMultiSelect {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for RegionMultiSelect {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let Self {
            id,
            value,
            placeholder,
            search_placeholder,
            cleanable,
            disabled,
            style,
            handler,
        } = self;

        let requested: Vec<&'static str> =
            value.iter().filter_map(|item| region_value(item)).collect();

        let state = window.use_keyed_state((id.clone(), "state"), cx, |window, cx| {
            ComboboxState::new(region_delegate(), positions_of(&requested), window, cx)
                .multiple(true)
                .searchable(true)
        });

        let listener = window.use_keyed_state((id.clone(), "listener"), cx, |window, cx| {
            let subscription = cx.subscribe_in(
                &state,
                window,
                |this: &mut EventListener<RegionMultiSelectHandler>,
                 _,
                 event: &ComboboxEvent<RegionDelegate>,
                 window,
                 cx| {
                    if let ComboboxEvent::Change(values) = event {
                        let handler = this.handler.borrow().clone();
                        if let Some(handler) = handler {
                            handler(values, window, cx);
                        }
                    }
                },
            );

            EventListener::new(handler.clone(), subscription)
        });
        listener.update(cx, |this, _| this.set_handler(handler));

        // 受控值同步：外部改了 value 就写回下拉状态。
        let current = state.read(cx).selected_values();
        if current != requested {
            state.update(cx, |state, cx| {
                state.set_selected_values(&requested, window, cx);
                cx.notify();
            });
        }

        div()
            .w_full()
            .refine_style(&style)
            .child(
                Combobox::new(&state)
                    .cleanable(cleanable)
                    .disabled(disabled)
                    .when_some(placeholder, |this, placeholder| {
                        this.placeholder(placeholder)
                    })
                    .when_some(search_placeholder, |this, placeholder| {
                        this.search_placeholder(placeholder)
                    })
                    .with_size(Size::Medium),
            )
            .into_any_element()
    }
}

/// 把 ISO 代码列表换成数据源里的下标。
fn positions_of(values: &[&'static str]) -> Vec<IndexPath> {
    let delegate: RegionDelegate = region_delegate();
    values
        .iter()
        .filter_map(|value| delegate.position(value))
        .collect()
}
