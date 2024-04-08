#[derive(Clone,Eq,PartialEq,Copy)]
pub enum State{
    Alive,
    Dead
}

pub struct GameGrid {
    pub squares: u32,
    pub state: Vec<Vec<State>>
}

pub fn create_initial_game_grid(squares:u32) -> GameGrid{
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
    grid.state[i][j]
}
