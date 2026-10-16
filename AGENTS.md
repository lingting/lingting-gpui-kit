# lingting-gpui-kit

GPUI Kit 的分层封装库。依赖方向：`lingting-gpui-core` 不依赖任何内部 crate，
`lingting-gpui-nav` → `gpui-kit` + `core`，`lingting-gpui-components` → `lingting-gpui-nav` + `core`，
`lingting-gpui-layout` → `lingting-gpui-components` / `lingting-gpui-nav` + `core`，
`lingting-gpui-kit` → 其余四个 crate 且只做统一再导出。

## 文档规则

说明文档统一放在仓库根目录的 `doc/` 下，不散落在各 crate 内。

- `doc/readme.md` 是文档索引。新增、删除、重命名或移动任何文档时，必须同步更新该索引。
- `doc/components/` 放组件文档：`lingting-gpui-components/src/` 下每个文件夹对应一个文档，文件名与文件夹同名，
  例如 `src/title_bar/` 对应 `doc/components/title_bar.md`。文档说明该模块提供的内容、对外 API、使用方式与取舍。
- `doc/layout/` 放布局文档，`doc/nav/` 放导航文档，组织方式与 `doc/components/` 相同。
- 某个 crate 内没有子文件夹时，用该目录下的 `readme.md` 作为这个 crate 的文档。
- 文档只描述当前已实现的能力，不写尚未实现的计划内容；代码变更后若文档不再准确，先记录，待确认后再同步。

## 长度单位规则

全库统一使用 **`Rems`**（`rems()`）作为自绘 UI 的长度单位。理由：GPUI 的 rem 以
`window.rem_size()`（由 `gpui-component` 的 Root 设为 `theme.font_size`）为基准，与 `gap_2`、
`px_3`、`text_sm`、`rounded_lg` 等缩放助手同源；主题字号一变，字号、间距、控件尺寸、图标一起缩放。

- 新增的尺寸、内边距、间距、圆角、图标几何一律用 `rems()`，不要用 `px()`。
- 公开 API 的长度参数用 `impl Into<Rems>`，内部字段与常量用 `Rems`；只有确实需要同时接受 rem 与 px
  的组件（如 `Menu`）才改用 `AbsoluteLength`，见文末的 `Menu` 说明。
- 需要 0 时同样写 `rems(0.)`，不要写 `px(0.)`。
- 与既有 rem 数值保持同一比例（1rem = 16px 基准），例如 32px 写作 `rems(2.)`、14px 写作 `rems(0.875)`。

### 例外：只能用 `Pixels` 的三类位置

以下位置必须继续用 `px()` / `Pixels`，并且要写注释说明为什么脱离缩放：

1. **GPUI 强制 `Pixels` 的类型**：`BoxShadow` 的 `offset` / `blur_radius` / `spread_radius`。
2. **系统窗口装饰基准层**：`lingting-gpui-components/src/title_bar/` 全部（`TITLE_BAR_HEIGHT`、
   `TITLE_BAR_GAP`、`WindowsControls` 的宽度与图标尺寸）、`lingting-gpui-components/src/menu/constants.rs`
   的 `TITLE_BAR_ITEM_HEIGHT` / `TITLE_BAR_ICON_SIZE` / `TITLE_BAR_TEXT_SIZE`、
   `lingting-gpui-kit/src/window_options_builder/traffic_light.rs`、
   以及两个布局的 `title_bar_height` 与 `desktop_top_layout` 的 `MENU_GAP`。
   这一层要和系统摆放的窗口控制按钮、红绿灯对齐，跟随字号缩放会错位。
3. **平台物理量**：`window_options_builder` 的窗口 `size` / `size_min`（`Size<DevicePixels>`）、
   屏幕 `visible_bounds` 与 `scale_factor`（`f32`）、`titlebar.traffic_light_position`（GPUI 要求 `Point<Pixels>`）。

`Menu` 的长度参数收 `AbsoluteLength`，rem 与 px 都能传，因此同一个组件覆盖两种场景：

- 默认值是 rem（`ITEM_HEIGHT` / `ICON_SIZE` / `TEXT_SIZE`），用于侧栏这类跟随字号缩放的自绘容器；
- 放进标题栏时改传 `constants::TITLE_BAR_ITEM_HEIGHT` / `TITLE_BAR_ICON_SIZE` / `TITLE_BAR_TEXT_SIZE`
  这三个 px 常量，与系统窗口装饰对齐（理由见上面例外 2）。
