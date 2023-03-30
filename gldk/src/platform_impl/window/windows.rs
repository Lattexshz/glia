use core::ffi::{c_int, c_void};
use std::ffi::{c_char, CStr, CString};
use std::ptr::addr_of;
use raw_window_handle::{RawWindowHandle, Win32WindowHandle};
use windows_sys::s;
use windows_sys::Win32::Foundation::{HINSTANCE, HWND, LPARAM, LRESULT, WPARAM};
use windows_sys::Win32::Graphics::Gdi::{GetDC, ValidateRect};
use windows_sys::Win32::Graphics::OpenGL::*;
use windows_sys::Win32::System::LibraryLoader::{FreeLibrary, GetModuleHandleA, GetProcAddress, LoadLibraryA};
use windows_sys::Win32::UI::WindowsAndMessaging::*;
use crate::window::WindowID;

pub mod wgl {
    include!(concat!(env!("OUT_DIR"), "/wgl_bindings.rs"));
}

/// Functions that are not necessarily always available
pub mod wgl_extra {
    include!(concat!(env!("OUT_DIR"), "/wgl_extra_bindings.rs"));
}

#[link(name = "opengl32")]
extern "C" {}


pub struct RWindow {
    hwnd: HWND,
    hinstance: HINSTANCE
}

impl RWindow {
    pub fn new(width:u32,height:u32,title:&str) -> Self {
        unsafe {
            let instance = GetModuleHandleA(std::ptr::null());
            debug_assert!(instance != 0);

            let window_class = s!("window");

            let wc = WNDCLASSA {
                hCursor: LoadCursorW(0, IDC_ARROW),
                hInstance: instance,
                lpszClassName: window_class,
                style: CS_HREDRAW | CS_VREDRAW | CS_OWNDC,
                lpfnWndProc: Some(wndproc),
                cbClsExtra: 0,
                cbWndExtra: 0,
                hIcon: 0,
                hbrBackground: 0,
                lpszMenuName: std::ptr::null(),
            };

            let atom = RegisterClassA(&wc);
            debug_assert!(atom != 0);

            let hwnd = CreateWindowExA(
                0,
                window_class,
                s!("This is a sample window"),
                WS_OVERLAPPEDWINDOW | WS_VISIBLE,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                0,
                0,
                instance,
                std::ptr::null(),
            );

            Self {
                hwnd,
                hinstance: instance,
            }
        }
    }

    pub fn handle(&self) -> RawWindowHandle {
        let mut window_handle = Win32WindowHandle::empty();
        window_handle.hwnd = self.hwnd as *mut c_void;
        window_handle.hinstance = self.hinstance as *mut c_void;
        RawWindowHandle::Win32(window_handle)
    }

    pub fn id(&self) -> WindowID {
        WindowID(self.hwnd as u64)
    }

    pub fn run(&self) {
        unsafe {
            let mut message = core::mem::zeroed();

            while GetMessageA(&mut message, 0, 0, 0) != 0 {
                DispatchMessageA(&message);
            }
        }
    }
}

extern "system" fn wndproc(window: HWND, message: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    unsafe {
        match message {
            WM_CREATE => {
                // let pfd = PIXELFORMATDESCRIPTOR {
                //     nSize: std::mem::size_of::<PIXELFORMATDESCRIPTOR>() as u16,
                //     nVersion: 1,
                //     dwFlags: PFD_DRAW_TO_WINDOW | PFD_DRAW_TO_BITMAP | PFD_SUPPORT_OPENGL | PFD_GENERIC_ACCELERATED | PFD_DOUBLEBUFFER | PFD_SWAP_LAYER_BUFFERS,
                //     iPixelType: PFD_TYPE_RGBA,
                //     cColorBits: 32,
                //     cRedBits: 0,
                //     cRedShift: 255,
                //     cGreenBits: 255,
                //     cGreenShift: 0,
                //     cBlueBits: 0,
                //     cBlueShift: 0,
                //     cAlphaBits: 0,
                //     cAlphaShift: 0,
                //     cAccumBits: 0,
                //     cAccumRedBits: 0,
                //     cAccumGreenBits: 0,
                //     cAccumBlueBits: 0,
                //     cAccumAlphaBits: 0,
                //     cDepthBits: 32,
                //     cStencilBits: 8,
                //     cAuxBuffers: 0,
                //     iLayerType: PFD_MAIN_PLANE,
                //     bReserved: 0,
                //     dwLayerMask: 0,
                //     dwVisibleMask: 0,
                //     dwDamageMask: 0,
                // };
                // let hdc = GetDC(window);
                // let pixel_format = ChoosePixelFormat(hdc,addr_of!(pfd));
                // SetPixelFormat(hdc,pixel_format as c_int,addr_of!(pfd));
                // let ctx = wgl::CreateContext(hdc as wgl::types::HDC);
                // wgl::MakeCurrent(hdc as wgl::types::HDC,ctx);
                //
                // gl::load_with(|ext| {
                //     wgl::GetProcAddress(ext.as_ptr() as wgl::types::LPCSTR) as *mut c_void
                // });
                //
                // gl::ClearColor(1.0,1.0,0.0,1.0);

                0
            }
            WM_PAINT => {
                ValidateRect(window, std::ptr::null());
                0
            }
            WM_DESTROY => {
                PostQuitMessage(0);
                0
            }
            _ => DefWindowProcA(window, message, wparam, lparam),
        }
    }
}
