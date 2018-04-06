pub mod menu_input;
pub mod menu_option;
pub mod menu_clickable;
pub mod menu_background;
pub mod menu_title;
pub mod menu_range;
pub mod menu_bool;
pub mod standard_menu;
pub mod menu_enum;
pub mod menu_mode;

pub use self::menu_option::MenuOption;
pub use self::menu_clickable::MenuClickable;
pub use self::menu_input::MenuInput;
pub use self::menu_background::MenuBackground;
pub use self::menu_title::MenuTitle;
pub use self::menu_range::MenuRange;
pub use self::standard_menu::StandardMenu;
pub use self::menu_bool::MenuBool;
pub use self::menu_enum::MenuEnum;
pub use self::menu_mode::MenuMode;

use ::rendering::RenderableTestRenderable;
use ::input::ExternalInput;

pub trait Menu {
    type Action;

    fn get_options(&self) -> Vec<&MenuOption<Action=Self::Action>> { vec![] }

    fn get_options_mut(&mut self) -> Vec<&mut MenuOption<Action=Self::Action>> { vec![] }

    fn get_input_mut(&mut self) -> &mut MenuInput;

    fn get_current_selection(&self) -> usize;

    fn set_current_selection(&mut self, index: usize);

    fn get_background(&self) -> Option<&MenuBackground> { None }

    fn get_title(&self) -> Option<&MenuTitle> { None }

    fn render(&self) -> Vec<Box<RenderableTestRenderable>> { 
        let mut output = vec![];

        if let Some(ref background) = self.get_background() {
            output.extend(background.render());
        }

        if let Some(ref title) = self.get_title() {
            output.extend(title.to_renderables());
        }

        let current_selection = self.get_current_selection();
        let option_renderables: Vec<Box<RenderableTestRenderable>> = self.get_options()
        .iter()
        .flat_map(|opt| {
            let index = opt.get_index();
            opt.render(index == current_selection)
        })
        .collect();

        output.extend(option_renderables);
        output
     }

    fn process_input(&mut self, external_input: &ExternalInput) {
        self.get_input_mut().update_switches(&external_input)
    }

    fn move_down(&mut self) {
        let total_options = self.get_options().len();
        let current_selection = self.get_current_selection();

        if current_selection == total_options - 1 {
            self.set_current_selection(0);
        }
        else {
            self.set_current_selection(current_selection + 1);
        }
    }

    fn move_up(&mut self) {
        let total_options = self.get_options().len();
        let current_selection = self.get_current_selection();

        if current_selection == 0 {
            self.set_current_selection(total_options - 1);
        }
        else {
            self.set_current_selection(current_selection - 1);
        }
    }

    fn update(&mut self, _t_step: f64) -> Option<Self::Action> {
        let current_menu_input = self.get_input_mut().clone(); 
        let down_switch = current_menu_input.down_switch.clone();
        let up_switch = current_menu_input.up_switch.clone();

        if down_switch.pressed() {
            self.move_down();
        }
        if up_switch.pressed() {
            self.move_up();
        }

        let current_selection = self.get_current_selection();

        let mut return_action: Option<Self::Action> = None;
        for option in self.get_options_mut().iter_mut() {
            let index = option.get_index();
            if let Some(action) = option.update(index == current_selection, &current_menu_input) {
                return_action = Some(action);
                break;
            }
        }

        self.get_input_mut().clear_switches();
        return_action
     }
}