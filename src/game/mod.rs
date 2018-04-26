use gg::games::Game;
use gg::games::GameInput;
use gg::games::view_details::{ViewDetails2D, ViewDetails};
use na::{Vector2, Vector3, Vector4, Rotation2};
use ::rendering::{BezierRect, BezierQuadControl, BezierBranchRect, BezierBranchCirc, PaddleRect, BezierCubicControl};
use gg::rendering::renderables::BoxBorder;
use gg::rendering::{PlainText, TextAlign, Circle, Annulus, Rectangle, Renderable, Polygon, Arrow, TextureRect};
use gg::rendering::{WindowSpec, DisplaySettings};
use gg::games::view_details;
use ::rendering::{GamePrimitive, RenderableTestRenderable};
use ::common::{GameSettings};
use ::main_menu_mode::MainMenuMode;

mod game_mode;

pub use self::game_mode::{GameModeEnum, GameMode};

pub struct RenderableTestGame {
    current_game_mode: Box<GameMode>,
    current_game_mode_enum: GameModeEnum
}

impl RenderableTestGame {
    pub fn new(display_settings: DisplaySettings) -> Self {
        let settings = GameSettings::new_with_display_settings(display_settings);

        RenderableTestGame {
            current_game_mode: Box::new(MainMenuMode::new(settings.clone())),
            current_game_mode_enum: GameModeEnum::Menu(settings)
        }
    }

    pub fn get_mode(&self) -> &Box<GameMode> {
        &self.current_game_mode
    }
    
    pub fn get_mode_mut(&mut self) -> &mut Box<GameMode> {
        &mut self.current_game_mode
    }

    pub fn switch_mode(&mut self, mode_enum: GameModeEnum) {
        self.current_game_mode_enum = mode_enum.clone();
        self.current_game_mode = mode_enum.into();
        self.current_game_mode.init();
    }
}

impl Game for RenderableTestGame {
    type Primitive = GamePrimitive;

    fn update_input(&mut self) {
        self.get_mode_mut().update_input()
    }

    #[allow(unused_variables)]
    fn update_logic(&mut self, t_step: f64) {
        if let Some(mode_enum) = self.get_mode().change_mode() {
            self.switch_mode(mode_enum);
        }
        self.get_mode_mut().update_logic(t_step)
    }

    fn get_renderables(&mut self, _: WindowSpec) -> Vec<Box<RenderableTestRenderable>> { 
        let output = self.get_mode().get_renderables();
        output
    }

    fn get_input<'a>(&'a mut self) -> Option<&'a mut GameInput> {
        self.get_mode_mut().get_input()
    }

    fn get_view(&self) -> view_details::ViewDetails {
        self.get_mode().get_view()
    }

    fn on_exit(&mut self) {}

    fn should_exit(&self) -> bool { self.current_game_mode.should_exit() }

    fn get_console_logs(&mut self) -> Vec<String> { vec![] }

    fn change_display_settings(&mut self) -> Option<DisplaySettings> {self.get_mode_mut().change_display_settings()}
}