//! This is documentation for the `edwards` module.
//!
//! The edwards module is meant to be used for bar.
mod curve;
mod point;
mod scalar;

pub use crate::edwards::curve::{Curve, CurveID, Params};
pub use crate::edwards::point::{Point, ProyCoordinates};
pub use crate::edwards::scalar::Scalar;
