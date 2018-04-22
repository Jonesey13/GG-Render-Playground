pub mod primitives;
pub mod glium_renderer;
pub mod renderables;
pub mod animations;
use gg::rendering::Renderable;

pub use ::rendering::primitives::*;
pub use ::rendering::glium_renderer::GliumRenderer;
pub use ::rendering::primitives::GamePrimitive;
pub use ::rendering::renderables::*;
pub type RenderableTestRenderable = Renderable<GamePrimitive>;