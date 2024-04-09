use std::error::Error;
use macroquad::prelude::*;
mod graphical_interface;
mod game_logic;
mod parser;

use std::thread;
use std::time;
const SQUARES: u32 = 16;
#[macroquad::main("Conways Game Of Life")]
async fn main() -> Result<(),Box<dyn Error>>{
    let mut game_grid = parser::parse_grid("grid.csv".to_string())?;
    loop {
        clear_background(WHITE);
        
        graphical_interface::draw_grid(&game_grid).await;
        game_grid = game_logic::update_game_grid(&game_grid);

        thread::sleep(time::Duration::from_millis(500));
        next_frame().await
    }
}
