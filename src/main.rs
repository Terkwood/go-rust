extern crate ggez;
#[macro_use]
extern crate lazy_static;

mod draw;
mod game;
mod pixel_math;

use game::{MainState, BOARD_SIZE};

use ggez::conf;
use ggez::event;
use ggez::graphics;
use ggez::{Context, ContextBuilder};
use std::env;
use std::path;

fn main() {
    let cb = ContextBuilder::new("Go", "ggez")
        .window_setup(conf::WindowSetup::default().title("Go: The Way of the Warrior"))
        .window_mode(conf::WindowMode::default().dimensions(
            pixel_math::SCREEN_SIZE.0 as u32,
            pixel_math::SCREEN_SIZE.1 as u32,
        ));

    let ctx = &mut cb.build().unwrap();

    // We add the CARGO_MANIFEST_DIR/resources do the filesystems paths so
    // we we look in the cargo project for files.
    if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        ctx.filesystem.mount(&path, true);
    }

    println!("{}", graphics::get_renderer_info(ctx).unwrap());
    let state = &mut MainState::new(ctx).unwrap();
    if let Err(e) = event::run(ctx, state) {
        println!("Error encountered: {}", e);
    } else {
        println!("Game exited cleanly.");
    }
}
