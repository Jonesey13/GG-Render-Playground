use na::{Vector2, Vector3, Vector4, Matrix2, Rotation2, convert};
use ::rendering::{SpikyCircle, CircleArrow, AnnularSegment, PaddleRect, PolyCubicControl, BezierCubicControl, Line};
use ::rendering::{GamePrimitive };
use gg::rendering::{Renderable};
use gg::geometry::{Polynomial2d, build_interpolating_poly2d};
use ::constants;
use gg::geometry;
use std::f64::consts::PI;
use num::Zero;

#[derive(Clone, Debug)]
pub struct GrabItem {
    pos: Vector3<f64>,
    scale: f64,
    select_color: Vector4<f64>,
    rot_angle: f64,
    paddle_spec: Option<Polynomial2d>
}

impl GrabItem {
    pub fn new(
        pos: Vector3<f64>,
        scale: f64,
        rot_angle: f64
    ) -> Self {
        Self {
            pos,
            scale,
            select_color: constants::grab_item::FOREGROUND_COLOUR.into(),
            rot_angle,
            paddle_spec: None
        }
    }

    fn get_pacman_primitives(&self) -> Vec<GamePrimitive> {
        let angular_dim: Vector2<f64> = constants::grab_item::GRAB_ANGLES.into();

        let background = AnnularSegment::new(
            Vector2::new(0.0, self.scale / 2.0),
            angular_dim + Vector2::new(self.rot_angle, self.rot_angle),
            self.pos,
            constants::grab_item::BACKGROUND_COLOUR.into(),
            false
        );

        let outer_circle = AnnularSegment::new(
            Vector2::new(self.scale * (1.0 - constants::grab_item::THICKNESS) / 2.0, 
                        self.scale * (1.0 + constants::grab_item::THICKNESS) / 2.0),
            angular_dim + Vector2::new(self.rot_angle, self.rot_angle),
            self.pos + Vector3::new(0.0, 0.0, -0.01),
            self.select_color,
            false
        );

        let local_pos = Vector2::new(self.pos.x, self.pos.y);

        let upper_right_line = geometry::Line::new(
            local_pos,
            local_pos + self.scale / 2.0 * (Rotation2::new(2.0 * PI * (angular_dim.x + self.rot_angle)) * Vector2::x())
        );

        let mut upper_right_line = Line::new_rounded(
            upper_right_line.beg, 
            upper_right_line.end, 
            constants::grab_item::THICKNESS * self.scale, 
            self.select_color, 
            self.pos.z - 0.001, 
            false);

        let lower_right_line = geometry::Line::new(
            local_pos,
            local_pos + self.scale / 2.0 * (Rotation2::new(2.0 * PI * (angular_dim.y + self.rot_angle)) * Vector2::x())
        );

        let mut lower_right_line = Line::new_rounded(
            lower_right_line.beg, 
            lower_right_line.end, 
            constants::grab_item::THICKNESS * self.scale, 
            self.select_color, 
            self.pos.z - 0.001, 
            false);


        let mut output = vec![
            GamePrimitive::Circ(background.into()),
            GamePrimitive::Circ(outer_circle.into()),
        ];

        output.append(&mut upper_right_line.get_primitives());
        output.append(&mut lower_right_line.get_primitives());
        output
    }

    fn get_paddle(&mut self) -> PaddleRect {
        let paddle_dim: Vector2<f64> = constants::grab_item::PADDLE_DIM.into();
        let paddle_pos: Vector2<f64> = constants::grab_item::PADDLE_POS.into();

        if self.paddle_spec.is_none() {
            self.paddle_spec = Some(build_grab_poly(paddle_dim * self.scale, 
            constants::grab_item::GRAB_RADIUS * self.scale, 
            paddle_pos * self.scale));
        }

        PaddleRect::new_with_color(
            self.paddle_spec.clone().unwrap().into(), 
            paddle_dim.x * self.scale, 
            self.pos + Vector3::new(0.0, 0.0, -0.01), 
            Rotation2::new(2.0 * PI * self.rot_angle), 
            self.select_color,
            10)
    }
}

impl Renderable<GamePrimitive> for GrabItem {
    fn get_primitives(&mut self) -> Vec<GamePrimitive> { 
        let mut output: Vec<GamePrimitive> = self.get_pacman_primitives();

        output.append(&mut self.get_paddle().get_primitives());
        output
    }
}

pub fn build_grab_poly(paddle_dim: Vector2<f64>, ball_radius: f64, paddle_pos: Vector2<f64>) -> Polynomial2d {
    let full_radius = paddle_dim.x / 2.0 + ball_radius;
    let ball_pos = Vector2::new(ball_radius, 0.0);
    let local_pos = ball_pos + paddle_pos;
    let subtended_half_angle = PI * paddle_dim.y / ( 2.0 * PI * full_radius);
    
    let lower_point = full_radius * Vector2::new(
        (PI + subtended_half_angle).cos(), 
        (PI + subtended_half_angle).sin()
        ) + local_pos;
    let lower_middle_point = full_radius * Vector2::new(
        (PI + subtended_half_angle / 3.0).cos(), 
        (PI + subtended_half_angle / 3.0).sin()
        ) + local_pos;
    let upper_middle_point = full_radius * Vector2::new(
        (PI - subtended_half_angle / 3.0).cos(), 
        (PI - subtended_half_angle / 3.0).sin()
        ) + local_pos;
    let upper_point = full_radius * Vector2::new(
        (PI - subtended_half_angle).cos(), 
        (PI - subtended_half_angle).sin()
        ) + local_pos;
    
    let knots = vec![0.0, 0.33, 0.66, 1.0];

    let values = vec![lower_point, lower_middle_point, upper_middle_point, upper_point];

    build_interpolating_poly2d(knots, values)
}