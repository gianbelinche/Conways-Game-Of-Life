#[derive(Clone,Eq,PartialEq)]
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
