#[derive(Clone,Eq,PartialEq,Copy,Debug)]
/// Describes the 2 possible states of a Cell in the Grid
pub enum State{
    Alive,
    Dead
}
#[derive(Clone)]
/// Represents the squared grid of size squares
pub struct GameGrid {
    pub squares: u32,
    pub state: Vec<Vec<State>>
}

/// Creates an initial grid of size squares
pub fn create_initial_game_grid(squares:u32) -> GameGrid{
    let mut state = vec![];
    for _ in 0..squares {
        state.push(vec![State::Dead;squares as usize]);
    }
    GameGrid{squares,state}
}

/// Update the grid state based on game of lifes rules
pub fn update_game_grid(game_grid: &mut GameGrid) {
    let mut new_game_grid = create_initial_game_grid(game_grid.squares);
    for i in 0..game_grid.squares {
        for j in 0..game_grid.squares {
            let new_state = get_new_state(i as usize,j as usize,&game_grid);
            new_game_grid.state[i as usize][j as usize] = new_state;
        }
    }

    *game_grid = new_game_grid;
}

/// Given a cell, returns the new state based on game of lifes rules
fn get_new_state(i: usize,j: usize,grid: &GameGrid) -> State {
    let alive_neighbours = get_alive_neighbours(i,j,grid);
    if grid.state[i][j] == State::Dead && alive_neighbours == 3 {
        State::Alive
    } else if grid.state[i][j] == State::Alive && (alive_neighbours == 2 || alive_neighbours == 3) {
        State::Alive
    } else {
        State::Dead
    }
}

/// Returns the number of alive neighbours for a given cell in the grid
fn get_alive_neighbours(i: usize, j: usize, grid: &GameGrid) -> u32 {

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

/// Returns sub of i and 1 bounded by squares
fn index_no_overflow_sub(i: i32,squares: u32) -> i32 {
    if (i - 1) < 0 {
        (squares - 1) as i32
    } else {
        i-1
    }
}

/// Returns add of i and 1 bounded by squares
fn index_no_overflow_add(i: i32, squares: u32) -> i32{
    if (i + 1) > (squares - 1) as i32 {
        0
    } else {
        i + 1
    }
}

#[cfg(test)]
mod tests {
    use super::update_game_grid;
    use super::GameGrid;
    use super::State;

    #[test]
    fn test_update_game_grid_all_dead(){
        let squares = 4;
        let state = vec![
            vec![State::Dead,State::Dead,State::Dead,State::Dead],
            vec![State::Dead,State::Dead,State::Dead,State::Dead],
            vec![State::Dead,State::Dead,State::Dead,State::Dead],
            vec![State::Dead,State::Dead,State::Dead,State::Dead],
        ];

        let mut game_grid = GameGrid{squares,state};
        let old_game_grid = game_grid.clone();

        update_game_grid(&mut game_grid);

        assert_eq!(game_grid.state,old_game_grid.state);
    }


    #[test]
    fn test_update_game_grid_all_alive(){
        let squares = 4;
        let state = vec![
            vec![State::Alive,State::Alive,State::Alive,State::Alive],
            vec![State::Alive,State::Alive,State::Alive,State::Alive],
            vec![State::Alive,State::Alive,State::Alive,State::Alive],
            vec![State::Alive,State::Alive,State::Alive,State::Alive],
        ];

        let mut game_grid = GameGrid{squares,state};

        update_game_grid(&mut game_grid);
        
        let result_grid_state = vec![
            vec![State::Dead,State::Dead,State::Dead,State::Dead],
            vec![State::Dead,State::Dead,State::Dead,State::Dead],
            vec![State::Dead,State::Dead,State::Dead,State::Dead],
            vec![State::Dead,State::Dead,State::Dead,State::Dead],
        ];
        assert_eq!(result_grid_state,game_grid.state);
    }

    #[test]
    fn test_update_game_grid_one_alive(){
        let squares = 4;
        let state = vec![
            vec![State::Dead,State::Dead,State::Dead,State::Dead],
            vec![State::Dead,State::Alive,State::Dead,State::Dead],
            vec![State::Dead,State::Dead,State::Dead,State::Dead],
            vec![State::Dead,State::Dead,State::Dead,State::Dead],
        ];

        let mut game_grid = GameGrid{squares,state};

        update_game_grid(&mut game_grid);
        
        let result_grid_state = vec![
            vec![State::Dead,State::Dead,State::Dead,State::Dead],
            vec![State::Dead,State::Dead,State::Dead,State::Dead],
            vec![State::Dead,State::Dead,State::Dead,State::Dead],
            vec![State::Dead,State::Dead,State::Dead,State::Dead],
        ];
        assert_eq!(result_grid_state,game_grid.state);
    }

    #[test]
    fn test_update_game_grid_one_alive_stays_alive(){
        let squares = 4;
        let state = vec![
            vec![State::Dead,State::Dead,State::Alive,State::Dead],
            vec![State::Dead,State::Alive,State::Dead,State::Dead],
            vec![State::Alive,State::Dead,State::Dead,State::Dead],
            vec![State::Dead,State::Dead,State::Dead,State::Dead],
        ];

        let mut game_grid = GameGrid{squares,state};

        update_game_grid(&mut game_grid);
        
        let result_grid_state = vec![
            vec![State::Dead,State::Dead,State::Dead,State::Dead],
            vec![State::Dead,State::Alive,State::Dead,State::Dead],
            vec![State::Dead,State::Dead,State::Dead,State::Dead],
            vec![State::Dead,State::Dead,State::Dead,State::Dead],
        ];
        assert_eq!(result_grid_state,game_grid.state);
    }

    #[test]
    fn test_update_game_grid_dead_with_3_alive_neighbours_is_alive(){
        let squares = 5;
        let state = vec![
            vec![State::Dead,State::Alive,State::Dead,State::Dead,State::Dead],
            vec![State::Dead,State::Dead,State::Dead,State::Dead,State::Dead],
            vec![State::Alive,State::Dead,State::Alive,State::Dead,State::Dead],
            vec![State::Dead,State::Dead,State::Dead,State::Dead,State::Dead],
            vec![State::Dead,State::Dead,State::Dead,State::Dead,State::Dead],
        ];

        let mut game_grid = GameGrid{squares,state};

        update_game_grid(&mut game_grid);
        
        let result_grid_state = vec![
            vec![State::Dead,State::Dead,State::Dead,State::Dead,State::Dead],
            vec![State::Dead,State::Alive,State::Dead,State::Dead,State::Dead],
            vec![State::Dead,State::Dead,State::Dead,State::Dead,State::Dead],
            vec![State::Dead,State::Dead,State::Dead,State::Dead,State::Dead],
            vec![State::Dead,State::Dead,State::Dead,State::Dead,State::Dead],
        ];
        assert_eq!(result_grid_state,game_grid.state);
    }

    #[test]
    fn test_update_game_grid_glider(){
        let squares = 5;
        let state = vec![
            vec![State::Dead,State::Alive,State::Dead,State::Dead,State::Dead],
            vec![State::Dead,State::Dead,State::Alive,State::Dead,State::Dead],
            vec![State::Alive,State::Alive,State::Alive,State::Dead,State::Dead],
            vec![State::Dead,State::Dead,State::Dead,State::Dead,State::Dead],
            vec![State::Dead,State::Dead,State::Dead,State::Dead,State::Dead],
        ];

        let mut game_grid = GameGrid{squares,state};

        update_game_grid(&mut game_grid);
        
        let result_grid_state = vec![
            vec![State::Dead,State::Dead,State::Dead,State::Dead,State::Dead],
            vec![State::Alive,State::Dead,State::Alive,State::Dead,State::Dead],
            vec![State::Dead,State::Alive,State::Alive,State::Dead,State::Dead],
            vec![State::Dead,State::Alive,State::Dead,State::Dead,State::Dead],
            vec![State::Dead,State::Dead,State::Dead,State::Dead,State::Dead],
        ];
        assert_eq!(result_grid_state,game_grid.state);
    }

}
