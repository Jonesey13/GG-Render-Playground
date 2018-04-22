use ::rendering::{Renderable, GamePrimitive, BezierBranchRect};

impl Renderable<GamePrimitive> for BezierBranchRect {
    fn get_primitives(&mut self) -> Vec<GamePrimitive> { vec![GamePrimitive::BezierBranchRect(self.clone())] }
}