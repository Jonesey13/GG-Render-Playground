use ::rendering::{GamePrimitive};
use gg::rendering::{TextureRect, Renderable};

impl Renderable<GamePrimitive> for TextureRect {
    fn get_primitives(&mut self) -> Vec<GamePrimitive> { vec![GamePrimitive::TextureRect(self.clone())] }
}