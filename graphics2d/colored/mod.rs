//! Shaders for colored rendering.

/// Vertex shader for GLSL 1.20
pub const VERTEX_GLSL_120: &'static [u8] = include_bytes!("120.glslv");
/// Vertex shader for GLSL 1.50
pub const VERTEX_GLSL_150_CORE: &'static [u8] = include_bytes!("150_core.glslv");

/// Fragment shader for GLSL 1.20
pub const FRAGMENT_GLSL_120: &'static [u8] = include_bytes!("120.glslf");
/// Fragmentshader for GLSL 1.50
pub const FRAGMENT_GLSL_150_CORE: &'static [u8] = include_bytes!("150_core.glslf");
