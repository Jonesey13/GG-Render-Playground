use ::rendering::{RenderableTestPrimitive};
use gg::rendering::{Renderable, Polygon};

impl Renderable<RenderableTestPrimitive> for Polygon {
    fn get_primitives(&mut self) -> Vec<RenderableTestPrimitive> { vec![RenderableTestPrimitive::Poly(self.clone())] }
}