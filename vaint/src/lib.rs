pub use geometries::{Shape, ShapeStyle};
pub use opengl::{Color, PixelCoord, Vertex};

#[path = "util/algorithms.rs"]
pub mod algorithms;
pub mod geometries;
mod opengl;
#[path = "util/tracing.rs"]
mod tracing;
