use std::error::Error;
use super::game_logic;

pub fn parse_grid(path: String) -> Result<game_logic::GameGrid, Box<dyn Error>> {
    // Returns a grid from a path to a file, it does not have error handling yet
    let mut state: Vec<Vec<game_logic::State>> = vec![];
    let mut rdr = csv::ReaderBuilder::new().has_headers(false).from_path(path)?;
    for result in rdr.records() {
        let mut vector_internal: Vec<game_logic::State> = vec![];
        let record = result?;
        let vector: Vec<String> = record.deserialize(None)?;
        for v in vector {
            if v == "1" {
                vector_internal.push(game_logic::State::Alive);
            } else {
                vector_internal.push(game_logic::State::Dead);
            }
        }
        state.push(vector_internal);
    }
    Ok(game_logic::GameGrid{squares:state.len() as u32,state})
}
