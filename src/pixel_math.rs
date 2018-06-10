mod pixel_math {
    pub const SCREEN_SIZE: (f32, f32) = (800.0, 600.0);

    const DRAWABLE_SIZE: f32 = SCREEN_SIZE.0 / 8.0 * 5.0;

    pub const MARGIN: (f32, f32) = (
        (SCREEN_SIZE.0 - DRAWABLE_SIZE) / 2.0,
        (SCREEN_SIZE.1 - DRAWABLE_SIZE) / 2.0,
    );

    pub const POSITION_SIZE: (f32, f32) = (
        (SCREEN_SIZE.0 - MARGIN.0 * 2.0) / 3.0,
        (SCREEN_SIZE.1 - MARGIN.1 * 2.0) / 3.0,
    );

    const BOARD_SIZE = 19;

    pub const COLUMNS: [f32] = (MARGIN.0 + POSITION_SIZE.0, MARGIN.0 + POSITION_SIZE.0 * 2.0);

    pub const ROWS: [f32] = (MARGIN.1 + POSITION_SIZE.1, MARGIN.1 + POSITION_SIZE.1 * 2.0);

    pub const X_PIECE_OFFSET: [f32] =
        (POSITION_SIZE.0 / 2.0 * 0.70, POSITION_SIZE.1 / 2.0 * 0.70);

    pub fn screen_to_board(x: f32, y: f32) -> Option<(u16, u16)> {
        use pixel_math::{MARGIN, SCREEN_SIZE};
        if x < MARGIN.0 || x > SCREEN_SIZE.0 - MARGIN.0 {
            None
        } else if y < MARGIN.1 || y > SCREEN_SIZE.1 - MARGIN.1 {
            None
        } else {
            let i = (x - MARGIN.0) / POSITION_SIZE.0;
            let j = (y - MARGIN.1) / POSITION_SIZE.1;
            Some((i as u16, j as u16))
        }
    }
}