use ::animation::{AnimationSpec, Animatable, AnimationWindow, AnimationFunctionEnum, AnimationType};
use na::{Vector2, Vector3, Vector4, Rotation2};
use ::rendering::{RenderableTestRenderable, CubicRect, BezierCubicControl};
use ::constants;
use num::Zero;

pub struct PongStretch {
    animation_spec: AnimationSpec<StretchPhase>,
    original_dim: Vector2<f64>,
    stretch_dim: Vector2<f64>,
    current_dim: Vector2<f64>,
    colour: Vector4<f64>
}

impl PongStretch {
    pub fn new() -> Self {
        Self {
            animation_spec: Self::build_animation_spec(),
            original_dim: constants::pong_stretch::ORIGINAL_DIM.into(),
            stretch_dim: constants::pong_stretch::STRETCH_DIM.into(),
            current_dim: Vector2::zero(),
            colour: constants::pong::PADDLE_COLOUR.into(),
            
        }
    }

    fn build_animation_spec() -> AnimationSpec<StretchPhase> {
        let normal_window = AnimationWindow::new(
            AnimationFunctionEnum::Linear, 0.3, StretchPhase::Normal
        );
        let stretch_out_window = AnimationWindow::new(
            AnimationFunctionEnum::Linear, 1.0, StretchPhase::StretchOut
        );
        let stretched_window = AnimationWindow::new(
            AnimationFunctionEnum::Linear, 0.5, StretchPhase::Hold
        );
        let return_window = AnimationWindow::new(
            AnimationFunctionEnum::Linear, 1.0, StretchPhase::StretchBack
        );
        AnimationSpec::new(vec![normal_window, stretch_out_window, stretched_window, return_window], AnimationType::Linear)
    }

    fn update_current_dim(&mut self) {
        let animation_time = self.animation_spec.get_current_time();
        let current_time = animation_time.stage_time;
        self.current_dim = match animation_time.stage {
            StretchPhase::Normal => self.original_dim,
            StretchPhase::StretchBack => current_time * self.original_dim + (1.0 - current_time) * self.stretch_dim,
            StretchPhase::Hold => self.stretch_dim,
            StretchPhase::StretchOut => (1.0 - current_time) * self.original_dim + current_time * self.stretch_dim
        };
    }

    fn get_cubic(&self) -> CubicRect {
        let start_pos = Vector2::new(0.0, -self.current_dim.y / 2.0);
        let change = Vector2::new(0.0, self.current_dim.y);
        let control = BezierCubicControl::new_linear(start_pos, change);

        CubicRect::new_with_color(
            control.into(), 
            self.current_dim.x, 
            Vector3::zero(), 
            Rotation2::new(0.0), 
            self.colour, 
            100)
    }
}

impl Animatable for PongStretch {
    fn update(&mut self, t_step: f64) {
        self.animation_spec.update(t_step);
        self.update_current_dim();
    }

    fn render(&self) -> Vec<Box<RenderableTestRenderable>> {
        vec![
            Box::new(self.get_cubic())
        ]
    }

    fn get_name(&self) -> String {"Pong Stretch".to_string()}

    fn reset(&mut self) {
        self.animation_spec.reset()
    }
}

#[derive(Debug, Clone)]
pub enum StretchPhase {
    Normal,
    StretchOut,
    Hold,
    StretchBack
}