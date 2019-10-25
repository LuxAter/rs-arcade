#[macro_use]
extern crate derive_builder;

use std::{env, path};

use ggez::event;
use ggez::graphics;
// use ggez::nalgebra as na;
use ggez::{Context, ContextBuilder, GameResult};

mod config;
mod ui;

pub struct GameState {
    game_state: u32,
    pub settings: config::Config,
    pub fonts: Vec<graphics::Font>,
    pub ui: ui::Ui,
}

impl GameState {
    fn new(ctx: &mut Context, settings: config::Config) -> GameResult<GameState> {
        let s = GameState {
            game_state: 1,
            settings: settings,
            fonts: vec![graphics::Font::new(ctx, "/font/PressStart2P-Regular.ttf")?],
            ui: ui::UiBuilder::default().build().unwrap(),
        };
        Ok(s)
    }
}

fn load_main_menu(state: &mut GameState) {
    state.ui = ui::UiBuilder::default()
        .push(
            ui::UiBuilder::default()
                .size([0.8, 0.1])
                .origin([0.1, 0.05])
                .text("RS-ARCADE".to_string())
                .align(graphics::Align::Center)
                .font_size(32.0)
                .font(Some(state.fonts[0]))
                .color([1.0, 0.0, 0.0, 1.0])
                .color_hover([0.0, 1.0, 0.0, 1.0])
                .color_click([0.0, 0.0, 1.0, 1.0])
                .background([0.1, 0.1, 0.1, 0.5])
                .background_hover([0.2, 0.2, 0.2, 0.5])
                .background_click([0.3, 0.3, 0.3, 0.5])
                .callback(Some(|state| state.game_state = 2))
                .build()
                .unwrap(),
        )
        .build()
        .unwrap();
}

impl ggez::event::EventHandler for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        const DESIRED_FPS: u32 = 60;
        while ggez::timer::check_update_time(ctx, DESIRED_FPS) {
            let _seconds = 1.0 / (DESIRED_FPS as f32);
            println!("{:?}", self.game_state);
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::BLACK);
        self.ui.draw(ctx, None)?;
        graphics::present(ctx)?;
        ggez::timer::yield_now();
        Ok(())
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        key: event::KeyCode,
        _mods: event::KeyMods,
        _: bool,
    ) {
        match key {
            event::KeyCode::Escape => {
                event::quit(ctx);
            }
            event::KeyCode::F => {
                self.settings.window.fullscreen =
                    Some(!self.settings.window.fullscreen.unwrap_or(false));
                self.settings.write().expect("Failed to write config");
            }
            _ => (),
        }
    }

    fn resize_event(&mut self, ctx: &mut Context, width: f32, height: f32) {
        graphics::set_screen_coordinates(ctx, graphics::Rect::new(0.0, 0.0, width, height))
            .unwrap();
    }
}

pub fn main() -> GameResult {
    if cfg!(unix) {
        // TODO: Remove when proper wayland support is added <23-10-19, Arden Rasmussen>
        env::set_var("WINIT_UNIX_BACKEND", "x11");
    }
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };
    let settings = config::load(&format!(
        "{}/{}",
        resource_dir.to_str().unwrap(),
        "settings.json"
    ));
    let cb = ContextBuilder::new("Rcade", "Arden Rasmussen")
        .window_setup(ggez::conf::WindowSetup::default().title("Rcade"))
        .window_mode(
            ggez::conf::WindowMode::default()
                .dimensions(settings.window.res[0], settings.window.res[1])
                .fullscreen_type(if settings.window.fullscreen.unwrap_or(false) {
                    ggez::conf::FullscreenType::True
                } else {
                    ggez::conf::FullscreenType::Windowed
                })
                .borderless(settings.window.borderless.unwrap_or(false))
                .resizable(settings.window.resizable.unwrap_or(false)),
        )
        .add_resource_path(resource_dir);
    let (ctx, events_loop) = &mut cb.build()?;
    let game = &mut GameState::new(ctx, settings)?;
    load_main_menu(game);
    ggez::event::run(ctx, events_loop, game)
}
