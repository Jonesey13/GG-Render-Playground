use gg::rendering::{Renderable, Rectangle};
use ::rendering::GamePrimitive;

impl Renderable<GamePrimitive> for Rectangle {
    fn get_primitives(&mut self) -> Vec<GamePrimitive> { vec![GamePrimitive::Rect(self.clone())] }
}