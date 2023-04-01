use crate::sys::{wgl, wgl_extra};
use crate::window::{KeyCode, WindowEvent, WindowID};
use core::ffi::c_void;
use std::ffi::CString;

use raw_window_handle::{RawWindowHandle, Win32WindowHandle};

use crate::{GLConfig, GLVersion};
use std::ptr::{addr_of, null_mut};

use windows_sys::core::PCSTR;
use windows_sys::s;
use windows_sys::Win32::Foundation::{HINSTANCE, HWND, LPARAM, LRESULT, WPARAM};
use windows_sys::Win32::Graphics::Gdi::GetDC;
use windows_sys::Win32::Graphics::OpenGL::*;
use windows_sys::Win32::System::LibraryLoader::{GetModuleHandleA, GetProcAddress};
use windows_sys::Win32::UI::WindowsAndMessaging::*;

pub struct RWindow {
    hwnd: HWND,
    hinstance: HINSTANCE,
    ctx: HGLRC
}

impl RWindow {
    pub fn new(width: u32, height: u32, title: &str, conf: GLConfig) -> Self {
        let (major, minor) = match conf.version {
            GLVersion::V3_0 => (3, 0),
            GLVersion::V3_1 => (3, 1),
            GLVersion::V3_2 => (3, 2),
            GLVersion::V3_3 => (3, 3),
            GLVersion::V4_0 => (4, 0),
            GLVersion::V4_1 => (4, 1),
            GLVersion::V4_2 => (4, 2),
            GLVersion::V4_3 => (4, 3),
            GLVersion::V4_4 => (4, 4),
            GLVersion::V4_5 => (4, 5),
            GLVersion::V4_6 => (4, 6),
        };

        unsafe {
            let instance = GetModuleHandleA(std::ptr::null());

            debug_assert!(instance != 0);

            let window_class = s!("window");

            let wc = WNDCLASSA {
                hCursor: LoadCursorW(0, IDC_ARROW),
                hInstance: instance,
                lpszClassName: window_class,
                style: CS_HREDRAW | CS_VREDRAW | CS_OWNDC,
                lpfnWndProc: Some(Self::wndproc),
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
                format!("{}\0", title).as_ptr(),
                WS_OVERLAPPEDWINDOW | WS_VISIBLE,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                width as i32,
                height as i32,
                0,
                0,
                instance,
                std::ptr::null(),
            );

            let pfd = PIXELFORMATDESCRIPTOR {
                nSize: std::mem::size_of::<PIXELFORMATDESCRIPTOR>() as u16,
                nVersion: 1,
                dwFlags: PFD_DRAW_TO_WINDOW | PFD_SUPPORT_OPENGL | PFD_DOUBLEBUFFER,
                iPixelType: PFD_TYPE_RGBA,
                cColorBits: 32,
                cRedBits: 0,
                cRedShift: 0,
                cGreenBits: 0,
                cGreenShift: 0,
                cBlueBits: 0,
                cBlueShift: 0,
                cAlphaBits: 0,
                cAlphaShift: 0,
                cAccumBits: 0,
                cAccumRedBits: 0,
                cAccumGreenBits: 0,
                cAccumBlueBits: 0,
                cAccumAlphaBits: 0,
                cDepthBits: 24,
                cStencilBits: 8,
                cAuxBuffers: 0,
                iLayerType: PFD_MAIN_PLANE,
                bReserved: 0,
                dwLayerMask: 0,
                dwVisibleMask: 0,
                dwDamageMask: 0,
            };

            let hdc = GetDC(hwnd);
            let pixel_format = ChoosePixelFormat(hdc, addr_of!(pfd));
            SetPixelFormat(hdc, pixel_format, addr_of!(pfd));

            let old_ctx = wgl::CreateContext(hdc as wgl::types::HDC);
            wgl::MakeCurrent(hdc as wgl::types::HDC, old_ctx);

            let att = [
                wgl_extra::CONTEXT_MAJOR_VERSION_ARB,
                major,
                wgl_extra::CONTEXT_MINOR_VERSION_ARB,
                minor,
                wgl_extra::CONTEXT_FLAGS_ARB,
                0,
                wgl_extra::CONTEXT_PROFILE_MASK_ARB,
                wgl_extra::CONTEXT_CORE_PROFILE_BIT_ARB,
                0,
            ];

            let func = crate::sys::WGLARBFunctions::load();
            let ctx =
                (func.wglCreateContextAttribsARB)(hdc as wgl_extra::types::HDC, null_mut(), &att);

            wgl::DeleteContext(old_ctx);

            Self {
                hwnd,
                hinstance: instance,
                ctx: ctx as HGLRC
            }
        }
    }

    pub fn get_proc_address(&self, addr: &str) -> *const c_void {
        let addr = CString::new(addr.as_bytes()).unwrap();
        let addr = addr.as_ptr();

        unsafe {
            let p = wgl::GetProcAddress(addr) as *const core::ffi::c_void;
            if !p.is_null() {
                return p;
            }
            let gl = GetModuleHandleA("Opengl32.dll".as_ptr());
            GetProcAddress(gl, addr as PCSTR).unwrap() as *const _
        }
    }

    pub fn handle(&self) -> RawWindowHandle {
        let mut window_handle = Win32WindowHandle::empty();
        window_handle.hwnd = self.hwnd as *mut c_void;
        window_handle.hinstance = self.hinstance as *mut c_void;
        RawWindowHandle::Win32(window_handle)
    }

    pub fn id(&self) -> WindowID {
        WindowID(0)
    }

    pub fn make_current(&self) {
        unsafe {
            wgl::MakeCurrent(GetDC(self.hwnd) as wgl::types::HDC, self.ctx as crate::sys::wgl::types::HGLRC);
        }
    }

    pub fn run<F>(&self, callback: F)
    where
        F: Fn(WindowEvent),
    {
        unsafe {
            let mut message = core::mem::zeroed();

            while GetMessageA(&mut message, 0, 0, 0) != 0 {
                DispatchMessageA(&message);

                match message.message {
                    WM_PAINT => {
                        callback(WindowEvent::Update);
                    }

                    WM_CLOSE => {
                        callback(WindowEvent::CloseRequested);
                    }

                    WM_KEYDOWN => {
                        callback(WindowEvent::Keydown(KeyCode(message.wParam as u32)));
                    }

                    WM_KEYUP => {
                        callback(WindowEvent::Keyup(KeyCode(message.wParam as u32)));
                    }

                    _ => {}
                }
            }
        }
    }

    pub fn swap_buffers(&self) {
        unsafe {
            SwapBuffers(GetDC(self.hwnd));
        }
    }

    extern "system" fn wndproc(
        window: HWND,
        message: u32,
        wparam: WPARAM,
        lparam: LPARAM,
    ) -> LRESULT {
        unsafe {
            if message == WM_NCCREATE {
                let cs = lparam as *const CREATESTRUCTA;
                let _this = (*cs).lpCreateParams as *const RWindow;
            }

            match message {
                WM_CREATE => 0,
                WM_PAINT => 0,
                WM_DESTROY => {
                    PostQuitMessage(0);
                    0
                }
                _ => DefWindowProcA(window, message, wparam, lparam),
            }
        }
    }
}
