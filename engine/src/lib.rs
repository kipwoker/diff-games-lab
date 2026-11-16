//! Maximal stable bridge for a linear differential game, in support-function form.
//!
//! A convex set is carried as 110 support values over a fixed direction grid, never as a
//! vertex list. The modules follow the layers of the method:
//!
//! * geometry — [`matrix`], [`linear_system`], [`combinations`], [`polytope`],
//!   [`support_function`], [`convex_polygon`], [`directions`];
//! * the game — [`pontryagin_difference`] (the dual update), [`minkowski_sum`],
//!   [`stable_bridge`];
//! * the answer — [`strategy`], which turns the bridge into two controllers and a
//!   blockwise certificate;
//! * input and output — [`input_parser`], [`output_writer`].
//!
//! The documentation in `../docs/` derives the recurrence these modules evaluate.

#![forbid(unsafe_code)]

pub mod combinations;
pub mod convex_polygon;
pub mod directions;
pub mod error;
pub mod input_parser;
pub mod linear_system;
pub mod matrix;
pub mod minkowski_sum;
pub mod nonlinear;
pub mod output_writer;
pub mod polytope;
pub mod pontryagin_difference;
pub mod stable_bridge;
pub mod strategy;
pub mod support_function;

pub use stable_bridge::{StableBridge, SetupError};

/// Number of sampled directions.
pub const DIRECTION_COUNT: usize = 110;

/// Pieces one step is refined into when the interval map is applied.
pub const SUBSTEPS_PER_STEP: usize = 110;

/// Truncation tolerance of the matrix series.
pub const SERIES_TOLERANCE: f64 = 1e-5;

/// Vertex de-duplication and half-space acceptance slack.
///
/// Coarse on purpose: the set is a polyhedral approximation built from 110 directions, and
/// the slack absorbs the rounding of the vertex solves that built it.
pub const VERTEX_EPSILON: f64 = 0.01;
