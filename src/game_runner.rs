use std::error::Error;
use macroquad::prelude::*;
use std::env;
use super::graphical_interface;
use super::game_logic;
use super::parser;

use std::thread;
use std::time;

pub async fn run_game() -> Result<(),Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        return Err(Box::new(parser::CustomError("Incorrect number of arguments".to_string())));
    }
    let mut game_grid = parser::parse_grid(&args[1])?;
    loop {
        clear_background(WHITE);
        
        graphical_interface::draw_grid(&game_grid).await;
        game_grid = game_logic::update_game_grid(&game_grid);

        thread::sleep(time::Duration::from_millis(args[2].parse::<u64>()?));
        next_frame().await
    }
}
