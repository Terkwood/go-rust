extern crate ggez;

use game::Board;
use ggez::graphics::{Mesh, MeshBuilder, Point2};
use ggez::{Context, GameResult};
use pixel_math;
use pixel_math::{MARGIN, POSITION_SIZE, SCREEN_SIZE};

pub fn build_game_mesh(ctx: &mut Context, board: &Board) -> GameResult<Mesh> {
    let mb = &mut MeshBuilder::new();

    add_background_to_mesh(mb);

    //add_pieces_to_mesh(mb, board);

    mb.build(ctx)
}

pub fn add_background_to_mesh(mb: &mut MeshBuilder) {
    let rows = pixel_math::rows();
    let columns = pixel_math::columns();
    const LINE_WIDTH: f32 = 4.0;

    for r in rows {
        mb.line(
            &[
                Point2::new(MARGIN.0, r),
                Point2::new(SCREEN_SIZE.0 - MARGIN.0 - POSITION_SIZE.0, r),
            ],
            LINE_WIDTH,
        );
    }

    for c in columns {
        mb.line(
            &[
                Point2::new(c, MARGIN.1),
                Point2::new(c, SCREEN_SIZE.1 - MARGIN.1 - POSITION_SIZE.1),
            ],
            LINE_WIDTH,
        );
    }
}
