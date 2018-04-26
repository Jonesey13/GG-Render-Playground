use ::animation::{AnimationSpec, Animatable, AnimationWindow, AnimationFunctionEnum, AnimationType};
use na::{Vector2, Vector3, Vector4, Rotation2};
use ::rendering::{RenderableTestRenderable, PaddleRect, BezierCubicControl, PolyCubicControl, Circle};
use ::constants;
use num::Zero;
use std::f64::consts::PI;
use gg::geometry::Polynomial2d;
use gg::geometry::build_interpolating_poly2d;

pub struct PongGrab {
    animation_spec: AnimationSpec<GrabPhase>,
    dim: Vector2<f64>,
    grab_back: f64,
    grab_angle: f64,
    grab_poly: Option<Polynomial2d>,
    ball_pos: Vector2<f64>,
    colour: Vector4<f64>
}

impl PongGrab {
    pub fn new() -> Self {
        Self {
            animation_spec: Self::build_animation_spec(),
            dim: constants::pong_grab::DIM.into(),
            grab_back: 0.0,
            grab_angle: 0.0,
            grab_poly: None,
            ball_pos: Vector2::x(),
            colour: constants::pong::PADDLE_COLOUR.into(),
        }
    }

    fn build_animation_spec() -> AnimationSpec<GrabPhase> {
        let approach_window = AnimationWindow::new(
            AnimationFunctionEnum::Linear, 0.3, GrabPhase::BallApproach
        );
        let hold_window = AnimationWindow::new(
            AnimationFunctionEnum::Linear, 1.0, GrabPhase::Grab
        );
        let release_window = AnimationWindow::new(
            AnimationFunctionEnum::Linear, 0.5, GrabPhase::Release
        );
        AnimationSpec::new(vec![approach_window, hold_window, release_window], AnimationType::Linear)
    }

    fn get_cubic(&self) -> PaddleRect {
        PaddleRect::new_with_color(
            self.get_control(), 
            self.dim.x, 
            Vector3::zero(), 
            Rotation2::new(0.0), 
            self.colour, 
            100)
    }

    fn get_ball(&self) -> Circle {
        Circle::new(
            constants::ball::RADIUS,
            Vector3::new(self.ball_pos.x, self.ball_pos.y, 0.0),
            constants::ball::BALL_COLOUR.into(),
            false
        )
    }

    fn get_control(&self) -> PolyCubicControl {
        match self.grab_poly {
            Some(ref poly) => poly.clone().into(),
            _ =>  {
                let start_pos = Vector2::new(0.0, -self.dim.y / 2.0);
                let start_middle_pos = Vector2::new(0.0, -self.dim.y / 4.0);
                let end_middle_pos = Vector2::new(0.0, self.dim.y / 4.0);
                let end_pos = Vector2::new(0.0, self.dim.y / 2.0);

                BezierCubicControl::new(start_pos, start_middle_pos, end_middle_pos, end_pos).into()
            }
        }
    }

    fn update_grab(&mut self) {
        let animation_time = self.animation_spec.get_current_time();
        let current_time = animation_time.stage_time;

        self.grab_poly = match animation_time.stage {
            GrabPhase::Grab => Some(build_grab_poly(self.dim, constants::ball::RADIUS)),
            GrabPhase::BallApproach => Some(build_grab_poly(self.dim, constants::pong_grab::GRAB_PRE_RADIUS)),
            _ => None
        };
    }

    fn update_ball_pos(&mut self) {
        let animation_time = self.animation_spec.get_current_time();
        let current_time = animation_time.stage_time;
        let full_radius = self.dim.x / 2.0 + constants::ball::RADIUS;
        let ball_start_pos: Vector2<f64> = constants::pong_grab::BALL_START_POS.into();

        self.ball_pos = match animation_time.stage {
            GrabPhase::BallApproach => current_time * full_radius * Vector2::x() + (1.0 - current_time) * ball_start_pos,
            GrabPhase::Grab =>  full_radius * Vector2::x(),
            GrabPhase::Release => (1.0 - current_time) * full_radius * Vector2::x() + current_time * ball_start_pos
        };
    }
}

impl Animatable for PongGrab {
    fn update(&mut self, t_step: f64) {
        self.animation_spec.update(t_step);
        self.update_grab();
        self.update_ball_pos();
    }

    fn render(&self) -> Vec<Box<RenderableTestRenderable>> {
        vec![
            Box::new(self.get_cubic()),
            Box::new(self.get_ball())
        ]
    }

    fn get_name(&self) -> String {"Pong Grab".to_string()}

    fn reset(&mut self) {
        self.animation_spec.reset()
    }
}

#[derive(Debug, Clone)]
pub enum GrabPhase {
    BallApproach,
    Grab,
    Release
}

pub fn build_grab_poly(paddle_dim: Vector2<f64>, ball_radius: f64) -> Polynomial2d {
    let full_radius = paddle_dim.x / 2.0 + ball_radius;
    let ball_pos = Vector2::new(full_radius, 0.0);
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