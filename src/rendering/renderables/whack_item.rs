use na::{Vector2, Vector3, Vector4, Matrix2, Rotation2, convert};
use ::rendering::{SpikyCircle, CircleArrow};
use ::rendering::{GamePrimitive };
use gg::rendering::{Renderable};
use ::constants;

#[derive(Copy, Clone, Debug)]
pub struct WhackItem {
    pos: Vector3<f64>,
    scale: f64,
    select_color: Vector4<f64>,
    rot_angle: f64
}

impl WhackItem {
    pub fn new(
        pos: Vector3<f64>,
        scale: f64,
        rot_angle: f64
    ) -> Self {
        Self {
            pos,
            scale,
            select_color: constants::whack_item::FOREGROUND_COLOUR.into(),
            rot_angle
        }
    }

    fn get_spiky_circle(&self) -> SpikyCircle {
        SpikyCircle::new_with_boundary(
            self.pos,
            self.scale / 2.0,
            self.scale * constants::whack_item::SPIKE_SIZE,
            constants::whack_item::NUM_SPIKES,
            self.rot_angle,
            constants::whack_item::BACKGROUND_COLOUR.into(),
            self.scale * constants::whack_item::THICKNESS,
            self.select_color,
            false
        )
    }

    fn get_icon(&self) -> CircleArrow {
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

impl Renderable<GamePrimitive> for WhackItem {
    fn get_primitives(&mut self) -> Vec<GamePrimitive> { 
        let mut output: Vec<GamePrimitive> = self.get_spiky_circle().get_primitives();

        output.append(&mut self.get_icon().get_primitives());
        output
    }
}