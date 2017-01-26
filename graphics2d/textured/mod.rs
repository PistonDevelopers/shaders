//! Shaders for textured rendering.

/// Vertex shader for GLSL 1.20
pub const VERTEX_GLSL_120: &'static [u8] = include_bytes!("120.glslv");
/// Vertex shader for GLSL 1.50
pub const VERTEX_GLSL_150_CORE: &'static [u8] = include_bytes!("150_core.glslv");

/// Fragment shader for GLSL 1.20
pub const FRAGMENT_GLSL_120: &'static [u8] = include_bytes!("120.glslf");
/// Fragment shader for GLSL 1.50
pub const FRAGMENT_GLSL_150_CORE: &'static [u8] = include_bytes!("150_core.glslf");

/// Vertex shader for GLSL 1.20
pub const VERTEX_GLSL_120_WEBGL: &'static [u8] = include_bytes!("120_webgl.glslv");
/// Vertex shader for GLSL 1.50
pub const VERTEX_GLSL_150_CORE_WEBGL: &'static [u8] = include_bytes!("150_core_webgl.glslv");

/// Fragment shader for GLSL 1.20
pub const FRAGMENT_GLSL_120_WEBGL: &'static [u8] = include_bytes!("120_webgl.glslf");
/// Fragment shader for GLSL 1.50
pub const FRAGMENT_GLSL_150_CORE_WEBGL: &'static [u8] = include_bytes!("150_core_webgl.glslf");
