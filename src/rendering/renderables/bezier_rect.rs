use gg::rendering::{Renderable};
use ::rendering::{RenderableTestPrimitive, BezierRect};

impl Renderable<RenderableTestPrimitive> for BezierRect {
    fn get_primitives(&mut self) -> Vec<RenderableTestPrimitive> { vec![RenderableTestPrimitive::BezierRect(self.clone())] }
}