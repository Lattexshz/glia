mod platform_impl;
mod sys;
pub mod window;
pub mod error;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GLVersion {
    V3_0,
    V3_1,
    V3_2,
    V3_3,

    V4_0,
    V4_1,
    V4_2,
    V4_3,
    V4_4,
    V4_5,
    V4_6,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GLConfig {
    pub version: GLVersion,
}

impl Default for GLConfig {
    fn default() -> Self {
        Self {
            version: GLVersion::V3_1,
        }
    }
}
