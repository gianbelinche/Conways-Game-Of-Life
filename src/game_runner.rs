use crate::custom_error::CustomError;
use crate::game_logic::GameGrid;
use macroquad::prelude::*;
use std::env;

use super::game_logic;
use super::graphical_interface;
use super::parser;

use std::thread;
use std::time;

/// Checks if the game is valid, returns the game grid and the milliseconds
pub fn check_game_validity() -> Result<(GameGrid, u64), CustomError> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        return Err(CustomError::IncorrectNumberOfArguments);
    }
    let milliseconds = match args[2].parse::<u64>() {
        Ok(res) => res,
        Err(_) => return Err(CustomError::BadMilliseconds),
    };
    Ok((parser::parse_grid(&args[1])?, milliseconds))
}

/// Runs the game loop
pub async fn run_game(game_grid: &mut GameGrid, milliseconds: u64) {
    loop {
        clear_background(WHITE);

        graphical_interface::draw_grid(game_grid).await;
        game_logic::update_game_grid(game_grid);

        thread::sleep(time::Duration::from_millis(milliseconds));
        next_frame().await
    }
}
