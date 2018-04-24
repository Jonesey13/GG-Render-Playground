use na::{Vector2, Vector3, Vector4, Matrix2, Rotation2, convert};
use ::rendering::{Line};
use ::rendering::{GamePrimitive };
use gg::rendering::{Renderable, Rectangle};
use std::f64::consts::PI;
use ::constants;

#[derive(Copy, Clone, Debug)]
pub struct StretchItem {
    pos: Vector3<f64>,
    scale: f64,
    select_color: Vector4<f64>,
    rot_angle: f64
}

impl StretchItem {
    pub fn new(
        pos: Vector3<f64>,
        scale: f64,
        rot_angle: f64
    ) -> Self {
        Self {
            pos,
            scale,
            select_color: constants::stretch_item::FOREGROUND_COLOUR.into(),
            rot_angle
        }
    }

    fn get_plus_sign_primitives(&self) -> Vec<GamePrimitive> {
        let hori_rect = Rectangle::new_with_rotation(
            constants::stretch_item::DEFAULT_SIZE, 
            constants::stretch_item::RECT_WIDTH, 
            self.pos,
            Rotation2::new(2.0 * PI * self.rot_angle),
            self.select_color,
            false
        );

        let vert_rect = Rectangle::new_with_rotation(
            constants::stretch_item::DEFAULT_SIZE, 
            constants::stretch_item::RECT_WIDTH, 
            self.pos,
            Rotation2::new(2.0 * PI * (self.rot_angle + 0.25)),
            self.select_color,
            false
        );

        vec![
            GamePrimitive::Rect(hori_rect),
            GamePrimitive::Rect(vert_rect)
        ]
    }

    fn get_icon_primitives(&self) -> Vec<GamePrimitive> {
        let icon_scale = self.scale * constants::whack_item::ICON_SCALE;
        let arrow_dim: Vector2<f64> = constants::whack_item::ARROW_DIM.into();

        CircleArrow::new_clockwise(
            icon_scale / 2.0,
            constants::whack_item::ARROW_ANGLE_DIM.into(),
            icon_scale * arrow_dim,
            self.pos + Vector3::new(0.0, 0.0, -0.02),
            self.select_color,
            icon_scale * constants::whack_item::ICON_THICKNESS,
            false
        )
    }
}

impl Renderable<GamePrimitive> for StretchItem {
    fn get_primitives(&mut self) -> Vec<GamePrimitive> { 
        let mut output: Vec<GamePrimitive> = self.get_plus_sign_primitives();

        output.append(&mut self.get_icon_primitives());
        output
    }
}