use crate::{
    display::Display,
    grid::{CellGrid, GridCell},
};
use sdl2::{event::Event, mouse::MouseWheelDirection, sys::SDL_KeyCode};
use std::cell::Cell;

pub const FPS: u32 = 60;
pub const CELL_SIZE: u32 = 16;

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum GameState {
    Edit,
    Run,
    Quit,
}

pub struct Game {
    display: Display,
    grid: CellGrid,
    update_delay: Cell<u32>,
}

impl Game {
    pub fn new(width: u32, height: u32) -> Self {
        let width = width / CELL_SIZE;
        let height = height / CELL_SIZE;

        Self {
            display: Display::new(width * CELL_SIZE, height * CELL_SIZE),
            grid: CellGrid::new(width, height),
            update_delay: Cell::new(FPS),
        }
    }

    pub fn update_delay(&self) -> u32 {
        self.update_delay.get()
    }

    pub fn increase_update_delay(&self) {
        let current_update_delay = self.update_delay.get();

        if current_update_delay > FPS / 12 {
            self.update_delay.set(current_update_delay - (FPS / 12));
        } else {
            self.update_delay.set(1);
        }
    }

    pub fn decrease_update_delay(&self) {
        let current_update_delay = self.update_delay.get();

        self.update_delay.set(current_update_delay + (FPS / 12));
    }
}

pub fn clear_canvas(game: &Game) {
    game.display.set_draw_color(0, 0, 0);
    game.display.clear();
}

pub fn draw_grid(game: &Game) {
    game.display.set_draw_color(255, 255, 255);
    for y in 1..game.grid.height {
        game.display
            .draw_line(0, y * CELL_SIZE, game.display.width, y * CELL_SIZE);
    }
    for x in 1..game.grid.width {
        game.display
            .draw_line(x * CELL_SIZE, 0, x * CELL_SIZE, game.display.height);
    }
}

pub fn draw_cells(game: &Game) {
    game.display.set_draw_color(255, 255, 255);
    for y in 0..game.grid.height {
        for x in 0..game.grid.width {
            if game.grid.cell(x, y).alive.get() {
                game.display
                    .fill_rect(x * CELL_SIZE, y * CELL_SIZE, CELL_SIZE);
            }
        }
    }
}

pub fn compute_cells_next_state(game: &Game) {
    let mut neighbourhood: u32;
    let mut cell: &GridCell;
    let mut top: bool;
    let mut right: bool;
    let mut bottom: bool;
    let mut left: bool;

    for y in 0..game.grid.height - 1 {
        for x in 0..game.grid.width - 1 {
            cell = game.grid.cell(x, y);
            neighbourhood = 0;
            top = y > 0;
            bottom = y < game.grid.height - 1;
            left = x > 0;
            right = x < game.grid.width - 1;

            if top {
                neighbourhood += game.grid.cell(x, y - 1).alive.get() as u32;
                if left {
                    neighbourhood += game.grid.cell(x - 1, y - 1).alive.get() as u32;
                }
                if right {
                    neighbourhood += game.grid.cell(x + 1, y - 1).alive.get() as u32;
                }
            }
            if bottom {
                neighbourhood += game.grid.cell(x, y + 1).alive.get() as u32;
                if left {
                    neighbourhood += game.grid.cell(x - 1, y + 1).alive.get() as u32;
                }
                if right {
                    neighbourhood += game.grid.cell(x + 1, y + 1).alive.get() as u32;
                }
            }
            if left {
                neighbourhood += game.grid.cell(x - 1, y).alive.get() as u32;
            }
            if right {
                neighbourhood += game.grid.cell(x + 1, y).alive.get() as u32;
            }

            cell.next_state
                .set(neighbourhood == 3 || (neighbourhood == 2 && cell.alive.get()));
        }
    }
}

pub fn update_cells_state(game: &Game) {
    for y in 0..game.grid.height - 1 {
        for x in 0..game.grid.width - 1 {
            let cell = game.grid.cell(x, y);
            cell.alive.set(cell.next_state.get());
        }
    }
}

pub fn handle_events(game: &Game, game_state: &mut GameState) {
    let events = game.display.get_events();

    for event in events {
        match event {
            Event::Quit { .. } => {
                *game_state = GameState::Quit;
                break;
            }
            Event::KeyDown {
                timestamp: _,
                window_id: _,
                keycode,
                scancode: _,
                keymod: _,
                repeat: _,
            } => {
                if keycode.is_none() {
                    continue;
                }
                let code = keycode.unwrap().into_i32();
                match code {
                    SDLK_ESCAPE => {
                        *game_state = GameState::Quit;
                        break;
                    }
                    SDLK_SPACE => {
                        *game_state = match *game_state {
                            GameState::Run => GameState::Edit,
                            GameState::Edit => GameState::Run,
                            _ => GameState::Quit,
                        }
                    }
                    SDLK_UP => {
                        if *game_state == GameState::Run {
                            game.increase_update_delay();
                        }
                    }
                    SDLK_DOWN => {
                        if *game_state == GameState::Run {
                            game.decrease_update_delay();
                        }
                    }
                    _ => {}
                }
            }
            Event::MouseButtonDown {
                timestamp: _,
                window_id: _,
                which: _,
                mouse_btn,
                clicks: _,
                x,
                y,
            } => {
                if *game_state == GameState::Edit {
                    match mouse_btn {
                        sdl2::mouse::MouseButton::Left => {
                            game.grid
                                .cell(x as u32 / CELL_SIZE, y as u32 / CELL_SIZE)
                                .alive
                                .set(true);
                        }
                        sdl2::mouse::MouseButton::Right => {
                            game.grid
                                .cell(x as u32 / CELL_SIZE, y as u32 / CELL_SIZE)
                                .alive
                                .set(false);
                        }
                        _ => {}
                    }
                }
            }
            Event::MouseWheel {
                timestamp: _,
                window_id: _,
                which: _,
                x: _,
                y,
                direction,
                precise_x: _,
                precise_y: _,
                mouse_x: _,
                mouse_y: _,
            } => {
                if *game_state == GameState::Run {
                    match direction {
                        MouseWheelDirection::Normal => {
                            if y > 0 {
                                game.increase_update_delay();
                            } else if y < 0 {
                                game.decrease_update_delay();
                            }
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }
}

const SDLK_ESCAPE: i32 = SDL_KeyCode::SDLK_ESCAPE as i32;
const SDLK_SPACE: i32 = SDL_KeyCode::SDLK_SPACE as i32;
const SDLK_UP: i32 = SDL_KeyCode::SDLK_UP as i32;
const SDLK_DOWN: i32 = SDL_KeyCode::SDLK_DOWN as i32;

pub fn to_screen(game: &Game) {
    game.display.present();
}
