use std::intrinsics::transmute;

pub mod wgl {
    include!(concat!(env!("OUT_DIR"), "/wgl_bindings.rs"));
}

/// Functions that are not necessarily always available
pub mod wgl_extra {
    include!(concat!(env!("OUT_DIR"), "/wgl_extra_bindings.rs"));
}

pub type WGLCREATECONTEXTATTRIBSARBPROC = fn(
    hDc: wgl_extra::types::HDC,
    hShareContext: wgl_extra::types::HGLRC,
    attribs: &[wgl_extra::types::GLenum],
) -> wgl_extra::types::HGLRC;

pub type WGLSWAPINTERVALEXTPROC = fn(i:u32);

pub struct WGLARBFunctions {
    pub wglCreateContextAttribsARB: WGLCREATECONTEXTATTRIBSARBPROC,
    pub wglSwapIntervalEXT: WGLSWAPINTERVALEXTPROC
}

impl WGLARBFunctions {
    pub fn load() -> Self {
        let wglCreateContextAttribsARB: WGLCREATECONTEXTATTRIBSARBPROC = unsafe {
            transmute(wgl::GetProcAddress(
                "wglCreateContextAttribsARB\0".as_ptr() as wgl::types::LPCSTR
            ))
        };
        let wglSwapIntervalEXT: WGLSWAPINTERVALEXTPROC = unsafe {
            transmute(wgl::GetProcAddress(
                "wglSwapIntervalEXT\0".as_ptr() as wgl::types::LPCSTR
            ))
        };
        Self {
            wglCreateContextAttribsARB,
            wglSwapIntervalEXT,
        }
    }
}
