extern crate sdl2;

use std::time::Duration;

use game::{
    clear_canvas, compute_cells_next_state, draw_cells, draw_grid, handle_events, to_screen,
    update_cells_state, Game, GameState, FPS,
};

mod display;
mod game;
mod grid;

fn main() {
    let tick_duration = Duration::new(0, 1_000_000_000 / FPS);
    let mut game_state = GameState::Edit;
    let mut tick_num: u32 = 0;
    let game = Game::new(700, 700);

    while game_state != GameState::Quit {
        clear_canvas(&game);
        if game_state == GameState::Edit {
            draw_grid(&game);
        }
        draw_cells(&game);
        to_screen(&game);
        ::std::thread::sleep(tick_duration);
        handle_events(&game, &mut game_state);
        if game_state == GameState::Run && tick_num % game.update_rate() == 0 {
            compute_cells_next_state(&game);
            update_cells_state(&game);
            tick_num = 0;
        } else {
            tick_num += 1;
        }
    }
}
