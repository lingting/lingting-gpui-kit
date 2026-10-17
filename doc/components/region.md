# region

国家与地区：静态数据查询、区域旗帜、标签、单选与多选。

数据全部来自 `framework-region` 的构建期静态表（`regions` / `phones` / `m49`），本模块不复制任何数据，
只额外维护一份 ISO 代码索引；`RegionItem` 只持有 `&'static Region`，克隆不产生拷贝。

## 数据查询

| 函数 | 说明 |
| --- | --- |
| `find_region(&str) -> Option<&'static Region>` | 按 ISO 代码查地区，大小写与首尾空白不敏感 |
| `filter_regions(&str) -> Vec<&'static Region>` | 输入为空返回全部，否则按 `region_matches` 过滤 |
| `region_matches(&Region, &str) -> bool` | 命中 ISO / ISO3、国际区号、号码前缀、中英文名；英文名忽略大小写 |
| `normalize_region_value(&str) -> String` | 去首尾空白并转大写 |

```rust
let region = find_region("cn").unwrap();
assert_eq!(region.name.zh, "中国");
```

## RegionFlag

区域旗帜元素，默认 1.25rem × 1rem、圆角 0.125rem；`width` / `height` 可覆盖，也可用 `Styled` 追加样式。
地区代码未知时不渲染任何内容。

```rust
RegionFlag::new("CN")
RegionFlag::new("CN").width(rems(2.)).height(rems(1.5))
```

GPUI 的 `svg()` 把 SVG 当单色遮罩绘制（只保留 alpha、用文字颜色填充），彩色旗帜会被涂成一整块色块。
因此这里改用 `img()`：SVG 交给 GPUI 的图片管线按原色栅格化，结果按资源路径缓存在进程内，
同一面旗帜只解析一次。代价是首次渲染是异步的，图片就绪前该位置为空。

## RegionAssets

246 个区域旗帜 SVG 内嵌在本 crate 的 `assets/flags` 下（`rust-embed`），资源路径固定为 `flags/<flag>.svg`，
`<flag>` 取自 `Region::flag`。应用必须把它合并进自己的 `AssetSource`，否则旗帜画不出来：

```rust
impl gpui::AssetSource for AppAssets {
    fn load(&self, path: &str) -> gpui::Result<Option<Cow<'static, [u8]>>> {
        if let Some(bytes) = RegionAssets::load(path)? {
            return Ok(Some(bytes));
        }
        gpui_kit::assets::Assets.load(path)
    }

    fn list(&self, path: &str) -> gpui::Result<Vec<gpui::SharedString>> {
        let mut paths = gpui_kit::assets::Assets.list(path)?;
        paths.extend(RegionAssets::list(path)?);
        paths.sort();
        paths.dedup();
        Ok(paths)
    }
}
```

`RegionAssets::flag_path(flag)` 返回拼好的资源路径，`RegionAssets::flag_svg(flag)` 直接返回 SVG 字节。

## RegionTag

「旗帜 + 中文名」的一行标签，间距 0.25rem。用 `render_with` 换成自己的内容；地区代码未知时不渲染。

```rust
RegionTag::new("CN")
RegionTag::new("CN").render_with(|region| {
    div().child(format!("+{} {}", region.calling_codes[0], region.name.zh)).into_any_element()
})
```

## RegionSelect / RegionMultiSelect

两个受控下拉，状态按 `id` 存在元素状态里，因此 `id` 必须在同一层级唯一。

- `RegionSelect`：单选，走 GPUI Component 的 `Select`；
- `RegionMultiSelect`：多选，走 `Combobox`（点选不关闭下拉，按 Esc 或点空白处收起）。

参考实现用一个 `multiple` 开关切换两种模式；`Select` 本身只支持单选，多选必须走 `Combobox`，
两种语义塞进同一个状态更难用，所以这里拆成两个类型。

| 方法 | 默认值 | 说明 |
| --- | --- | --- |
| `new(impl Into<ElementId>)` | — | `id` 用于稳定下拉状态 |
| `value(...)` | 空 | 受控值：单选收单个 ISO 代码，多选收 ISO 代码列表 |
| `placeholder(...)` | 组件内置 | 未选时的占位文案 |
| `search_placeholder(...)` | 组件内置 | 搜索框占位文案 |
| `cleanable(bool)` | `true` | 是否显示清空按钮 |
| `disabled(bool)` | `false` | 是否禁用 |
| `on_change(...)` | 无 | 单选回调 `Option<&'static str>`；多选回调 `&[&'static str]` |

```rust
RegionSelect::new("region")
    .value(state.region.clone())
    .on_change(cx.listener(|this, value, _, cx| {
        this.region = value.map(str::to_string);
        cx.notify();
    }))

RegionMultiSelect::new("regions")
    .value(state.regions.clone())
    .on_change(cx.listener(|this, values, _, cx| {
        this.regions = values.iter().map(|v| v.to_string()).collect();
        cx.notify();
    }))
```

`value` 每次渲染都会写回下拉状态，因此外部改值可以直接生效；未知代码会被忽略。

## 注意点

- 旗帜依赖应用侧合并 `RegionAssets`，这是唯一必须由调用方完成的接入步骤。
- 下拉列表是全量地区（约 250 行），搜索走 `region_matches`，不做远程加载。
- `RegionItem` / `RegionDelegate` / `region_delegate` / `region_value` 是给自定义下拉或列表复用的，
  默认组件内部已经接好，一般不需要直接使用。
