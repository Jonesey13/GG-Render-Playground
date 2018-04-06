use gg::rendering::{Rectangle};
use ::rendering::{RenderableTestRenderable, BoxBorder};
use na::{Vector2, Vector3, Vector4};

pub struct MenuBackground {
    rect: Rectangle
}

impl MenuBackground {
    pub fn new(pos: Vector3<f64>, dim: Vector2<f64>, colour: Vector4<f64>) -> Self {
        Self{
            rect: Rectangle::new_regular(dim.x, dim.y, pos, colour, true)
        }
    }

    pub fn build_border_box(&self) -> BoxBorder {
        BoxBorder::new(0.01, self.rect.pos, self.rect.height, self.rect.length, Vector4::new(1.0, 1.0, 1.0, 1.0), true)
    }

    pub fn render(&self) -> Vec<Box<RenderableTestRenderable>> {
        vec![
            Box::new(self.rect),
            Box::new(self.build_border_box())
        ]
    }
}