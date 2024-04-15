use crate::custom_error::CustomError;

use super::game_logic;


/// Returns a grid from a path to a file, it does not have error handling yet
pub fn parse_grid(path: &String) -> Result<game_logic::GameGrid, CustomError> {
    let mut state: Vec<Vec<game_logic::State>> = vec![];
    let mut rdr =  match csv::ReaderBuilder::new().has_headers(false).from_path(path) {
        Ok(res) => res,
        Err(_) => return Err(CustomError::FileNotFound)
    };
    for result in rdr.records() {
        let mut vector_internal: Vec<game_logic::State> = vec![];
        let record = match result{
            Ok(res) => res,
            Err(_) => return Err(CustomError::IncorrectFormat)
        };
        let vector: Vec<String> = match record.deserialize(None) {
            Ok(res) => res,
            Err(_) => return Err(CustomError::IncorrectFormat)
        };
        for v in vector {
            match v.as_str() {
                "1" => vector_internal.push(game_logic::State::Alive),
                "0" => vector_internal.push(game_logic::State::Dead),
                ""  => continue,
                _ => return Err(CustomError::WrongState)
            }
        }
        state.push(vector_internal);
    }
    let squares = state.len() as u32;
    for v in &state {
        if v.len() as u32 != squares {
            return Err(CustomError::MissingLines);
        }
    }

    Ok(game_logic::GameGrid{squares,state})
}
