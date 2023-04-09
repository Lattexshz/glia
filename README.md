# IMPORTANT NOTICE: This library has been archived since 16:50 (UTC+0900).
# Please use [GSFK](https://github.com/Lattexshz/GSFK) instead, which allows OpenGL context creation, Vulkan context creation, etc. in one library!

# GLDK
### OpenGL application Development toolKit

GLDK is an open source, cross-platform toolkit for developing OpenGL applications;  
GLDK abstracts platform-dependent processes such as window creation, management, and context creation.  
  
## Example
```Rust
use gldk::window::{GLDKWindow, WindowEvent};
use gldk::{GLConfig, GLVersion};


fn main() {
    let window = GLDKWindow::new(
        500,
        500,
        "GLDK example",
        Some(GLConfig {
            version: GLVersion::V3_3,
        }),
    ).unwrap();

    window.make_current();
    window.swap_interval(true);

    gl::load_with(|s| window.get_proc_address(s));

    window.show();

    window.run(|event| match event {
        WindowEvent::Update => {
            // Drawing!
            window.swap_buffers();
        }

        _ => {}
    });
}
```

[Examples of how it actually works](https://github.com/Lattexshz/gldk/blob/main/gldk/examples/triangle.rs)

# Release
The latest release (source code, dll) is available [here](https://github.com/Lattexshz/gldk/releases)

# License
GLDK is licensed under MIT LICENSE
