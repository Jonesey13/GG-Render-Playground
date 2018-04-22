use ::rendering::{GamePrimitive};
use gg::rendering::{Renderable, Polygon};

impl Renderable<GamePrimitive> for Polygon {
    fn get_primitives(&mut self) -> Vec<GamePrimitive> { vec![GamePrimitive::Poly(self.clone())] }
}