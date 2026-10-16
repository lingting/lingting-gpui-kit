use gpui_kit::Rems;

/// `text_xs`，0.75rem（12px 基准）。
///
/// 字号档（本项到 [`REM_3XL`]）与 GPUI 的 `text_xs` / `text_sm` / `text_base` /
/// `text_lg` / `text_xl` / `text_2xl` / `text_3xl` 一一对应。
pub const REM_XS: Rems = gpui_kit::rems(0.75);

/// `text_sm`，0.875rem（14px 基准）。
pub const REM_SM: Rems = gpui_kit::rems(0.875);

/// `text_base`，1rem（16px 基准）。
pub const REM_BASE: Rems = gpui_kit::rems(1.);

/// `text_lg`，1.125rem（18px 基准）。
pub const REM_LG: Rems = gpui_kit::rems(1.125);

/// `text_xl`，1.25rem（20px 基准）。
pub const REM_XL: Rems = gpui_kit::rems(1.25);

/// `text_2xl`，1.5rem（24px 基准）。
pub const REM_2XL: Rems = gpui_kit::rems(1.5);

/// `text_3xl`，1.875rem（30px 基准）。
pub const REM_3XL: Rems = gpui_kit::rems(1.875);

/// 0.5rem（8px 基准），常用于小内边距与紧凑间隔。
pub const REM_0P5: Rems = gpui_kit::rems(0.5);

/// 1.5rem（24px 基准），常用于中等内边距。
pub const REM_1P5: Rems = gpui_kit::rems(1.5);

/// 2rem（32px 基准），常用于控件高度与段间距。
pub const REM_2: Rems = gpui_kit::rems(2.);

/// 3rem（48px 基准），常用于大块间隔。
pub const REM_3: Rems = gpui_kit::rems(3.);

/// 4rem（64px 基准），常用于列表项高度与页面级留白。
pub const REM_4: Rems = gpui_kit::rems(4.);

/// [`REM_XS`] 的函数形式。
pub const fn rem_xs() -> Rems {
    REM_XS
}

/// [`REM_SM`] 的函数形式。
pub const fn rem_sm() -> Rems {
    REM_SM
}

/// [`REM_BASE`] 的函数形式。
pub const fn rem_base() -> Rems {
    REM_BASE
}

/// [`REM_LG`] 的函数形式。
pub const fn rem_lg() -> Rems {
    REM_LG
}

/// [`REM_XL`] 的函数形式。
pub const fn rem_xl() -> Rems {
    REM_XL
}

/// [`REM_2XL`] 的函数形式。
pub const fn rem_2xl() -> Rems {
    REM_2XL
}

/// [`REM_3XL`] 的函数形式。
pub const fn rem_3xl() -> Rems {
    REM_3XL
}

/// [`REM_0P5`] 的函数形式。
pub const fn rem_0p5() -> Rems {
    REM_0P5
}

/// [`REM_1P5`] 的函数形式。
pub const fn rem_1p5() -> Rems {
    REM_1P5
}

/// [`REM_2`] 的函数形式。
pub const fn rem_2() -> Rems {
    REM_2
}

/// [`REM_3`] 的函数形式。
pub const fn rem_3() -> Rems {
    REM_3
}

/// [`REM_4`] 的函数形式。
pub const fn rem_4() -> Rems {
    REM_4
}
