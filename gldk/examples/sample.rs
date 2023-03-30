use gldk::window::GLDKWindow;

fn main() {
    let window = GLDKWindow::new(500,500,"Sample application");
    window.run();
}