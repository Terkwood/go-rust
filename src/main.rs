mod draw;
mod pixel_math;
use pixel_math::BOARD_SIZE;

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
