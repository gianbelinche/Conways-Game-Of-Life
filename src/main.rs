use macroquad::prelude::*;
mod graphical_interface;
mod game_logic;

const SQUARES: u32 = 16;
#[macroquad::main("Conways Game Of Life")]
async fn main() {
    loop {
        clear_background(WHITE);
        
        let game_grid = game_logic::create_initial_game_grid(SQUARES);
        graphical_interface::draw_grid(&game_grid).await;
        game_logic::update_game_grid(&game_grid);
        next_frame().await
    }
}
