pub mod bezier_rect;
pub mod bezier_branch_rect;
pub mod bezier_branch_circ;
pub mod paddle_rect;
pub mod rect_with_gradient;
use gg::rendering::{PlainText, Rectangle, CirclePart, Polygon, TextureRect};

pub use self::bezier_rect::{BezierRect, BezierQuadControl};
pub use self::bezier_branch_rect::BezierBranchRect;
pub use self::bezier_branch_circ::BezierBranchCirc;
pub use self::paddle_rect::{PaddleRect, BezierCubicControl, PolyCubicControl};
pub use self::rect_with_gradient::RectWithGradient;

#[derive(Clone)]
pub enum GamePrimitive {
    Rect(Rectangle),
    RectWithGradient(RectWithGradient),
    Circ(CirclePart),
    Poly(Polygon),
    Text(PlainText),
    BezierRect(BezierRect),
    PaddleRect(PaddleRect),
    BezierBranchRect(BezierBranchRect),
    BezierBranchCirc(BezierBranchCirc),
    TextureRect(TextureRect),
}