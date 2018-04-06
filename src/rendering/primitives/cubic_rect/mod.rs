use na;
use na::{Vector2, Vector3, Vector4, Matrix2, Rotation2, convert};
use gg::rendering::render_by_shaders::GliumStandardPrimitive;
use gg::rendering::shaders::Shaders;
use glium;
use num::Zero;

#[derive(Copy, Clone, Debug)]
pub struct CubicRect {
    pub control: BezierCubicControl,
    pub height: f64,
    pub pos: Vector3<f64>,
    pub rot: Rotation2<f64>,
    pub color: Vector4<f64>,
    pub detail_level: usize
}

#[derive(Copy, Clone, Debug)]
pub struct BezierCubicControl {
    pub one: Vector2<f64>,
    pub two: Vector2<f64>,
    pub three: Vector2<f64>,
    pub four: Vector2<f64>    
}

impl BezierCubicControl {
    pub fn new(one: Vector2<f64>, two: Vector2<f64>, three: Vector2<f64>, four: Vector2<f64>) -> Self {
        Self {
            one,
            two,
            three,
            four
        }
    }

    pub fn new_linear(pos: Vector2<f64>, change: Vector2<f64>) -> Self {
        Self {
            one: pos,
            two: pos + change / 3.0,
            three: pos + 2.0 * change / 3.0,
            four: pos + change
        }
    }

    pub fn get_point(&self, t: f64) -> Vector2<f64> {
        (1.0 - t).powi(3) * self.one
        + 3.0 * t * (1.0 - t).powi(2) * self.two
        + 3.0 * t.powi(2) * (1.0 - t) * self.three
        + t.powi(3) * self.four
    }
}

impl CubicRect {
    /// Intended for Standalone use
    pub fn new_with_color (
        control_points: BezierCubicControl,
        height: f64,
        pos: Vector3<f64>,
        rot: Rotation2<f64>,
        color: Vector4<f64>,
        detail_level: usize
    ) -> CubicRect {
        CubicRect { 
            control: control_points,
            height: height,
            pos: pos,
            rot,
            color: color,
            detail_level
        }
    }

    /// Intended for Other Bezier RenderableTestPrimitive Types
    pub fn new (
        control_points: BezierCubicControl,
        height: f64,
        pos: Vector3<f64>,
        rot: Rotation2<f64>,
        detail_level: usize
    ) -> CubicRect {
        CubicRect { 
            control: control_points,
            height: height,
            pos: pos,
            rot,
            color: Vector4::zero(),
            detail_level
        }
    }
}

impl GliumStandardPrimitive for CubicRect {
    type Vertex = CubicRectVertex;

    fn get_shaders() -> Shaders {
        Shaders::VertexTesselationFragment(
            include_str!("vertex.vs"),
            include_str!("tess_control.tcs"),
            include_str!("tess_eval.tes"),
            include_str!("fragment.fs"))
    }

    fn get_vertex(self) -> Vec<Self::Vertex> { vec![self.clone().into()] }

    fn get_primitive_type() -> glium::index::PrimitiveType {
        glium::index::PrimitiveType::Patches{ vertices_per_patch: 1 }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct CubicRectVertex {
    pub c0: [f32; 2],
    pub c1: [f32; 2],
    pub c2: [f32; 2],
    pub c3: [f32; 2],
    pub height: f32,
    pub pos: [f32; 3],
    pub color: [f32; 4],
    pub rot: [[f32; 2]; 2],
    pub detail_level: i32
}

implement_vertex!(CubicRectVertex, c0, c1, c2, c3, height, pos, color, rot, detail_level);

impl From<CubicRect> for CubicRectVertex {
    fn from(rect: CubicRect) -> Self {
        let output = CubicRectVertex {
            c0: *na::convert::<_, Vector2<f32>>(rect.control.one).as_ref(),
            c1: *na::convert::<_, Vector2<f32>>(rect.control.two).as_ref(),
            c2: *na::convert::<_, Vector2<f32>>(rect.control.three).as_ref(),
            c3: *na::convert::<_, Vector2<f32>>(rect.control.four).as_ref(),
            height: rect.height as f32,
            pos: *na::convert::<_, Vector3<f32>>(rect.pos).as_ref(),
            color: *na::convert::<_, Vector4<f32>>(rect.color).as_ref(),
            rot: *convert::<_, Matrix2<f32>>(*rect.rot.matrix()).as_ref(),
            detail_level: rect.detail_level as i32
        };
        output
    }
}
