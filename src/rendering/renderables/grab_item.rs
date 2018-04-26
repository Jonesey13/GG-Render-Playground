use na::{Vector2, Vector3, Vector4, Matrix2, Rotation2, convert};
use ::rendering::{SpikyCircle, CircleArrow, AnnularSegment, PaddleRect, PolyCubicControl, BezierCubicControl};
use ::rendering::{GamePrimitive };
use gg::rendering::{Renderable};
use gg::geometry::{Polynomial2d, build_interpolating_poly2d};
use ::constants;
use std::f64::consts::PI;

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
            Vector2::new(0.0, constants::grab_item::GRAB_RADIUS * self.scale),
            angular_dim.into(),
            self.pos,
            constants::grab_item::BACKGROUND_COLOUR.into(),
            false
        );

        vec![
            GamePrimitive::Circ(background.into())
        ]
    }

    fn get_paddle(&mut self) -> PaddleRect {
        let paddle_dim: Vector2<f64> = constants::grab_item::PADDLE_DIM.into();

        if self.paddle_spec.is_none() {
            self.paddle_spec = Some(build_grab_poly(paddle_dim * self.scale, constants::grab_item::GRAB_RADIUS * self.scale));
        }

        PaddleRect::new(
            self.paddle_spec.clone().unwrap().into(), 
            paddle_dim.x * self.scale, 
            self.pos, 
            Rotation2::new(2.0 * PI * self.rot_angle), 
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

pub fn build_grab_poly(paddle_dim: Vector2<f64>, ball_radius: f64) -> Polynomial2d {
    let full_radius = paddle_dim.x / 2.0 + ball_radius;
    let ball_pos = Vector2::new(0.0, 0.0);
    let subtended_half_angle = PI * paddle_dim.y / ( 2.0 * PI * full_radius);
    
    let lower_point = full_radius * Vector2::new(
        (PI + subtended_half_angle).cos(), 
        (PI + subtended_half_angle).sin()
        ) + ball_pos;
    let lower_middle_point = full_radius * Vector2::new(
        (PI + subtended_half_angle / 3.0).cos(), 
        (PI + subtended_half_angle / 3.0).sin()
        ) + ball_pos;
    let upper_middle_point = full_radius * Vector2::new(
        (PI - subtended_half_angle / 3.0).cos(), 
        (PI - subtended_half_angle / 3.0).sin()
        ) + ball_pos;
    let upper_point = full_radius * Vector2::new(
        (PI - subtended_half_angle).cos(), 
        (PI - subtended_half_angle).sin()
        ) + ball_pos;
    
    let knots = vec![0.0, 0.33, 0.66, 1.0];

    let values = vec![lower_point, lower_middle_point, upper_middle_point, upper_point];

    build_interpolating_poly2d(knots, values)
}