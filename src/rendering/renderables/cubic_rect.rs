use gg::rendering::{Renderable};
use ::rendering::{GamePrimitive, CubicRect};

impl Renderable<GamePrimitive> for CubicRect {
    fn get_primitives(&mut self) -> Vec<GamePrimitive> { vec![GamePrimitive::CubicRect(self.clone())] }
}