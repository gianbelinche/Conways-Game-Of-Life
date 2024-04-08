#[derive(Clone,Eq,PartialEq,Copy)]
pub enum State{
    // Describes the 2 possible states of a Cell in the Grid
    Alive,
    Dead
}

pub struct GameGrid {
    // Represents the squared grid of size squares
    pub squares: u32,
    pub state: Vec<Vec<State>>
}

pub fn create_initial_game_grid(squares:u32) -> GameGrid{
    // Creates an initial grid of size squares
    // Right now the grid is harcoded as vertical lines, one dead an one alive, later this will be changed
    let mut state = vec![];
    for i in 0..squares {
        if i % 2 == 0 {
            state.push(vec![State::Dead;squares as usize]);
        } else {
            state.push(vec![State::Alive;squares as usize]);
        }
    }
    GameGrid{squares,state}
}

pub fn update_game_grid(game_grid: &GameGrid) -> GameGrid{
    // Update the grid state based on game of lifes rules, returns the new state
    let mut new_game_grid = create_initial_game_grid(game_grid.squares);
    for i in 0..game_grid.squares {
        for j in 0..game_grid.squares {
            let new_state = get_new_state(i as usize,j as usize,&game_grid);
            new_game_grid.state[i as usize][j as usize] = new_state;
        }
    }

    new_game_grid
}

fn get_new_state(i: usize,j: usize,grid: &GameGrid) -> State {
    // Given a cell, returns the new state based on game of lifes rules, WIP
    grid.state[i][j]
}
