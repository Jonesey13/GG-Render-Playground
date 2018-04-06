use ::rendering::{RenderableTestPrimitive};
use gg::rendering::{TextureRect, Renderable};

impl Renderable<RenderableTestPrimitive> for TextureRect {
    fn get_primitives(&mut self) -> Vec<RenderableTestPrimitive> { vec![RenderableTestPrimitive::TextureRect(self.clone())] }
}