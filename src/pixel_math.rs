pub const SCREEN_SIZE: (f32, f32) = (800.0, 600.0);

const DRAWABLE_SIZE: f32 = SCREEN_SIZE.0 / 8.0 * 5.0;

pub const MARGIN: (f32, f32) = (
    (SCREEN_SIZE.0 - DRAWABLE_SIZE) / 2.0,
    (SCREEN_SIZE.1 - DRAWABLE_SIZE) / 2.0,
);

pub const POSITION_SIZE: (f32, f32) = (
    (SCREEN_SIZE.0 - MARGIN.0 * 2.0) / BOARD_SIZE as f32,
    (SCREEN_SIZE.1 - MARGIN.1 * 2.0) / BOARD_SIZE as f32,
);

pub const BOARD_SIZE: usize = 19;

pub fn columns() -> Vec<f32> {
    (0..BOARD_SIZE)
        .map(|i| MARGIN.0 + POSITION_SIZE.0 * i as f32)
        .collect::<Vec<f32>>()
}

pub fn rows() -> Vec<f32> {
    (0..BOARD_SIZE)
        .map(|j| MARGIN.1 + POSITION_SIZE.1 * j as f32)
        .collect::<Vec<f32>>()
}

pub fn screen_to_board(x: f32, y: f32) -> Option<(u16, u16)> {
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
