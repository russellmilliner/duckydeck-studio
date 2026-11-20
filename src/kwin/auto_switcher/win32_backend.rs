#[cfg(target_os = "windows")]
use super::{resolve_full_browser_caption, WindowChangeEvent};

#[cfg(target_os = "windows")]
pub fn is_windows_pid_duckydeck(pid: u32) -> bool {
    win32_window::is_pid_duckydeck(pid)
}

#[cfg(target_os = "windows")]
pub(super) mod win32_window {
    use std::ffi::OsString;
    use std::os::windows::ffi::OsStringExt;
    use std::path::Path;

    #[repr(C)]
    #[derive(Clone, Copy, Default)]
    pub struct POINT {
        pub x: i32,
        pub y: i32,
    }

    #[link(name = "user32")]
    extern "system" {
        pub fn GetForegroundWindow() -> isize;
        pub fn FindWindowExW(
            hWndParent: isize,
            hWndChildAfter: isize,
            lpszClass: *const u16,
            lpszWindow: *const u16,
        ) -> isize;
        pub fn SetWindowPos(
            hWnd: isize,
            hWndInsertAfter: isize,
            X: i32,
            Y: i32,
            cx: i32,
            cy: i32,
            uFlags: u32,
        ) -> i32;
        pub fn GetWindowTextW(hWnd: isize, lpString: *mut u16, nMaxCount: i32) -> i32;
        pub fn GetClassNameW(hWnd: isize, lpClassName: *mut u16, nMaxCount: i32) -> i32;
        pub fn GetWindowThreadProcessId(hWnd: isize, lpdwProcessId: *mut u32) -> u32;
        pub fn GetCursorPos(lpPoint: *mut POINT) -> i32;
        pub fn WindowFromPoint(Point: POINT) -> isize;
        pub fn GetAncestor(hwnd: isize, gaFlags: u32) -> isize;
        pub fn GetAsyncKeyState(vKey: i32) -> i16;
        pub fn GetWindowLongPtrW(hWnd: isize, nIndex: i32) -> isize;
        pub fn SetWindowLongPtrW(hWnd: isize, nIndex: i32, dwNewLong: isize) -> isize;
    }

    #[link(name = "kernel32")]
    extern "system" {
        pub fn OpenProcess(dwDesiredAccess: u32, bInheritHandle: i32, dwProcessId: u32) -> isize;
        pub fn QueryFullProcessImageNameW(
            hProcess: isize,
            dwFlags: u32,
            lpExeName: *mut u16,
            lpdwSize: *mut u32,
        ) -> i32;
        pub fn CloseHandle(hObject: isize) -> i32;
    }

    #[link(name = "advapi32")]
    extern "system" {
        pub fn RegOpenKeyExW(
            hKey: isize,
            lpSubKey: *const u16,
            ulOptions: u32,
            samDesired: u32,
            phkResult: *mut isize,
        ) -> i32;
        pub fn RegQueryValueExW(
            hKey: isize,
            lpValueName: *const u16,
            lpReserved: *const u32,
            lpType: *mut u32,
            lpData: *mut u8,
            lpcbData: *mut u32,
        ) -> i32;
        pub fn RegSetValueExW(
            hKey: isize,
            lpValueName: *const u16,
            Reserved: u32,
            dwType: u32,
            lpData: *const u8,
            cbData: u32,
        ) -> i32;
        pub fn RegDeleteValueW(hKey: isize, lpValueName: *const u16) -> i32;
        pub fn RegCloseKey(hKey: isize) -> i32;
    }

    pub const GA_ROOT: u32 = 2;
    pub const PROCESS_QUERY_LIMITED_INFORMATION: u32 = 0x1000;
    pub const VK_LBUTTON: i32 = 0x01;
    pub const VK_ESCAPE: i32 = 0x1B;
    const HKEY_CURRENT_USER: isize = -2147483647isize; // 0x80000001
    const KEY_READ: u32 = 0x20019;
    const KEY_SET_VALUE: u32 = 0x0002;
    const REG_SZ: u32 = 1;

    fn wide_null(s: &str) -> Vec<u16> {
        s.encode_utf16().chain(std::iter::once(0)).collect()
    }

    pub fn is_registry_autostart_enabled() -> bool {
        let subkey = wide_null("Software\\Microsoft\\Windows\\CurrentVersion\\Run");
        let val_name = wide_null("DuckyDeckStudio");
        // SAFETY: RegOpenKeyExW / RegQueryValueExW / RegCloseKey use valid null-terminated UTF-16 buffers.
        unsafe {
            let mut hkey: isize = 0;
            if RegOpenKeyExW(HKEY_CURRENT_USER, subkey.as_ptr(), 0, KEY_READ, &mut hkey) != 0 {
                return false;
            }
            let mut data_len: u32 = 0;
            let status = RegQueryValueExW(
                hkey,
                val_name.as_ptr(),
                std::ptr::null(),
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                &mut data_len,
            );
            RegCloseKey(hkey);
            status == 0 && data_len > 2
        }
    }

    pub fn set_registry_autostart_enabled(enabled: bool) -> Result<std::path::PathBuf, String> {
        let subkey = wide_null("Software\\Microsoft\\Windows\\CurrentVersion\\Run");
        let val_name = wide_null("DuckyDeckStudio");
        let exe_path = std::env::current_exe()
            .unwrap_or_else(|_| std::path::PathBuf::from("duckydeck-studio.exe"));
        // SAFETY: Registry calls operate on HKCU\...\Run with valid UTF-16 buffers.
        unsafe {
            let mut hkey: isize = 0;
            let open_rc = RegOpenKeyExW(
                HKEY_CURRENT_USER,
                subkey.as_ptr(),
                0,
                KEY_SET_VALUE,
                &mut hkey,
            );
            if open_rc != 0 {
                return Err(format!("Failed opening HKCU Run key (code {open_rc})"));
            }
            if enabled {
                let cmd = format!("\"{}\" --minimized", exe_path.to_string_lossy());
                let cmd_wide = wide_null(&cmd);
                let byte_len = (cmd_wide.len() * 2) as u32;
                let set_rc = RegSetValueExW(
                    hkey,
                    val_name.as_ptr(),
                    0,
                    REG_SZ,
                    cmd_wide.as_ptr() as *const u8,
                    byte_len,
                );
                RegCloseKey(hkey);
                if set_rc != 0 {
                    return Err(format!("Failed setting Run registry value (code {set_rc})"));
                }
            } else {
                let _ = RegDeleteValueW(hkey, val_name.as_ptr());
                RegCloseKey(hkey);
            }
        }
        Ok(exe_path)
    }

    pub fn apply_oled_topmost_and_geometry(
        keep_above: bool,
        on_all_desktops: bool,
        geom: Option<(i32, i32, i32, i32)>,
    ) {
        const HWND_TOPMOST: isize = -1;
        const HWND_NOTOPMOST: isize = -2;
        const SWP_NOSIZE: u32 = 0x0001;
        const SWP_NOMOVE: u32 = 0x0002;
        const SWP_NOACTIVATE: u32 = 0x0010;
        const SWP_FRAMECHANGED: u32 = 0x0020;
        const GWL_EXSTYLE: i32 = -20;
        const WS_EX_TOOLWINDOW: isize = 0x0000_0080;

        let self_pid = std::process::id();
        // SAFETY: FindWindowExW / GetWindowThreadProcessId / GetWindowTextW / SetWindowPos are standard Win32 user32 calls.
        unsafe {
            let mut hwnd: isize = 0;
            loop {
                hwnd = FindWindowExW(0, hwnd, std::ptr::null(), std::ptr::null());
                if hwnd == 0 {
                    break;
                }
                let mut pid: u32 = 0;
                GetWindowThreadProcessId(hwnd, &mut pid);
                if pid != self_pid {
                    continue;
                }
                let mut title_buf = [0u16; 256];
                let len = GetWindowTextW(hwnd, title_buf.as_mut_ptr(), title_buf.len() as i32);
                if len <= 0 {
                    continue;
                }
                let title = OsString::from_wide(&title_buf[..len as usize])
                    .to_string_lossy()
                    .to_string();
                if title.starts_with("duckyPad OLED") {
                    let want_all_desktops = on_all_desktops && !title.contains("[SingleDesktop]");
                    let ex_style = GetWindowLongPtrW(hwnd, GWL_EXSTYLE);
                    let next_ex = if want_all_desktops {
                        ex_style | WS_EX_TOOLWINDOW
                    } else {
                        ex_style & !WS_EX_TOOLWINDOW
                    };
                    if next_ex != ex_style {
                        SetWindowLongPtrW(hwnd, GWL_EXSTYLE, next_ex);
                    }
                    let insert_after = if keep_above && !title.contains("[Unpinned]") {
                        HWND_TOPMOST
                    } else {
                        HWND_NOTOPMOST
                    };
                    if let Some((gx, gy, gw, gh)) = geom {
                        SetWindowPos(
                            hwnd,
                            insert_after,
                            gx,
                            gy,
                            gw,
                            gh,
                            SWP_NOACTIVATE | SWP_FRAMECHANGED,
                        );
                    } else {
                        SetWindowPos(
                            hwnd,
                            insert_after,
                            0,
                            0,
                            0,
                            0,
                            SWP_NOMOVE | SWP_NOSIZE | SWP_NOACTIVATE | SWP_FRAMECHANGED,
                        );
                    }
                }
            }
        }
    }

    pub fn is_pid_duckydeck(pid: u32) -> bool {
        if pid <= 1 {
            return false;
        }
        // SAFETY: OpenProcess / QueryFullProcessImageNameW / CloseHandle use a stack buffer.
        unsafe {
            let hproc = OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, 0, pid);
            if hproc == 0 {
                return false;
            }
            let mut path_buf = [0u16; 1024];
            let mut sz = path_buf.len() as u32;
            let ok =
                QueryFullProcessImageNameW(hproc, 0, path_buf.as_mut_ptr(), &mut sz) != 0 && sz > 0;
            CloseHandle(hproc);
            if !ok {
                return false;
            }
            let full = OsString::from_wide(&path_buf[..sz as usize])
                .to_string_lossy()
                .to_ascii_lowercase();
            full.contains("duckydeck-studio")
        }
    }

    pub fn inspect_hwnd(hwnd: isize) -> Option<(String, String, String)> {
        static HWND_CLASS_CACHE: std::sync::OnceLock<
            std::sync::Mutex<std::collections::HashMap<(isize, u32), (String, String)>>,
        > = std::sync::OnceLock::new();

        if hwnd == 0 {
            return None;
        }
        // SAFETY: Win32 window queries use stack buffers with explicit max lengths.
        unsafe {
            let root = {
                let r = GetAncestor(hwnd, GA_ROOT);
                if r != 0 {
                    r
                } else {
                    hwnd
                }
            };
            let mut pid: u32 = 0;
            GetWindowThreadProcessId(root, &mut pid);
            if pid == std::process::id() {
                return None;
            }

            let mut title_buf = [0u16; 512];
            let title_len = GetWindowTextW(root, title_buf.as_mut_ptr(), title_buf.len() as i32);
            let caption = if title_len > 0 {
                OsString::from_wide(&title_buf[..title_len as usize])
                    .to_string_lossy()
                    .to_string()
            } else {
                String::new()
            };

            let cache = HWND_CLASS_CACHE
                .get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()));
            if let Ok(guard) = cache.lock() {
                if let Some((rc, rn)) = guard.get(&(root, pid)) {
                    if rc.is_empty() && caption.is_empty() {
                        return None;
                    }
                    return Some((rc.clone(), rn.clone(), caption));
                }
            }

            let mut class_buf = [0u16; 256];
            let class_len = GetClassNameW(root, class_buf.as_mut_ptr(), class_buf.len() as i32);
            let win_class = if class_len > 0 {
                OsString::from_wide(&class_buf[..class_len as usize])
                    .to_string_lossy()
                    .to_string()
            } else {
                String::new()
            };

            let mut exe_stem = String::new();
            if pid != 0 {
                let hproc = OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, 0, pid);
                if hproc != 0 {
                    let mut path_buf = [0u16; 1024];
                    let mut sz = path_buf.len() as u32;
                    if QueryFullProcessImageNameW(hproc, 0, path_buf.as_mut_ptr(), &mut sz) != 0
                        && sz > 0
                    {
                        let full = OsString::from_wide(&path_buf[..sz as usize])
                            .to_string_lossy()
                            .to_string();
                        if let Some(stem) = Path::new(&full).file_stem().and_then(|s| s.to_str()) {
                            exe_stem = stem.to_string();
                        }
                    }
                    CloseHandle(hproc);
                }
            }

            let resource_class = if !exe_stem.is_empty() {
                exe_stem.clone()
            } else {
                win_class.clone()
            };
            let resource_name = if !win_class.is_empty() {
                win_class
            } else {
                resource_class.clone()
            };
            if let Ok(mut guard) = cache.lock() {
                if guard.len() >= 256 {
                    guard.clear();
                }
                guard.insert((root, pid), (resource_class.clone(), resource_name.clone()));
            }
            if resource_class.is_empty() && caption.is_empty() {
                return None;
            }
            Some((resource_class, resource_name, caption))
        }
    }
}

#[cfg(target_os = "windows")]
pub async fn pick_window_via_kwin_dbus() -> std::result::Result<WindowChangeEvent, String> {
    // Wait up to 15 seconds for user to click any window (or press Escape to cancel)
    for _ in 0..300 {
        tokio::time::sleep(std::time::Duration::from_millis(50)).await;
        // SAFETY: GetAsyncKeyState and GetCursorPos/WindowFromPoint are thread-safe Win32 queries.
        let picked = unsafe {
            if (win32_window::GetAsyncKeyState(win32_window::VK_ESCAPE) as u16 & 0x8000) != 0 {
                return Err("Window picker canceled".to_string());
            }
            if (win32_window::GetAsyncKeyState(win32_window::VK_LBUTTON) as u16 & 0x8000) != 0 {
                let mut pt = win32_window::POINT::default();
                let hwnd = if win32_window::GetCursorPos(&mut pt) != 0 {
                    win32_window::WindowFromPoint(pt)
                } else {
                    win32_window::GetForegroundWindow()
                };
                win32_window::inspect_hwnd(hwnd)
            } else {
                None
            }
        };
        if let Some((resource_class, resource_name, raw_caption)) = picked {
            let caption = resolve_full_browser_caption(&resource_class, &raw_caption);
            return Ok(WindowChangeEvent {
                resource_class,
                resource_name,
                caption,
                matched_profile: None,
                picked: true,
                hid_rejected_not_on_device: false,
            });
        }
    }
    Err("Window picker timed out".to_string())
}
