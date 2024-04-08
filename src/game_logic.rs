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
    let alive_neighbours = get_alive_neighbours(i,j,grid);
    if grid.state[i][j] == State::Dead && alive_neighbours == 3 {
        State::Alive
    } else if grid.state[i][j] == State::Alive && (alive_neighbours == 2 || alive_neighbours == 3) {
        State::Alive
    } else {
        State::Dead
    }
}

fn get_alive_neighbours(i: usize, j: usize, grid: &GameGrid) -> u32 {
    // Returns the number of alive neighbours for a given cell in the grid

    let mut alive_neighbours = 0;

    if grid.state[index_no_overflow_sub(i as i32, grid.squares) as usize][j] == State::Alive {
        alive_neighbours += 1;
    }
    if grid.state[index_no_overflow_add(i as i32, grid.squares) as usize][j] == State::Alive {
        alive_neighbours += 1;
    }
    if grid.state[i][index_no_overflow_add(j as i32, grid.squares) as usize] == State::Alive {
        alive_neighbours += 1;
    }
    if grid.state[i][index_no_overflow_sub(j as i32, grid.squares) as usize] == State::Alive {
        alive_neighbours += 1;
    }
    if grid.state[index_no_overflow_sub(i as i32, grid.squares) as usize][index_no_overflow_add(j as i32, grid.squares) as usize] == State::Alive {
        alive_neighbours += 1;
    }
    if grid.state[index_no_overflow_sub(i as i32, grid.squares) as usize][index_no_overflow_sub(j as i32, grid.squares) as usize] == State::Alive {
        alive_neighbours += 1;
    }
    if grid.state[index_no_overflow_add(i as i32, grid.squares) as usize][index_no_overflow_sub(j as i32, grid.squares) as usize] == State::Alive {
        alive_neighbours += 1;
    }
    if grid.state[index_no_overflow_add(i as i32, grid.squares) as usize][index_no_overflow_add(j as i32, grid.squares) as usize] == State::Alive {
        alive_neighbours += 1;
    } // TODO make this less ugly

    alive_neighbours
}

fn index_no_overflow_sub(i: i32,squares: u32) -> i32 {
    // Returns sub of i and 1 bounded by squares
    if (i - 1) < 0 {
        (squares - 1) as i32
    } else {
        i-1
    }
}

fn index_no_overflow_add(i: i32, squares: u32) -> i32{
    // Returns add of i and 1 bounded by squares
    if (i + 1) > (squares - 1) as i32 {
        0
    } else {
        i + 1
    }
}
