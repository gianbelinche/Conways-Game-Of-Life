use macroquad::window::next_frame;

mod graphical_interface;
mod game_logic;
mod parser;
mod game_runner;
mod custom_error;

#[macroquad::main("Conways Game Of Life")]
async fn main(){
    let (mut grid,milliseconds) = match game_runner::check_game_validity() {
        Ok(res) => res,
        Err(err) => {
            loop {
                graphical_interface::draw_error(&*err.to_string()).await;
                next_frame().await;
            }   
        }
    };
    game_runner::run_game(&mut grid,milliseconds).await;
}
