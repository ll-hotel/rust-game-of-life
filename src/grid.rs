use std::cell::Cell;

#[derive(Clone)]
pub struct GridCell {
    pub alive: Cell<bool>,
    pub next_state: Cell<bool>,
}

pub struct CellGrid {
    pub width: u32,
    pub height: u32,
    cells: Vec<GridCell>,
}

impl CellGrid {
    pub fn new(width: u32, height: u32) -> Self {
        let mut cells = Vec::new();
        for _ in 0..width * height {
            cells.push(GridCell {
                alive: Cell::new(false),
                next_state: Cell::new(false),
            });
        }
        Self {
            width,
            height,
            cells,
        }
    }

    pub fn cell(&self, x: u32, y: u32) -> &GridCell {
        let index = x as usize + (y as usize * self.width as usize);
        self.cells.get(index).expect(
            format!(
                "Cell index {index} for pos ({x}, {y}) should be less than {:?}",
                self.cells.len()
            )
            .as_str(),
        )
    }
}
