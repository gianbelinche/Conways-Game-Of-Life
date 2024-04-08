use macroquad::prelude::*;
mod graphical_interface;

const SQUARES: u32 = 16;
#[macroquad::main("Conways Game Of Life")]
async fn main() {
    loop {
        clear_background(WHITE);

        graphical_interface::create_grid(SQUARES).await;

        graphical_interface::fill_square(1,7,SQUARES).await;
        graphical_interface::fill_square(3,9,SQUARES).await;
        graphical_interface::fill_square(3,10,SQUARES).await;
        graphical_interface::fill_square(4,9,SQUARES).await;
        graphical_interface::fill_square(15,15,SQUARES).await;

        next_frame().await
    }
}
