extern crate ggez;

mod draw;
mod game;
mod pixel_math;

use game::{BOARD_SIZE, MainState};

use ggez::conf;
use ggez::event;
use ggez::graphics;
use ggez::timer;
use ggez::{Context, GameResult};
use std::env;
use std::path;

fn main() {
    println!("board size is {}", BOARD_SIZE);

    let rows = pixel_math::rows();
    for (i, r) in rows.iter().enumerate() {
        println!("row {} {}", i, r);
    }

    let columns = pixel_math::columns();
    for (j, c) in columns.iter().enumerate() {
        println!("col {} {}", j, c);
    }

    let c = conf::Conf::new();
    let ctx = &mut Context::load_from_conf("go", "ggez", c).unwrap();

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
