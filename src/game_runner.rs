use std::error::Error;
use macroquad::prelude::*;
use std::env;
use crate::game_logic::GameGrid;

use super::graphical_interface;
use super::game_logic;
use super::parser;

use std::thread;
use std::time;

pub fn check_game_validity() -> Result<(GameGrid,u64),Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        return Err(Box::new(parser::CustomError("Incorrect number of arguments".to_string())));
    }
    let milliseconds = args[2].parse::<u64>()?;
    Ok((parser::parse_grid(&args[1])?,milliseconds))
}

pub async fn run_game(game_grid: &mut GameGrid,milliseconds: u64) {
    loop {
        clear_background(WHITE);
        
        graphical_interface::draw_grid(game_grid).await;
        game_logic::update_game_grid(game_grid);

        thread::sleep(time::Duration::from_millis(milliseconds));
        next_frame().await
    }
}
