use gg::rendering::{PlainText, TextAlign};
use ::rendering::RenderableTestRenderable;
use na::{Vector2, Vector3, Vector4, Matrix2};
use super::{MenuOption, MenuInput};

pub struct MenuRange<A> {
    index: usize,
    option_text: String,
    position: Vector3<f64>,
    size: f64,
    current_value: isize,
    limits: Vector2<isize>,
    select_action: Option<A>,
    update_action: Option<A>
}

impl<A> MenuRange<A> {
    pub fn selected_colour() -> Vector4<f64> { Vector4::new(0.0, 1.0, 0.0, 1.0) }
    pub fn unselected_colour() -> Vector4<f64> { Vector4::new(1.0, 1.0, 1.0, 1.0) }

    pub fn new(
        index: usize,
        option_text: String, 
        position: Vector3<f64>, 
        size: f64, 
        initial_value: isize,
        limits: Vector2<isize>) -> Self {
        Self {
            index,
            option_text,
            position,
            size,
            current_value: initial_value,
            limits,
            select_action: None,
            update_action: None
        }
    }

    pub fn new_with_update_action(
        index: usize,
        option_text: String, 
        position: Vector3<f64>, 
        size: f64, 
        initial_value: isize,
        limits: Vector2<isize>,
        update_action: A) -> Self {
        Self {
            index,
            option_text,
            position,
            size,
            current_value: initial_value,
            limits,
            select_action: None,
            update_action: Some(update_action)
        }
    }
    
    pub fn get_full_text(&self) -> String {
        self.option_text.clone() + ": " + &format!("{}", self.current_value)
    }

    pub fn increment_down(&mut self) {
        if self.current_value - 1 != self.limits.x {
            self.current_value -= 1;
        }
    }

    pub fn increment_up(&mut self) {
        if self.current_value + 1 != self.limits.y {
            self.current_value += 1;
        }
    }

    pub fn get_value(&self) -> isize {
        self.current_value
    }
}

impl<A: Clone> MenuOption for MenuRange<A> {
    type Action = A;

    fn render(&self, selected: bool)-> Vec<Box<RenderableTestRenderable>> {
        let text_object = PlainText {
            content: self.get_full_text(),
            position: self.position,
            scale: Vector2::new(self.size, self.size),
            transform: Matrix2::identity(),
            color: if selected { MenuRange::<A>::selected_colour() } else { MenuRange::<A>::unselected_colour() },
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