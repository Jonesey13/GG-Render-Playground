use na::{Vector1, Vector3, Vector4, Rotation2, Matrix2, convert};
use num::Zero;
use gg::rendering::render_by_shaders::GliumStandardPrimitive;
use gg::rendering::shaders::Shaders;

// Gradient Applies Left -> Right BEFORE ROTATION
#[derive(Copy, Clone, Debug)]
pub struct RectWithGradient {
    pub length: f64,  /// x-axis
    pub height: f64,  /// y-axis
    pub rot: Rotation2<f64>,
    pub pos: Vector3<f64>,
    pub color: Vector4<f64>,
    pub left_gradient: f64,
    pub right_gradient: f64,
    pub fixed: bool,
}

impl RectWithGradient {
    pub fn new(
            length: f64, 
            height: f64, 
            pos: Vector3<f64>,
            rotation: Rotation2<f64>,
            color: Vector4<f64>,
            left_gradient: f64,
            right_gradient: f64,
            fixed: bool
        ) -> Self {
        Self {
            length,
            height,
            rot: rotation,
            pos,
            color,
            left_gradient,
            right_gradient,
            fixed
        }
    }
}

impl GliumStandardPrimitive for RectWithGradient {
    type Vertex = RectWithGradientVertex;

    fn get_shaders() -> Shaders {
        Shaders::VertexGeometryFragment(
            include_str!("vertex.vs"),
            include_str!("geometry.ges"),
            include_str!("fragment.fs"))
    }

    fn get_vertex(self) -> Vec<Self::Vertex> { vec![self.clone().into()] }
}

#[derive(Copy, Clone, Debug)]
pub struct RectWithGradientVertex {
    pub length: f32,
    pub height: f32,
    pub rot: [[f32; 2]; 2],
    pub pos: [f32; 3],
    pub color: [f32; 4],
    pub left_gradient: f64,
    pub right_gradient: f64,
    pub fixed_pos: u32
}

implement_vertex!(RectWithGradientVertex, length, height, rot, pos, color, left_gradient, right_gradient, fixed_pos);

impl From<RectWithGradient> for RectWithGradientVertex {
    fn from(rect: RectWithGradient) -> Self {
        Self {
            length: rect.length as f32,
            height: rect.height as f32,
            rot: *convert::<_, Matrix2<f32>>(*rect.rot.matrix()).as_ref(),
            pos: *convert::<_, Vector3<f32>>(rect.pos).as_ref(),
            color: *convert::<_, Vector4<f32>>(rect.color).as_ref(),
            left_gradient: rect.left_gradient,
            right_gradient: rect.right_gradient,
            fixed_pos: rect.fixed as u32
        }
    }
}
