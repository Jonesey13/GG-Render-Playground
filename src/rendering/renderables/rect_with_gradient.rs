use gg::rendering::{Renderable};
use ::rendering::*;

impl Renderable<GamePrimitive> for RectWithGradient {
    fn get_primitives(&mut self) -> Vec<GamePrimitive> { vec![GamePrimitive::RectWithGradient(self.clone())] }
}