use ::animation::{AnimationSpec, Animatable, AnimationWindow, AnimationFunctionEnum, AnimationType};
use na::{Vector2, Vector3, Vector4, Rotation2};
use ::rendering::{RenderableTestRenderable, CubicRect, BezierCubicControl};
use ::constants;
use num::Zero;
use std::f64::consts::PI;

pub struct PongFlip {
    animation_spec: AnimationSpec<FlipPhase>,
    dim: Vector2<f64>,
    flip_back: f64,
    flip_angle: f64,
    colour: Vector4<f64>
}

impl PongFlip {
    pub fn new() -> Self {
        Self {
            animation_spec: Self::build_animation_spec(),
            dim: constants::pong_flip::DIM.into(),
            flip_back: 0.0,
            flip_angle: 0.0,
            colour: constants::pong::PADDLE_COLOUR.into(),
        }
    }

    fn build_animation_spec() -> AnimationSpec<FlipPhase> {
        let normal_window = AnimationWindow::new(
            AnimationFunctionEnum::Linear, 0.3, FlipPhase::Normal
        );
        let flip_back_window = AnimationWindow::new(
            AnimationFunctionEnum::Linear, constants::pong_flip::FLIP_BACK_TIME, FlipPhase::FlipBack
        );
        let hold_window = AnimationWindow::new(
            AnimationFunctionEnum::Linear, 0.5, FlipPhase::Hold
        );
        let flip_window = AnimationWindow::new(
            AnimationFunctionEnum::Linear, constants::pong_flip::FLIP_DURATION, FlipPhase::Flip
        );
        AnimationSpec::new(vec![normal_window, flip_back_window, hold_window, flip_window], AnimationType::Linear)
    }

    fn get_cubic(&self) -> CubicRect {
        let start_pos = Vector2::new(0.0, -self.dim.y / 2.0);
        let start_middle_pos = Vector2::new(0.0, -self.dim.y / 4.0);
        let end_middle_pos = Vector2::new(0.0, self.dim.y / 4.0);
        let end_pos = Vector2::new(-self.flip_back, self.dim.y / 2.0);

        let control = BezierCubicControl::new(start_pos, start_middle_pos, end_middle_pos, end_pos);

        CubicRect::new_with_color(
            control, 
            self.dim.x, 
            Vector3::zero(), 
            Rotation2::new(2.0 * PI * self.flip_angle), 
            self.colour, 
            100)
    }

    fn update_flip(&mut self) {
        let animation_time = self.animation_spec.get_current_time();
        let current_time = animation_time.stage_time;

        self.flip_back = match animation_time.stage {
            FlipPhase::Normal => 0.0,
            FlipPhase::FlipBack => current_time * constants::pong_flip::FLIP_BACK_DIST,
            FlipPhase::Hold => constants::pong_flip::FLIP_BACK_DIST,
            FlipPhase::Flip => (1.0 - current_time) * constants::pong_flip::FLIP_BACK_DIST
        };

        self.flip_angle = match animation_time.stage {
            FlipPhase::Normal => 0.0,
            FlipPhase::FlipBack => 0.0,
            FlipPhase::Hold => 0.0,
            FlipPhase::Flip => -current_time / 2.0
        };
    }
}

impl Animatable for PongFlip {
    fn update(&mut self, t_step: f64) {
        self.animation_spec.update(t_step);
        self.update_flip();
    }

    fn render(&self) -> Vec<Box<RenderableTestRenderable>> {
        vec![
            Box::new(self.get_cubic())
        ]
    }

    fn get_name(&self) -> String {"Pong Flip".to_string()}

    fn reset(&mut self) {
        self.animation_spec.reset()
    }
}

#[derive(Debug, Clone)]
pub enum FlipPhase {
    Normal,
    FlipBack,
    Hold,
    Flip
}