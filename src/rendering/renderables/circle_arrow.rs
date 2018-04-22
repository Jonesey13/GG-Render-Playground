use gg::rendering::{Renderable, Polygon};
use ::rendering::{GamePrimitive, AnnularSegment, Arrow, Line, ArrowHeadShape};
use na;
use na::{Vector2, Vector3, Vector4, Matrix2, Rotation2, convert};
use gg::rendering::render_by_shaders::GliumStandardPrimitive;
use gg::rendering::shaders::Shaders;
use gg::geometry::{Polynomial2d, TwoDTransformable};
use glium;
use num::Zero;
use std::f64::consts::PI;
use gg::geometry;

#[derive(Copy, Clone, Debug)]
pub struct CircleArrow {
    pub rad: f64,
    pub angle_dim: Vector2<f64>,
    pub arrow_dim: Vector2<f64>,
    pub pos: Vector3<f64>,
    pub head_shape: ArrowHeadShape,
    pub color: Vector4<f64>,
    pub thickness: f64,
    pub clockwise: bool,
    pub fixed: bool
}

impl CircleArrow{
    pub fn new_clockwise(
        rad: f64,
        angle_dim: Vector2<f64>,
        arrow_dim: Vector2<f64>,
        pos: Vector3<f64>,
        color: Vector4<f64>,
        thickness: f64,
        fixed: bool
    ) -> Self {
        Self {
            rad,
            angle_dim,
            arrow_dim,
            pos,
            head_shape: ArrowHeadShape::RoundedLine,
            color,
            thickness,
            clockwise: true,
            fixed,
        }
    }

    pub fn new_anticlockwise(
        rad: f64,
        angle_dim: Vector2<f64>,
        arrow_dim: Vector2<f64>,
        pos: Vector3<f64>,
        color: Vector4<f64>,
        thickness: f64,
        fixed: bool
    ) -> Self {
        Self {
            rad,
            angle_dim,
            arrow_dim,
            pos,
            head_shape: ArrowHeadShape::RoundedLine,
            color,
            thickness,
            clockwise: false,
            fixed,
        }
    }

    fn get_circ_line(&self) -> AnnularSegment {
        let radial_dim = Vector2::new(self.rad - self.thickness / 2.0, self.rad + self.thickness / 2.0);

        AnnularSegment::new(
            radial_dim,
            self.angle_dim,
            self.pos,
            self.color,
            self.fixed
        )
    }

    fn generate_arrow_head(&self) -> Vec<GamePrimitive> {
        match self.head_shape {
            ArrowHeadShape::Flat => panic!("Unimplemented!"),
            ArrowHeadShape::RoundedLine => self.generate_arrow_head_rounded()
        }
    }

    pub fn generate_arrow_head_rounded(&self) -> Vec<GamePrimitive> {
        let arrowhead_points = vec![
            Vector2::new(-self.arrow_dim.x + self.thickness / 2.0, -self.arrow_dim.y), 
            Vector2::new(self.thickness / 2.0, 0.0),
            Vector2::new(-self.arrow_dim.x + self.thickness / 2.0, self.arrow_dim.y)
        ];
        let local_arrow_rotation = match self.clockwise {
            false => 0.25 + self.angle_dim.y,
            true => -0.25 + self.angle_dim.x
        };
        let local_arrow_pos = match self.clockwise {
            false => self.rad * (Rotation2::new(2.0 * PI * self.angle_dim.y) * Vector2::x()),
            true => self.rad * (Rotation2::new(2.0 * PI * self.angle_dim.x) * Vector2::x())
        };

        let rotation = Rotation2::new(2.0 * PI * local_arrow_rotation);

        let mut left_arrowhead_line = geometry::Line::new(
            rotation * arrowhead_points[0], 
            rotation * arrowhead_points[1]);
        left_arrowhead_line.shift_by(local_arrow_pos + Vector2::new(self.pos.x, self.pos.y));

        let mut left_line_renderable = Line::new_rounded(
            left_arrowhead_line.beg, 
            left_arrowhead_line.end, 
            self.thickness, 
            self.color, 
            self.pos.z, 
            self.fixed);

        let mut right_arrowhead_line = geometry::Line::new(
            rotation * arrowhead_points[1], 
            rotation * arrowhead_points[2]);
        right_arrowhead_line.shift_by(local_arrow_pos + Vector2::new(self.pos.x, self.pos.y));

        let mut right_line_renderable = Line::new_rounded(
            right_arrowhead_line.beg, 
            right_arrowhead_line.end, 
            self.thickness, 
            self.color, 
            self.pos.z, 
            self.fixed);

        let mut output: Vec<GamePrimitive> = left_line_renderable.get_primitives();
        output.append(&mut right_line_renderable.get_primitives());
        output
    }
}

impl Renderable<GamePrimitive> for CircleArrow {
    fn get_primitives(&mut self) -> Vec<GamePrimitive> { 
        let mut output: Vec<GamePrimitive> = self.get_circ_line().get_primitives();
        output.append(&mut self.generate_arrow_head());
        output
    }
}