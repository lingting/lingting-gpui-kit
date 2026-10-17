# ex_dialog

在 `gpui-component` 的 `Dialog` 之上补一层内边距的对话框。

```text
┌──────────────────────────────┐
│ 标题区（Dialog 原有布局）       │
├──────────────────────────────┤
│ ┌──────────────────────────┐ │
│ │ p_1  用户内容             │ │
│ └──────────────────────────┘ │
│ ┌──────────────────────────┐ │
│ │ p_1  用户底部（按钮）      │ │
│ └──────────────────────────┘ │
└──────────────────────────────┘
```

`ExDialog` 内部只持有一个 [`Dialog`]，方法逐个转发，签名与 `Dialog` 完全一致；唯一的差别是
用户传入的内容与底部会各自被包一层 `p_1`（0.25rem）：

- `content`：在交给用户构建器的 `DialogContent` 上先补 `p_1`；
- `footer`：用户传入的底部包一层 `div().p_1()`。

目的是避免内容与底部完全贴到弹窗边缘，让投影、焦点环等需要溢出空间的样式不被裁掉。
没有调用 `content` / `footer` 时不产生任何包装，行为与 `Dialog` 一致。

## 方法

`ExDialog` 转发 `Dialog` 的全部公开构建方法，默认值与 `Dialog` 相同：

| 方法 | 默认值 | 说明 |
| --- | --- | --- |
| `new(cx: &mut App)` | — | 无标题、无内容、无底部 |
| `trigger(impl IntoElement)` | 无 | 点击该元素打开对话框 |
| `content(Fn(DialogContent, &mut Window, &mut App) -> DialogContent)` | 无 | 内容，**额外补 `p_1`** |
| `title(impl IntoElement)` | 无 | 标题 |
| `footer(impl IntoElement)` | 无 | 底部，**额外补 `p_1`** |
| `button_props(DialogButtonProps)` | 默认 | 只覆盖传入的字段 |
| `on_close(Fn(&ClickEvent, &mut Window, &mut App))` | 无 | 关闭后回调 |
| `on_ok(Fn(..) -> bool)` | 关闭对话框 | 返回 `false` 不关闭 |
| `on_cancel(Fn(..) -> bool)` | 关闭对话框 | 返回 `false` 不关闭 |
| `close_button(bool)` | `true` | 右上角关闭按钮 |
| `margin_top(impl Into<Pixels>)` | 视口高度的 1/10 | 顶部偏移 |
| `w(impl Into<Pixels>)` / `width(..)` | 448px | 宽度，不会超过视口减去两侧边距 |
| `max_w(impl Into<Pixels>)` | 无 | 最大宽度 |
| `overlay(bool)` | `true` | 遮罩 |
| `overlay_closable(bool)` | `true` | 点击遮罩关闭 |
| `keyboard(bool)` | `true` | `Esc` 关闭 |
| `entrance(DialogEntrance)` | `SlideDown` | 入场动画 |

此外与 `Dialog` 一样：`ExDialog` 是 `IntoElement`，也是 `ParentElement` 与 `Styled`，
因此 `child` / `children` 与样式链（`p_3`、`rounded_lg` 等）都能用，样式作用在弹窗表面上。

## 与 `Dialog` 互相转换

`window.open_dialog` 这类命令式 API 的闭包只接受 `Dialog`，用两个方法换回：

```rust
use gpui_kit::component::WindowExt as _;
use lingting_gpui_components::ex_dialog::ExDialog;

window.open_dialog(cx, |dialog, window, cx| {
    ExDialog::from(dialog)
        .title("标题")
        .footer(buttons)
        .into_dialog()
});
```

| 转换 | 说明 |
| --- | --- |
| `ExDialog::from(Dialog)` | `Dialog` 包成 `ExDialog` |
| `ExDialog::into_dialog()` | 取回内层的 `Dialog` |

没有 `From<ExDialog> for Dialog`：`Dialog` 属于 `gpui-component`，在本地 crate 实现该方向的
`From` 会违反孤儿规则，只能用 `into_dialog()`。

## 注意点

- `p_1` 加在 `DialogContent` 自身，`Dialog` 原本给内容容器设的左右内边距与 `gap` 仍在。
- 用户在 `content` 构建器里自己再设 `p_*` 会覆盖这层 `p_1`（`refine` 语义），属于显式覆盖。
- 底部包装只负责内边距，`DialogFooter` 的 `justify_end` 等布局仍由调用方决定。

