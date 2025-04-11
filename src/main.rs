#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod constants;
mod types;
mod game_state;
mod event_handler;

use ggez::{event, GameResult};
use std::path::PathBuf;
use crate::game_state::GameState;

fn main() -> GameResult {
    // Determine if running as a bundle (CARGO_MANIFEST_DIR not set)
    let is_bundle = std::env::var("CARGO_MANIFEST_DIR").is_err();

    let (mut ctx, event_loop) = ggez::ContextBuilder::new("Rust Snake Game", "Jacob Mish")
        .window_setup(ggez::conf::WindowSetup::default().title("Snake Game"))
        .window_mode(
            ggez::conf::WindowMode::default()
                .dimensions(constants::REFERENCE_WIDTH, constants::REFERENCE_HEIGHT)
                .min_dimensions(constants::MIN_WINDOW_WIDTH, constants::MIN_WINDOW_HEIGHT)
                .resizable(true)
                .transparent(true),
        )
        .add_resource_path(if let Ok(manifest_dir) = std::env::var("CARGO_MANIFEST_DIR") {
            let mut path = PathBuf::from(manifest_dir);
            path.push("resources");
            path
        } else {
            let mut path = PathBuf::new();
            if let Ok(exe_path) = std::env::current_exe() {
                path = exe_path;
                path.pop();
                if cfg!(target_os = "macos") {
                    path.pop();
                    path.push("Resources");
                }
            }
            if !path.ends_with("Resources") && !path.ends_with("resources") {
                path.push("resources");
            }
            path
        })
        .build()?;

    // Pass the is_bundle flag to GameState::new
    let state = GameState::new(&mut ctx, is_bundle)?;
    event::run(ctx, event_loop, state)
}
