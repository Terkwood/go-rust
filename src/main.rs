extern crate ggez;

mod draw;
mod game;
mod pixel_math;

use game::BOARD_SIZE;

use ggez::conf;
use ggez::event;
use ggez::event::MouseButton;
use ggez::graphics;
use ggez::graphics::{DrawMode, Font, Point2, Text};
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
}
