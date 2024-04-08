use macroquad::prelude::*;
mod graphical_interface;
mod game_logic;

const SQUARES: u32 = 16;
#[macroquad::main("Conways Game Of Life")]
async fn main() {
    let mut game_grid = game_logic::create_initial_game_grid(SQUARES);
    loop {
        clear_background(WHITE);
        
        graphical_interface::draw_grid(&game_grid).await;
        game_grid = game_logic::update_game_grid(&game_grid);
        next_frame().await
    }
}
