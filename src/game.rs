use draw;
use ggez::conf;
use ggez::event;
use ggez::event::MouseButton;
use ggez::graphics;
use ggez::graphics::{DrawMode, Font, Point2, Text};
use ggez::timer;
use ggez::{Context, GameResult};

pub const BOARD_SIZE: usize = 19;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Piece {
    Black,
    White,
}

pub struct Board {
    contents: Vec<Vec<Option<Piece>>>,
}

impl Board {
    fn new() -> Board {
        let v: Vec<Vec<Option<Piece>>> = vec![vec![None; BOARD_SIZE]; BOARD_SIZE];
        Board { contents: v }
    }
}
pub struct MainState {
    board: Board,
    current_turn: Piece,
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        const DESIRED_FPS: u32 = 60;

        while timer::check_update_time(ctx, DESIRED_FPS) {
            // hello
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        const PURPLE: (u8, u8, u8) = (218, 112, 214);
        const CYAN: (u8, u8, u8) = (0, 255, 255);
        graphics::clear(ctx);

        let game_mesh = draw::build_game_mesh(ctx, &self.board)?;
        graphics::set_color(ctx, CYAN.into())?;
        graphics::draw_ex(ctx, &game_mesh, Default::default())?;

        graphics::present(ctx);
        Ok(())
    }
}


