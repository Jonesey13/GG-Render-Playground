use gg::rendering::{Renderable};
use ::rendering::{RenderableTestPrimitive, CubicRect};

impl Renderable<RenderableTestPrimitive> for CubicRect {
    fn get_primitives(&mut self) -> Vec<RenderableTestPrimitive> { vec![RenderableTestPrimitive::CubicRect(self.clone())] }
}