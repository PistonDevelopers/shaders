#![deny(missing_docs)]

//! Shaders for 2D graphics backends.
//!
//! Piston supports different backends for 2D using various APIs.
//! Since these backends uses the same shader code, it makes sense
//! to share them through a common library to simplify maintenance.

pub mod colored;
pub mod textured;
