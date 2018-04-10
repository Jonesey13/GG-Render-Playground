pub mod animation_window;
pub mod animation_spec;
pub mod animation_functions;
pub mod animation_type;
pub mod animation_function_enum;
pub mod animation_time;

pub use self::animation_window::AnimationWindow;
pub use self::animation_spec::AnimationSpec;
pub use self::animation_type::AnimationType;
pub use self::animation_function_enum::AnimationFunctionEnum;
pub use self::animation_time::AnimationTime;

use ::rendering::RenderableTestRenderable;

pub trait Animatable {
    fn update(&mut self, _t_step: f64) {

    }

    fn render(&self) -> Vec<Box<RenderableTestRenderable>>;

    fn reset(&mut self) {}

    fn get_name(&self) -> String {"Unnamed Animation".to_string()}
}