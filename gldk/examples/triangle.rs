use gl::types::*;
use gldk::window::{GLDKWindow, WindowEvent};
use gldk::{GLConfig, GLVersion};
use glm::Vector3;
use std::ffi::{c_char, c_void, CStr, CString};
use std::mem::size_of;
use std::ptr;
use winapi::um::gl::gl::{GLenum, GLint, GLuint};

static VS_SRC: &'static str = "
#version 400

layout (location = 0) in vec4 position;
layout (location = 1) in vec3 vertexColor;

out vec3 color;

void main() {
  color = vertexColor;
  gl_Position = position;
}";

static FS_SRC: &'static str = "
#version 400

in vec3 color;

layout (location = 0) out vec4 fragment;

void main() {
  fragment = vec4(color, 1.0);
}";

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

    unsafe {
        gl::DepthFunc(gl::LEQUAL);
        gl::Enable(gl::DEPTH_TEST);
        println!(
            "{} {} {}",
            CStr::from_ptr(gl::GetString(gl::VERSION) as *const c_char)
                .to_str()
                .unwrap(),
            CStr::from_ptr(gl::GetString(gl::VENDOR) as *const c_char)
                .to_str()
                .unwrap(),
            CStr::from_ptr(gl::GetString(gl::RENDERER) as *const c_char)
                .to_str()
                .unwrap()
        )
    }

    let vs = compile_shader(VS_SRC, gl::VERTEX_SHADER);
    let fs = compile_shader(FS_SRC, gl::FRAGMENT_SHADER);
    let program = link_program(vs, fs);

    let vertices = [
        // Triangle
        // Top
        glm::vec3(0.0, 0.5, 0.0),
        // Left
        glm::vec3(-0.5, -0.5, 0.0),
        // Right
        glm::vec3(0.5, -0.5, 0.0),
    ];

    let colors = [
        // Color
        // Top
        glm::vec3(1.0, 0.0, 0.0),
        // Left
        glm::vec3(0.0, 1.0, 0.0),
        // Right
        glm::vec3(0.0, 0.0, 1.0),
    ];

    let triangle = Mesh::new(&vertices, &colors);

    window.show();

    window.run(|event| match event {
        WindowEvent::Update => {
            unsafe {
                gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
                gl::UseProgram(program);
            }

            triangle.draw();
            window.swap_buffers();
        }

        _ => {}
    });
}

fn compile_shader(src: &str, ty: GLenum) -> GLuint {
    let shader;
    unsafe {
        shader = gl::CreateShader(ty);
        // Attempt to compile the shader
        let c_str = CString::new(src.as_bytes()).unwrap();
        gl::ShaderSource(shader, 1, &c_str.as_ptr(), ptr::null());
        gl::CompileShader(shader);

        // Get the compile status
        let mut status = gl::FALSE as GLint;
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut status);

        // Fail on error
        if status != (gl::TRUE as GLint) {
            let mut len = 0;
            gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
            let mut buf = Vec::with_capacity(len as usize);
            buf.set_len((len as usize) - 1); // subtract 1 to skip the trailing null character
            gl::GetShaderInfoLog(
                shader,
                len,
                ptr::null_mut(),
                buf.as_mut_ptr() as *mut GLchar,
            );
            panic!(
                "{}",
                std::str::from_utf8(&buf)
                    .ok()
                    .expect("ShaderInfoLog not valid utf8")
            );
        }
    }
    shader
}

fn link_program(vs: GLuint, fs: GLuint) -> GLuint {
    unsafe {
        let program = gl::CreateProgram();
        gl::AttachShader(program, vs);
        gl::AttachShader(program, fs);
        gl::LinkProgram(program);
        // Get the link status
        let mut status = gl::FALSE as GLint;
        gl::GetProgramiv(program, gl::LINK_STATUS, &mut status);

        // Fail on error
        if status != (gl::TRUE as GLint) {
            let mut len: GLint = 0;
            gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len);
            let mut buf = Vec::with_capacity(len as usize);
            buf.set_len((len as usize) - 1); // subtract 1 to skip the trailing null character
            gl::GetProgramInfoLog(
                program,
                len,
                ptr::null_mut(),
                buf.as_mut_ptr() as *mut GLchar,
            );
            panic!(
                "{}",
                std::str::from_utf8(&buf)
                    .ok()
                    .expect("ProgramInfoLog not valid utf8")
            );
        }
        program
    }
}

pub type VBO = GLuint;
pub type VAO = GLuint;

pub struct Mesh {
    vertices: VBO,
    colors: VBO,

    vao: VAO,
}

impl Mesh {
    pub fn new(vertices: &[Vector3<f32>], colors: &[Vector3<f32>]) -> Self {
        unsafe {
            let mut vao = 0;
            let mut vertices_vbo = 0;
            let mut colors_vbo = 0;

            gl::GenVertexArrays(1, &mut vao);
            gl::BindVertexArray(vao);

            gl::GenBuffers(1, &mut vertices_vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, vertices_vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertices.len() * size_of::<Vector3<f32>>())
                    .try_into()
                    .unwrap(),
                vertices.as_ptr() as *const c_void,
                gl::STATIC_DRAW,
            );
            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 0, 0 as *const c_void);

            gl::GenBuffers(1, &mut colors_vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, colors_vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (colors.len() * size_of::<Vector3<f32>>())
                    .try_into()
                    .unwrap(),
                colors.as_ptr() as *const c_void,
                gl::STATIC_DRAW,
            );
            gl::EnableVertexAttribArray(1);
            gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE, 0, 0 as *const c_void);

            Self {
                vertices: vertices_vbo,
                colors: colors_vbo,
                vao,
            }
        }
    }

    pub fn draw(&self) {
        unsafe {
            gl::BindVertexArray(self.vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }
    }
}

impl Drop for Mesh {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &self.vertices);
            gl::DeleteBuffers(1, &self.colors);
            gl::DeleteVertexArrays(1, &self.vao);
        }
    }
}
