use gg::rendering::{PlainText, TextAlign};
use ::rendering::RenderableTestRenderable;
use na::{Vector2, Vector3, Vector4, Matrix2};
use super::{MenuOption, MenuInput};

pub struct MenuClickable<T> {
    index: usize,
    option_text: String,
    position: Vector3<f64>,
    size: f64,
    pub action: T
}

impl<T> MenuClickable<T> {
    pub fn selected_colour() -> Vector4<f64> { Vector4::new(0.0, 1.0, 0.0, 1.0) }
    pub fn unselected_colour() -> Vector4<f64> { Vector4::new(1.0, 1.0, 1.0, 1.0) }

    pub fn new(index: usize, option_text: String, position: Vector3<f64>, size: f64, action: T) -> Self {
        Self {
            index,
            option_text,
            position,
            size,
            action
        }
    }
}

impl<T: Clone> MenuOption for MenuClickable<T> {
    type Action = T;

    fn render(&self, selected: bool)-> Vec<Box<RenderableTestRenderable>> {
        let text_object = PlainText {
            content: self.option_text.clone(),
            position: self.position,
            scale: Vector2::new(self.size, self.size),
            transform: Matrix2::identity(),
            color: if selected { MenuClickable::<T>::selected_colour() } else { MenuClickable::<T>::unselected_colour() },
            fixed: true,
            align: TextAlign::Centered
        };

        vec![Box::new(text_object)]
    }

    fn update(&mut self, selected: bool, menu_input: &MenuInput) -> Option<Self::Action> {
        if selected && menu_input.select_switch.released() {
            return Some(self.action.clone());
        }
        None
    }

    fn get_index(&self) -> usize {
        self.index
    }
}