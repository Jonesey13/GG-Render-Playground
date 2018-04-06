use ::rendering::{Renderable, BezierBranchCirc, RenderableTestPrimitive};

impl Renderable<RenderableTestPrimitive> for BezierBranchCirc {
    fn get_primitives(&mut self) -> Vec<RenderableTestPrimitive> { vec![RenderableTestPrimitive::BezierBranchCirc(self.clone())] }
}