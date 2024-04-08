use macroquad::prelude::*;

struct Grid {
    offset_x: f32,
    offset_y: f32,
    sq_size: f32
}

fn get_grid(squares: u32) -> Grid {
    let game_size = screen_width().min(screen_height());
    let offset_x = (screen_width() - game_size) / 2. + 10.;
    let offset_y = (screen_height() - game_size) / 2. + 10.;
    let sq_size = (screen_height() - offset_y * 2.) / squares as f32;

    Grid{offset_x,offset_y,sq_size}
}

pub async fn create_grid(squares: u32) {
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

pub async fn fill_square(i: u32, j:u32,squares:u32 ) {
    let grid = get_grid(squares);

    draw_rectangle(grid.offset_x + grid.sq_size * i as f32, grid.offset_y + grid.sq_size * j as f32, grid.sq_size, grid.sq_size, BLACK);
}

