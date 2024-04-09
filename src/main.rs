use macroquad::prelude::*;
mod graphical_interface;
mod game_logic;

use std::thread;
use std::time;
const SQUARES: u32 = 16;
#[macroquad::main("Conways Game Of Life")]
async fn main() {
    let mut game_grid = game_logic::create_initial_game_grid(SQUARES);
    loop {
        clear_background(WHITE);
        
        graphical_interface::draw_grid(&game_grid).await;
        game_grid = game_logic::update_game_grid(&game_grid);

        thread::sleep(time::Duration::from_millis(500));
        next_frame().await
    }
}
