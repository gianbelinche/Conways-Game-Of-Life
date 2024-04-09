use std::error::Error;
use macroquad::prelude::*;
use std::env;
mod graphical_interface;
mod game_logic;
mod parser;

use std::thread;
use std::time;
#[macroquad::main("Conways Game Of Life")]
async fn main() -> Result<(),Box<dyn Error>>{
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Incorrect number of args");
    }
    let mut game_grid = parser::parse_grid(&args[1])?;
    loop {
        clear_background(WHITE);
        
        graphical_interface::draw_grid(&game_grid).await;
        game_grid = game_logic::update_game_grid(&game_grid);

        thread::sleep(time::Duration::from_millis(500));
        next_frame().await
    }
}
