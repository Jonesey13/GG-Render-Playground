use ::rendering::{RenderableTestRenderable};
use super::MenuInput;

pub trait MenuOption {
    type Action;

    fn render(&self, selected: bool) -> Vec<Box<RenderableTestRenderable>>;
    fn update(&mut self, _selected: bool, &MenuInput) -> Option<Self::Action> {None}
    fn get_select_action(&self) -> Option<Self::Action> {None}
    fn get_index(&self) -> usize;
}