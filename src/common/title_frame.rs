use gg::rendering::{PlainText, TextAlign};
use ::rendering::RenderableTestRenderable;
use na::{Vector2, Vector3, Vector4, Matrix2};

pub struct TitleFrame {
    text: String,
    text_colour: Vector4<f64>,
    position: Vector2<f64>,
    size: f64,
}

impl TitleFrame {
    pub fn new(text: String, text_colour: Vector4<f64>, position: Vector2<f64>, size: f64) -> Self {
        Self {
            text,
            text_colour,
            position,
            size,
        }
    }

    pub fn to_renderables(&self) -> Vec<Box<RenderableTestRenderable>> {
        let text_object = PlainText {
            content: self.text.clone(),
            position: Vector3::new(self.position.x, self.position.y, 0.0),
            scale: Vector2::new(self.size, self.size), 
            transform: Matrix2::identity(),
            color: self.text_colour,
            fixed: true,
            align: TextAlign::Centered
        };

        vec![Box::new(text_object)]
    }
}