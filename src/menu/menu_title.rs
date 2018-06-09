use gg::rendering::{PlainText, TextAlign};
use ::rendering::RenderableTestRenderable;
use na::{Vector2, Vector3, Vector4, Matrix2};

pub struct MenuTitle {
    title_text: String,
    position: Vector3<f64>,
    size: f64,
    colour: Vector4<f64>
}

impl MenuTitle {
    pub fn new(title_text: String, position: Vector3<f64>, size: f64, colour: Vector4<f64>) -> Self {
        Self {
            title_text,
            position,
            size,
            colour
        }
    }

    pub fn to_renderables(&self) -> Vec<Box<RenderableTestRenderable>> {
        let text_object = PlainText {
            content: self.title_text.clone(),
            position: self.position,
            scale: Vector2::new(self.size, self.size),
            transform: Matrix2::identity(),
            color: self.colour,
            fixed: true,
            align: TextAlign::Centered
        };

        vec![Box::new(text_object)]
    }
}