use ::rendering::{Renderable, BezierBranchCirc, GamePrimitive};

impl Renderable<GamePrimitive> for BezierBranchCirc {
    fn get_primitives(&mut self) -> Vec<GamePrimitive> { vec![GamePrimitive::BezierBranchCirc(self.clone())] }
}