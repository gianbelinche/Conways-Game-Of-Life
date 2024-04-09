use std::error::Error;
use super::game_logic;

#[derive(Debug)]
pub struct CustomError(pub String);

impl std::fmt::Display for CustomError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for CustomError {
    fn description(&self) -> &str {
        &self.0
    }
}

pub fn parse_grid(path: &String) -> Result<game_logic::GameGrid, Box<dyn Error>> {
    // Returns a grid from a path to a file, it does not have error handling yet
    let mut state: Vec<Vec<game_logic::State>> = vec![];
    let mut rdr =  csv::ReaderBuilder::new().has_headers(false).from_path(path)?;
    for result in rdr.records() {
        let mut vector_internal: Vec<game_logic::State> = vec![];
        let record = result?;
        let vector: Vec<String> = record.deserialize(None)?;
        for v in vector {
            if v == "1" {
                vector_internal.push(game_logic::State::Alive);
            } else if v == "0" {
                vector_internal.push(game_logic::State::Dead);
            } else if v == ""{
                continue;
            } else {
                return Err(Box::new(CustomError("Incorrect CSV format: state not 0 nor 1".to_string())));
            }
        }
        state.push(vector_internal);
    }
    let squares = state.len() as u32;
    for v in &state {
        if v.len() as u32 != squares {
            return Err(Box::new(CustomError("Incorrect CSV format: missing line(s)".to_string())));
        }
    }

    Ok(game_logic::GameGrid{squares,state})
}
