pub use geometries::{Shape, ShapeObject, ShapeStyle};
pub use glium_app::{GlShapeList, GlWindow};
pub use opengl::{Color, PixelCoord, Vertex};

#[path = "util/algorithms.rs"]
pub mod algorithms;
#[path = "app/egui/mod.rs"]
pub mod egui_app;
pub mod geometries;
#[path = "app/glium/mod.rs"]
pub mod glium_app;

mod opengl;
#[path = "util/tracing.rs"]
pub mod tracing;
