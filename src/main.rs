use std::collections::HashMap;
use std::sync::RwLock;
use std::{env, path};

use ggez::event;
use ggez::graphics;
use ggez::nalgebra as na;
use ggez::{Context, ContextBuilder, GameResult};

mod config;

struct GameState {
    game_state: u32,
    pub settings: config::Config,
}

impl GameState {
    fn new(ctx: &mut Context, settings: config::Config) -> GameResult<GameState> {
        let s = GameState {
            game_state: 1,
            settings: settings,
        };
        Ok(s)
    }
}

impl ggez::event::EventHandler for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        const DESIRED_FPS: u32 = 60;
        while ggez::timer::check_update_time(ctx, DESIRED_FPS) {
            let seconds = 1.0 / (DESIRED_FPS as f32);
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::BLACK);
        graphics::present(ctx)?;
        ggez::timer::yield_now();
        Ok(())
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        key: event::KeyCode,
        mods: event::KeyMods,
        _: bool,
    ) {
        match key {
            event::KeyCode::Escape => {
                println!("TERMINATING");
                event::quit(ctx);
            }
            event::KeyCode::F => {
                self.settings.window.fullscreen =
                    Some(!self.settings.window.fullscreen.unwrap_or(false));
                self.settings.write().expect("Failed to write config");
                println!("TOGGLE FULLSCREEN");
            }
            _ => (),
        }
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
    let mut settings = config::load(&format!(
        "{}/{}",
        resource_dir.to_str().unwrap(),
        "settings.toml"
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
    ggez::event::run(ctx, events_loop, game)
}
