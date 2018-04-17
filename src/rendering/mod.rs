pub mod primitives;
pub mod glium_renderer;
pub mod renderables;
pub mod animations;
use gg::rendering::Renderable;

pub use ::rendering::primitives::{BezierRect, BezierBranchRect, BezierBranchCirc, BezierQuadControl, CubicRect, BezierCubicControl, PolyCubicControl};
pub use ::rendering::glium_renderer::GliumRenderer;
pub use ::rendering::primitives::RenderableTestPrimitive;
pub use ::rendering::renderables::*;
pub type RenderableTestRenderable = Renderable<RenderableTestPrimitive>;