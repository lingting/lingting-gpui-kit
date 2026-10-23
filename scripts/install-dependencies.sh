#!/usr/bin/env bash
#
# 安装 Linux 上编译本仓库需要的系统依赖。
#
# 用法:
#   bash scripts/install-dependencies.sh
#
# 依赖来源（不是猜的）:
#   - 基础清单取自 gpui-kit 官方仓库 longbridge/gpui-kit 的 script/install-linux.sh
#   - 本项目启用了 gpui-kit 的 `gpui-fast` 特性, 额外走 gpui-fast-linux ->
#     gpui-fast-wgpu -> zed-font-kit 这条链, 其中 freetype-sys 与
#     yeslogic-fontconfig-sys 的 build.rs 用 pkg-config 找系统库, 因此额外需要
#     libfreetype-dev 与 pkg-config。
#   - libwebkit2gtk 已移除: 本项目没有使用 gpui-webview。

set -euo pipefail

if [[ "${OSTYPE:-}" != linux-gnu* && "$(uname -s)" != "Linux" ]]; then
    echo "非 Linux 平台, 跳过系统依赖安装。" >&2
    exit 0
fi

SUDO=""
if [[ "$(id -u)" -ne 0 ]]; then
    SUDO="sudo"
fi

echo "==> 更新 apt 索引"
${SUDO} apt-get update

echo "==> 安装编译依赖"
${SUDO} apt-get install -y \
    gcc g++ clang pkg-config \
    libfontconfig-dev libfreetype-dev libwayland-dev \
    libxkbcommon-x11-dev libx11-xcb-dev \
    libssl-dev libzstd-dev libasound2-dev \
    vulkan-validationlayers libvulkan1

echo "==> 依赖安装完成"
