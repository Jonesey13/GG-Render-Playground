use gg::rendering::{Renderable};
use ::rendering::{GamePrimitive, BezierRect};

impl Renderable<GamePrimitive> for BezierRect {
    fn get_primitives(&mut self) -> Vec<GamePrimitive> { vec![GamePrimitive::BezierRect(self.clone())] }
}