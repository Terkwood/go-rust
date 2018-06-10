extern crate ggez;

use draw::ggez::graphics::{MeshBuilder, Point2};
use pixel_math;
use pixel_math::{MARGIN, SCREEN_SIZE};

fn add_background_to_mesh(mb: &mut MeshBuilder) {
    let rows = pixel_math::rows();
    let columns = pixel_math::columns();
    const LINE_WIDTH: f32 = 4.0;

    for r in rows {
        mb.line(
            &[
                Point2::new(MARGIN.0, r),
                Point2::new(SCREEN_SIZE.0 - MARGIN.0, r),
            ],
            LINE_WIDTH,
        );
    }

    for c in columns {
        mb.line(
            &[
                Point2::new(c, MARGIN.1),
                Point2::new(c, SCREEN_SIZE.1 - MARGIN.1),
            ],
            LINE_WIDTH,
        );
    }
}
