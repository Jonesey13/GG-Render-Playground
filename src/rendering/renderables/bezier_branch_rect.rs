use ::rendering::{Renderable, RenderableTestPrimitive, BezierBranchRect};

impl Renderable<RenderableTestPrimitive> for BezierBranchRect {
    fn get_primitives(&mut self) -> Vec<RenderableTestPrimitive> { vec![RenderableTestPrimitive::BezierBranchRect(self.clone())] }
}