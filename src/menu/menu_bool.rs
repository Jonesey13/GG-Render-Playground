use gg::rendering::{PlainText, TextAlign};
use ::rendering::RenderableTestRenderable;
use na::{Vector2, Vector3, Matrix2};
use super::{MenuOption, MenuInput};
use ::constants;

pub struct MenuBool<Action> {
    index: usize,
    option_text: String,
    position: Vector3<f64>,
    size: f64,
    current_value: bool,
    _select_action: Option<Action>,
    true_text: String,
    false_text: String
}

impl<A> MenuBool<A> {
    pub fn new(
        index: usize,
        option_text: String, 
        position: Vector3<f64>, 
        size: f64, 
        initial_value: bool,
        true_text: String,
        false_text: String) -> Self {
        Self {
            index,
            option_text,
            position,
            size,
            current_value: initial_value,
            true_text,
            false_text,
            _select_action: None
        }
    }
    
    pub fn get_full_text(&self) -> String {
        let value_text = match self.current_value {
            true => self.true_text.clone(),
            false => self.false_text.clone()
        };
        self.option_text.clone() + ": " + &value_text
    }

    pub fn toggle(&mut self) {
        self.current_value = !self.current_value;
    }

    pub fn get_value(&self) -> bool {
        self.current_value
    }
}

impl<A> MenuOption for MenuBool<A> {
    type Action = A;

    fn render(&self, selected: bool)-> Vec<Box<RenderableTestRenderable>> {
        let text_object = PlainText {
            content: self.get_full_text(),
            position: self.position,
            scale: Vector2::new(self.size, self.size),
            transform: Matrix2::identity(),
            color: if selected { constants::menu::SELECTED_COLOUR.into() } else { constants::menu::UNSELECTED_COLOUR.into() },
            fixed: true,
            align: TextAlign::Centered
        };

        vec![Box::new(text_object)]
    }

    fn update(&mut self, selected: bool, menu_input: &MenuInput) -> Option<Self::Action> {
        if selected {
            if menu_input.left_switch.pressed() || menu_input.right_switch.pressed() {
                self.toggle();
            }
        }
        None
    }

    fn get_index(&self) -> usize {
        self.index
    }
}