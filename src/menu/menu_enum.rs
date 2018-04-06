use gg::rendering::{PlainText, TextAlign};
use ::rendering::RenderableTestRenderable;
use na::{Vector2, Vector3, Vector4, Matrix2};
use super::{MenuOption, MenuInput};

pub struct MenuEnum<E, A> {
    index: usize,
    option_text: String,
    position: Vector3<f64>,
    size: f64,
    current_value: E,
    list_of_values: Vec<E>,
    list_of_values_text: Vec<String>,
    select_action: Option<A>,
    update_action: Option<A>
}

impl<E: PartialEq + Eq + Clone, A> MenuEnum<E, A> {
    pub fn selected_colour() -> Vector4<f64> { Vector4::new(0.0, 1.0, 0.0, 1.0) }
    pub fn unselected_colour() -> Vector4<f64> { Vector4::new(1.0, 1.0, 1.0, 1.0) }

    pub fn new(
        index: usize,
        option_text: String, 
        position: Vector3<f64>, 
        size: f64, 
        current_value: E,
        list_of_values: Vec<E>,
        list_of_values_text: Vec<String>) -> Self {
        Self {
            index,
            option_text,
            position,
            size,
            current_value,
            list_of_values,
            list_of_values_text,
            select_action: None,
            update_action: None
        }
    }

    pub fn new_with_update_action(
        index: usize,
        option_text: String, 
        position: Vector3<f64>, 
        size: f64, 
        current_value: E,
        list_of_values: Vec<E>,
        list_of_values_text: Vec<String>,
        update_action: A) -> Self {
        Self {
            index,
            option_text,
            position,
            size,
            current_value,
            list_of_values,
            list_of_values_text,
            select_action: None,
            update_action: Some(update_action)
        }
    }
    
    pub fn get_full_text(&self) -> String {
        self.option_text.clone() + ": " + &format!("{}", self.list_of_values_text[self.get_current_index()])
    }

    pub fn get_value(&self) -> &E {
        &self.current_value
    }

    pub fn get_current_index(&self) -> usize {
        self.list_of_values.clone()
            .into_iter()
            .position(|value| {value == self.current_value})
            .unwrap()
    }

    pub fn increment_down(&mut self) {
        if self.get_current_index() != 0 {
            self.current_value = self.list_of_values[self.get_current_index() - 1].clone();
        }
    }

    pub fn increment_up(&mut self) {
        if self.get_current_index() + 1 != self.list_of_values.len() {
            self.current_value = self.list_of_values[self.get_current_index() + 1].clone();
        }
    }
}

impl<E: PartialEq + Eq + Clone, A: Clone> MenuOption for MenuEnum<E, A> {
    type Action = A;

    fn render(&self, selected: bool)-> Vec<Box<RenderableTestRenderable>> {
        let text_object = PlainText {
            content: self.get_full_text(),
            position: self.position,
            scale: Vector2::new(self.size, self.size),
            transform: Matrix2::identity(),
            color: if selected { MenuEnum::<E,A>::selected_colour() } else { MenuEnum::<E,A>::unselected_colour() },
            fixed: true,
            align: TextAlign::Center
        };

        vec![Box::new(text_object)]
    }

    fn update(&mut self, selected: bool, menu_input: &MenuInput) -> Option<Self::Action> {
        if selected {
            if menu_input.left_switch.pressed() {
                self.increment_down();
                return self.update_action.clone();
            }
            if menu_input.right_switch.pressed() {
                self.increment_up();
                return self.update_action.clone();
            }
            if selected && menu_input.select_switch.pressed() {
                return self.select_action.clone();
            }
        }
        None
    }

    fn get_select_action(&self) -> Option<Self::Action> { self.select_action.clone() }

    fn get_index(&self) -> usize {
        self.index
    }
}