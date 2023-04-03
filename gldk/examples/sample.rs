use gl::types::{GLboolean, GLchar, GLenum, GLfloat, GLint, GLsizei, GLsizeiptr, GLuint};
use gl::{COLOR_BUFFER_BIT, VENDOR, VERSION};
use gldk::window::{GLDKWindow, WindowEvent};
use std::ffi::{c_char, c_void, CStr, CString};
use std::{mem, ptr};

fn main() {
    let window = GLDKWindow::new(500, 500, "GLDK rectangle drawing example", None);

    window.make_current();
    gl::load_with(|s| {
        window.get_proc_address(s)
    });

        window.run(|event| match event {
        WindowEvent::Update => {
            unsafe {
                gl::ClearColor(1.0,0.5,0.5,1.0);
                gl::Clear(gl::COLOR_BUFFER_BIT);
            }

            window.swap_buffers();
        }

        _ => {}
    });
}

