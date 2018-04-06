use ::rendering::RenderableTestPrimitive;
use gg::rendering::{Renderable, PlainText};

impl Renderable<RenderableTestPrimitive> for PlainText {    
    fn get_primitives(&mut self) -> Vec<RenderableTestPrimitive> { vec![RenderableTestPrimitive::Text(self.clone())] }
}