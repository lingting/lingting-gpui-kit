use gpui_kit::{Bounds, DevicePixels};

/// 主屏幕的物理边界与缩放因子。
#[derive(Clone, Copy, Debug)]
pub(crate) struct PrimaryScreen {
    /// 可见区域（排除任务栏等）的物理边界。
    pub visible_bounds: Bounds<DevicePixels>,
    /// 缩放因子，`1.0` 表示 100%。
    pub scale_factor: f32,
}

/// 查询主屏幕；平台未提供或查询失败时返回 `None`。
#[cfg(target_os = "windows")]
pub(crate) fn primary_screen() -> Option<PrimaryScreen> {
    use gpui_kit::{point, size};
    use windows_sys::Win32::Foundation::POINT;
    use windows_sys::Win32::Graphics::Gdi::{
        GetMonitorInfoW, MONITOR_DEFAULTTOPRIMARY, MONITORINFO, MonitorFromPoint,
    };
    use windows_sys::Win32::UI::HiDpi::{GetDpiForMonitor, MDT_EFFECTIVE_DPI};

    /// 100% 缩放对应的 DPI。
    const USER_DEFAULT_SCREEN_DPI: f32 = 96.;

    // SAFETY: 只读取主显示器的信息，所有参数都是栈上的值。
    unsafe {
        // 与 GPUI 一致：取主显示器。
        let monitor = MonitorFromPoint(POINT { x: 0, y: 0 }, MONITOR_DEFAULTTOPRIMARY);

        let mut info = MONITORINFO {
            cbSize: std::mem::size_of::<MONITORINFO>() as u32,
            ..Default::default()
        };
        if GetMonitorInfoW(monitor, &mut info) == 0 {
            return None;
        }

        let mut dpi_x = 0;
        let mut dpi_y = 0;
        // HRESULT 小于 0 表示失败；DPI 为 0 说明系统没有返回有效值。
        let dpi_ok = GetDpiForMonitor(monitor, MDT_EFFECTIVE_DPI, &mut dpi_x, &mut dpi_y) >= 0;
        let scale_factor = if dpi_ok && dpi_x > 0 {
            dpi_x as f32 / USER_DEFAULT_SCREEN_DPI
        } else {
            1.0
        };

        let work_area = info.rcWork;
        let visible_size = size(
            DevicePixels(work_area.right - work_area.left),
            DevicePixels(work_area.bottom - work_area.top),
        );
        // 空区域无法用于居中与尺寸判定，视作查询失败。
        if visible_size.width <= DevicePixels(0) || visible_size.height <= DevicePixels(0) {
            return None;
        }

        Some(PrimaryScreen {
            visible_bounds: Bounds::new(
                point(DevicePixels(work_area.left), DevicePixels(work_area.top)),
                visible_size,
            ),
            scale_factor,
        })
    }
}

/// 非 Windows 平台没有屏幕查询，返回 `None`。
#[cfg(not(target_os = "windows"))]
pub(crate) fn primary_screen() -> Option<PrimaryScreen> {
    None
}
