use macroquad::prelude::*;

use crate::game_logic::{GameGrid,State};

/// Represents the grid on a graphical level
struct Grid {
    offset_x: f32,
    offset_y: f32,
    sq_size: f32
}

/// Returns the graphical grid
fn get_grid(squares: u32) -> Grid {
    let game_size = screen_width().min(screen_height());
    let offset_x = (screen_width() - game_size) / 2. + 10.;
    let offset_y = (screen_height() - game_size) / 2. + 10.;
    let sq_size = (screen_height() - offset_y * 2.) / squares as f32;

    Grid{offset_x,offset_y,sq_size}
}

/// Draws the grid lines on screen
async fn draw_grid_outline(squares: u32) {
    let grid = get_grid(squares);

    for i in 0..squares+1 {
        draw_line(
            grid.offset_x,
            grid.offset_y + grid.sq_size * i as f32,
            screen_width() - grid.offset_x,
            grid.offset_y + grid.sq_size * i as f32,
            2.,
            LIGHTGRAY,
        );
    }

    for i in 0..squares+1 {
        draw_line(
            grid.offset_x + grid.sq_size * i as f32,
            grid.offset_y,
            grid.offset_x + grid.sq_size * i as f32,
            screen_height() - grid.offset_y,
            2.,
            LIGHTGRAY,
        );
    }
}

/// Fills a cell black given its index
async fn fill_square(i: u32, j:u32,squares:u32 ) {
    let grid = get_grid(squares);

    draw_rectangle(grid.offset_x + grid.sq_size * i as f32, grid.offset_y + grid.sq_size * j as f32, grid.sq_size, grid.sq_size, BLACK);
}

/// Draws the entire grid given its current state
pub async fn draw_grid(grid: &GameGrid) {
    draw_grid_outline(grid.squares).await;
    for (i,row) in grid.state.iter().enumerate() {
        for (j,state) in row.iter().enumerate() {
            if *state == State::Alive {
                fill_square(j as u32, i as u32, grid.squares).await;
            }
        }
    }
}

/// Draws the error message on screen
pub async fn draw_error(error: &str) {

    clear_background(RED);

    draw_text("ERROR:", 20.0, 40.0, 40.0, DARKGRAY);
    draw_text(error, 20.0, 70.0, 40.0, DARKGRAY);
}
