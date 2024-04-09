use macroquad::window::next_frame;

mod graphical_interface;
mod game_logic;
mod parser;
mod game_runner;

#[macroquad::main("Conways Game Of Life")]
async fn main(){
    if let Err(err) = game_runner::run_game().await {
        loop {
            graphical_interface::draw_error(&*err.to_string()).await;
            next_frame().await;
        }
    };
}
