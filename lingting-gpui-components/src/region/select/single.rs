//! 地区单选。

use std::cell::RefCell;
use std::rc::Rc;

use gpui_kit::component::StyledExt as _;
use gpui_kit::component::searchable_list::SearchableListDelegate as _;
use gpui_kit::component::select::{Select, SelectEvent, SelectState};
use gpui_kit::component::{IndexPath, Sizable, Size};
use gpui_kit::prelude::FluentBuilder as _;
use gpui_kit::*;

use crate::region::name::RegionNameLanguage;
use crate::region::select::delegate::{RegionDelegate, region_delegate, region_value};
use crate::region::select::listener::EventListener;

/// 值变化回调：`None` 表示清空。
pub type RegionSelectHandler = Rc<dyn Fn(Option<&'static str>, &mut Window, &mut App)>;

type HandlerCell = Rc<RefCell<Option<RegionSelectHandler>>>;

/// 地区单选。
///
/// 内部持有一个 `Select` 状态，同一个 `id` 在多次渲染之间复用，因此 `id` 必须在同一层级
/// 唯一。`value` 是受控值，每次渲染都会同步进下拉状态；用户确认或清空时通过
/// [`RegionSelect::on_change`] 把新的 ISO 代码交回调用方。
///
/// 已选展示与下拉列表行都按 [`RegionSelect::language`] 渲染地区名（默认中文），
/// 是否显示旗帜由 [`RegionSelect::flag`] 控制（默认显示）。
/// 搜索、清空、空状态都由 GPUI Component 的 `Select` 提供。
#[derive(IntoElement)]
pub struct RegionSelect {
    id: ElementId,
    value: Option<String>,
    language: RegionNameLanguage,
    flag: bool,
    placeholder: Option<SharedString>,
    search_placeholder: Option<SharedString>,
    cleanable: bool,
    disabled: bool,
    style: StyleRefinement,
    handler: HandlerCell,
}

impl RegionSelect {
    /// 创建单选。`id` 用于稳定下拉状态。
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            value: None,
            language: RegionNameLanguage::default(),
            flag: true,
            placeholder: None,
            search_placeholder: None,
            cleanable: true,
            disabled: false,
            style: StyleRefinement::default(),
            handler: Rc::new(RefCell::new(None)),
        }
    }

    /// 设置受控值（ISO 代码，大小写不敏感）；不设置表示未选。
    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    /// 设置地区名的文本语言，默认中文。
    pub fn language(mut self, language: RegionNameLanguage) -> Self {
        self.language = language;
        self
    }

    /// 已选展示是否显示旗帜，默认 `true`。
    pub fn flag(mut self, flag: bool) -> Self {
        self.flag = flag;
        self
    }

    /// 设置未选时的占位文案。
    pub fn placeholder(mut self, placeholder: impl Into<SharedString>) -> Self {
        self.placeholder = Some(placeholder.into());
        self
    }

    /// 设置搜索框占位文案。
    pub fn search_placeholder(mut self, search_placeholder: impl Into<SharedString>) -> Self {
        self.search_placeholder = Some(search_placeholder.into());
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
        handler: impl Fn(Option<&'static str>, &mut Window, &mut App) + 'static,
    ) -> Self {
        *self.handler.borrow_mut() = Some(Rc::new(handler));
        self
    }
}

impl Styled for RegionSelect {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for RegionSelect {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let Self {
            id,
            value,
            language,
            flag,
            placeholder,
            search_placeholder,
            cleanable,
            disabled,
            style,
            handler,
        } = self;

        let requested = value.as_deref().and_then(region_value);
        // 语言或旗帜变了就换一份下拉状态，避免把旧语言的展示留在状态里。
        let variant = format!("{}-{flag}", language.key());

        let state = window.use_keyed_state((id.clone(), format!("state-{variant}")), cx, {
            move |window, cx| {
                SelectState::new(
                    region_delegate(language, flag),
                    requested.and_then(|iso| position_of(iso, language, flag)),
                    window,
                    cx,
                )
            }
        });

        let listener = window.use_keyed_state((id.clone(), format!("listener-{variant}")), cx, {
            let state = state.clone();
            let handler = handler.clone();
            move |window, cx| {
                let subscription = cx.subscribe_in(
                    &state,
                    window,
                    |this: &mut EventListener<RegionSelectHandler>,
                     _,
                     event: &SelectEvent<RegionDelegate>,
                     window,
                     cx| {
                        let SelectEvent::Confirm(value) = event;
                        let handler = this.handler.borrow().clone();
                        if let Some(handler) = handler {
                            handler(*value, window, cx);
                        }
                    },
                );

                EventListener::new(handler, subscription)
            }
        });
        listener.update(cx, |this, _| this.set_handler(handler));

        // 受控值同步：外部改了 value 就写回下拉状态。
        let current = state.read(cx).selected_value().copied();
        if current != requested {
            state.update(cx, |state, cx| {
                match requested {
                    Some(iso) => state.set_selected_value(&iso, window, cx),
                    None => state.set_selected_index(None, window, cx),
                }
                cx.notify();
            });
        }

        div()
            .w_full()
            .refine_style(&style)
            .child(
                Select::new(&state)
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

/// 把 ISO 代码换成数据源里的下标。
fn position_of(iso: &'static str, language: RegionNameLanguage, flag: bool) -> Option<IndexPath> {
    region_delegate(language, flag).position(&iso)
}
