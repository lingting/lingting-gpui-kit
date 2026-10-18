//! 地区多选。

use std::cell::RefCell;
use std::rc::Rc;

use gpui_kit::component::StyledExt as _;
use gpui_kit::component::button::{Button, ButtonVariants as _};
use gpui_kit::component::combobox::{
    Combobox, ComboboxEvent, ComboboxState, ComboboxTriggerContext,
};
use gpui_kit::component::searchable_list::SearchableListDelegate as _;
use gpui_kit::component::select::Caret;
use gpui_kit::component::{
    ActiveTheme as _, Disableable as _, Icon, IconName, IndexPath, Sizable, Size,
};
use gpui_kit::prelude::FluentBuilder as _;
use gpui_kit::*;

use crate::region::name::RegionNameLanguage;
use crate::region::select::delegate::{RegionDelegate, RegionItem, region_delegate, region_value};
use crate::region::select::listener::EventListener;

/// 已选展示里多项之间的间距，0.25rem（`gap-1`）。
const TRIGGER_GAP: Rems = rems(0.25);

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
///
/// 已选展示与下拉列表行都按 [`RegionMultiSelect::language`] 渲染地区名（默认中文），
/// 是否显示旗帜由 [`RegionMultiSelect::flag`] 控制（默认显示）。`Combobox` 自带的已选展示
/// 只拼纯文本标题，这里换成自绘的一行，因此清空按钮与箭头也一并自绘。
#[derive(IntoElement)]
pub struct RegionMultiSelect {
    id: ElementId,
    value: Vec<String>,
    language: RegionNameLanguage,
    flag: bool,
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

    /// 设置受控值（ISO 代码列表，大小写不敏感）。
    pub fn value<I, S>(mut self, value: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.value = value.into_iter().map(Into::into).collect();
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
            language,
            flag,
            placeholder,
            search_placeholder,
            cleanable,
            disabled,
            style,
            handler,
        } = self;

        let requested: Vec<&'static str> =
            value.iter().filter_map(|item| region_value(item)).collect();
        // 语言或旗帜变了就换一份下拉状态，避免把旧语言的展示留在状态里。
        let variant = format!("{}-{flag}", language.key());

        let state = window.use_keyed_state((id.clone(), format!("state-{variant}")), cx, {
            let requested = requested.clone();
            move |window, cx| {
                ComboboxState::new(
                    region_delegate(language, flag),
                    positions_of(&requested, language, flag),
                    window,
                    cx,
                )
                .multiple(true)
                .searchable(true)
            }
        });

        let listener = window.use_keyed_state((id.clone(), format!("listener-{variant}")), cx, {
            let state = state.clone();
            let handler = handler.clone();
            move |window, cx| {
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

                EventListener::new(handler, subscription)
            }
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

        let trigger_state = state.clone();
        let trigger = move |trigger: &ComboboxTriggerContext<RegionDelegate>,
                            _: &mut Window,
                            cx: &mut App| {
            let selection = trigger.selection();
            let placeholder = trigger.placeholder().cloned();

            let body = if selection.is_empty() {
                div()
                    .text_color(cx.theme().muted_foreground)
                    .child(placeholder.unwrap_or_else(|| "请选择".into()))
                    .into_any_element()
            } else {
                selected_row(selection)
            };

            let trailing = if cleanable && !selection.is_empty() {
                let state = trigger_state.clone();
                Button::new("region-multi-clean")
                    .icon(Icon::new(IconName::Close))
                    .text()
                    .xsmall()
                    .tab_stop(false)
                    .disabled(trigger.is_disabled())
                    .on_click(move |_, _, cx| {
                        cx.stop_propagation();
                        state.update(cx, |state, cx| state.clear_selection(cx));
                    })
                    .into_any_element()
            } else {
                Caret::new(trigger.size())
                    .text_color(cx.theme().muted_foreground)
                    .into_any_element()
            };

            div()
                .w_full()
                .min_w_0()
                .flex()
                .items_center()
                .justify_between()
                .gap(TRIGGER_GAP)
                .child(
                    div()
                        .flex_1()
                        .min_w_0()
                        .overflow_hidden()
                        .whitespace_nowrap()
                        .child(body),
                )
                .child(trailing)
                .into_any_element()
        };

        div()
            .w_full()
            .refine_style(&style)
            .child(
                Combobox::new(&state)
                    .disabled(disabled)
                    .when_some(placeholder, |this, placeholder| {
                        this.placeholder(placeholder)
                    })
                    .when_some(search_placeholder, |this, placeholder| {
                        this.search_placeholder(placeholder)
                    })
                    .render_trigger(trigger)
                    .with_size(Size::Medium),
            )
            .into_any_element()
    }
}

/// 已选地区的一行：旗帜（可选）+ 地区名，多项之间用顿号分隔。
fn selected_row(selection: &[(IndexPath, RegionItem)]) -> AnyElement {
    let mut row = div()
        .flex()
        .flex_row()
        .items_center()
        .gap(TRIGGER_GAP)
        .overflow_hidden()
        .whitespace_nowrap();

    for (ix, (_, item)) in selection.iter().enumerate() {
        if ix > 0 {
            row = row.child("、");
        }
        row = row.child(item.trigger_row());
    }

    row.into_any_element()
}

/// 把 ISO 代码列表换成数据源里的下标。
fn positions_of(
    values: &[&'static str],
    language: RegionNameLanguage,
    flag: bool,
) -> Vec<IndexPath> {
    let delegate = region_delegate(language, flag);
    values
        .iter()
        .filter_map(|value| delegate.position(value))
        .collect()
}
