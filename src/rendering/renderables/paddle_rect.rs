use gg::rendering::{Renderable};
use ::rendering::{GamePrimitive, PaddleRect};

impl Renderable<GamePrimitive> for PaddleRect {
    fn get_primitives(&mut self) -> Vec<GamePrimitive> { vec![GamePrimitive::PaddleRect(self.clone())] }
}