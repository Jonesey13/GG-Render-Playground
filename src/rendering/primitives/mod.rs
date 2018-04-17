pub mod bezier_rect;
pub mod bezier_branch_rect;
pub mod bezier_branch_circ;
pub mod cubic_rect;
use gg::rendering::{PlainText, Rectangle, CirclePart, Polygon, TextureRect};

pub use self::bezier_rect::{BezierRect, BezierQuadControl};
pub use self::bezier_branch_rect::BezierBranchRect;
pub use self::bezier_branch_circ::BezierBranchCirc;
pub use self::cubic_rect::{CubicRect, BezierCubicControl, PolyCubicControl};

#[derive(Clone)]
pub enum RenderableTestPrimitive {
    Rect(Rectangle),
    Circ(CirclePart),
    Poly(Polygon),
    Text(PlainText),
    BezierRect(BezierRect),
    CubicRect(CubicRect),
    BezierBranchRect(BezierBranchRect),
    BezierBranchCirc(BezierBranchCirc),
    TextureRect(TextureRect)
}