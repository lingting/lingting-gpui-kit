# lingting-gpui-kit

GPUI Kit 的分层封装库。依赖方向：`lingting-gpui-nav` → `gpui-kit`，
`lingting-gpui-components` → `lingting-gpui-nav`，`lingting-gpui-layout` → `lingting-gpui-components` /
`lingting-gpui-nav`，`lingting-gpui-kit` → 其余三个 crate 且只做统一再导出。

## 文档规则

说明文档统一放在仓库根目录的 `doc/` 下，不散落在各 crate 内。

- `doc/readme.md` 是文档索引。新增、删除、重命名或移动任何文档时，必须同步更新该索引。
- `doc/components/` 放组件文档：`lingting-gpui-components/src/` 下每个文件夹对应一个文档，文件名与文件夹同名，
  例如 `src/title_bar/` 对应 `doc/components/title_bar.md`。文档说明该模块提供的内容、对外 API、使用方式与取舍。
- `doc/layout/` 放布局文档，`doc/nav/` 放导航文档，组织方式与 `doc/components/` 相同。
- 某个 crate 内没有子文件夹时，用该目录下的 `readme.md` 作为这个 crate 的文档。
- 文档只描述当前已实现的能力，不写尚未实现的计划内容；代码变更后若文档不再准确，先记录，待确认后再同步。
