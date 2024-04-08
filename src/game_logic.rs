pub struct GameGrid {
    pub state: Vec<Vec<bool>>
}

pub fn create_initial_game_grid(squares:u32) -> GameGrid{
    let mut state = vec![];
    for i in 0..squares {
        if i % 2 == 0 {
            state.push(vec![false;squares as usize]);
        } else {
            state.push(vec![true;squares as usize]);
        }
    }
    GameGrid{state}
}
