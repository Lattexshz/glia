use gl::{COLOR_BUFFER_BIT,DEPTH_BUFFER_BIT,VERSION, VENDOR};
use gldk::window::{GLDKWindow, WindowEvent};

fn main() {
    let window = GLDKWindow::new(500, 500, "Sample application", None);
    let gl = window.get_gl();

    println!("GL_VERSION: {}",gl.get_string(VERSION));
    window.run(|event| {
        match event {
            WindowEvent::Update => {
                gl.clear_color(1.0,0.0,1.0,0.5);
                gl.clear(COLOR_BUFFER_BIT | DEPTH_BUFFER_BIT);
                window.swap_buffers();
            }

            _ => {

            }
        }
    });
}
