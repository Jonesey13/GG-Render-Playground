use ::common::GameSettings;
use ::rendering::RenderableTestRenderable;
use gg::rendering::{DisplaySettings};
use gg::games::GameInput;
use gg::games::view_details::{ViewDetails, ViewDetails2D};

#[derive(Clone, Debug)]
pub enum GameModeEnum {
    Menu(GameSettings),
    RenderableTest(GameSettings),
    AnimationMenu(GameSettings),
}

#[derive(Clone, Debug)]
pub enum MenuEnum {
    AnimationMenu,
    MainMenu
}

pub trait GameMode {
    fn init(&mut self) {}
    fn update_input(&mut self) {}
    fn update_logic(&mut self, _t_step: f64) {}
    fn update_rendering(&mut self) {}
    fn get_renderables(&self) -> Vec<Box<RenderableTestRenderable>> { vec![] }
    fn get_input<'a>(&'a mut self) -> Option<&'a mut GameInput> { None }
    fn change_mode(&self) -> Option<GameModeEnum> { None }
    fn should_exit(&self) -> bool { false }
    fn change_display_settings(&mut self) -> Option<DisplaySettings> {None}
    fn get_view(&self) -> ViewDetails {
        ViewDetails::TwoDim(ViewDetails2D::default())
    }
}

impl From<GameModeEnum> for Box<GameMode> {
    fn from(game_mode_enum: GameModeEnum) -> Self {
        match game_mode_enum {
            GameModeEnum::Menu(settings) => Box::new(::main_menu_mode::MainMenuMode::new(settings)),
            GameModeEnum::RenderableTest(_) => Box::new(::renderable_test_mode::RenderableTestMode::default()),
            GameModeEnum::AnimationMenu(settings) => Box::new(::animation_test_mode::AnimationMode::new(settings))
        }
    }
}