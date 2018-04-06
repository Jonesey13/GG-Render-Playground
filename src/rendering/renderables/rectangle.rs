use gg::rendering::{Renderable, Rectangle};
use ::rendering::RenderableTestPrimitive;

impl Renderable<RenderableTestPrimitive> for Rectangle {
    fn get_primitives(&mut self) -> Vec<RenderableTestPrimitive> { vec![RenderableTestPrimitive::Rect(self.clone())] }
}