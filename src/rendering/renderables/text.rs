use ::rendering::GamePrimitive;
use gg::rendering::{Renderable, PlainText};

impl Renderable<GamePrimitive> for PlainText {    
    fn get_primitives(&mut self) -> Vec<GamePrimitive> { vec![GamePrimitive::Text(self.clone())] }
}