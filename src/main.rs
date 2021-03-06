#![feature(set_stdio)]
#![feature(conservative_impl_trait)]
#![feature(const_fn)]
extern crate generic_game as gg;
extern crate nalgebra as na;
extern crate time;
extern crate num;
extern crate rand;
extern crate image;
#[macro_use]
extern crate glium;
extern crate rusttype;

use gg::debug::*;
use gg::{debug, window, handler_basic, games, Handler};
use gg::rendering::{DisplaySettings};
use ::rendering::GamePrimitive;
use std::env;
use std::io::*;
mod game;
mod rendering;
mod main_menu_mode;
mod menu;
mod common;
mod animation;
mod input;
mod constants;
mod renderable_test_mode;
mod animation_test_mode;

fn main() {
    env::set_var("RUST_BACKTRACE", "full");
    debug::set_flags(DebugFlags::DEFAULTDEBUG);
    debug(&format!("Starting Up - Date: {}", time::now_utc().ctime()));
    let error_writer = Box::new(ErrorWriter::new());
    set_panic(Some(error_writer));

    let display_settings = DisplaySettings {
        res: (1920, 1080),
        fullscreen: false,
        text_glyph_detail: 128.0,
            ..Default::default()
    };

    let image1 = image::load(Cursor::new(&include_bytes!("Racing2.png")[..]),
                        image::PNG).unwrap();
    let texture_array = vec![image1];

    let renderer = Box::new(rendering::GliumRenderer::new_with_textures(display_settings, texture_array));

    let input_handler: Box<gg::input::InputHandler> = Box::new(gg::input::multihandler::MultiInput::new());
    let window_handler: Box<window::WindowHandler> = Box::new(window::GlutinInput::new());

    let game: Box<games::Game<Primitive=GamePrimitive>> = Box::new(game::RenderableTestGame::new(display_settings));
    let mut handler: Box<Handler> = Box::new(handler_basic::HandlerBasic::new(renderer, input_handler, window_handler, game));

    handler.init();
    while !handler.should_exit() {
        debug_clock_start_main();
        handler.update_input();
        handler.update_rendering();
        handler.update_logic();
        debug_clock_stop_main();
    }
    handler.on_exit();
}
