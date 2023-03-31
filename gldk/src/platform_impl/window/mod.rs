use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};

#[cfg(target_os = "linux")]
pub mod linux;

#[cfg(target_os = "linux")]
pub use self::linux::*;

#[cfg(target_os = "macos")]
pub mod macos;
#[cfg(target_os = "macos")]
pub use self::macos::*;

#[cfg(target_os = "windows")]
pub mod windows;
#[cfg(target_os = "windows")]
pub use self::windows::*;

unsafe impl HasRawWindowHandle for RWindow {
    fn raw_window_handle(&self) -> RawWindowHandle {
        self.handle()
    }
}
