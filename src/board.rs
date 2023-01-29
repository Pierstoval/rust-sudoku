use crate::cell::Cell;
use crate::{PADDING, BOX_INDEXES, ROW_INDEXES, COLUMN_INDEXES};
use crate::context::index_to_xy;

pub struct Board {
    pub cells: Vec<Cell>,
    pub board_size: f32,
    pub cell_size: f32,
    pub selected_index: Option<usize>,
    pub selected_number: Option<u32>,
}

impl Board {
    pub fn new() -> Self {
        Board {
            cells: vec![Cell::new(); 81],
            board_size: 0.0,
            cell_size: 0.0,
            selected_index: None,
            selected_number: None,
        }
    }

    pub fn clear_highlight(&mut self) {
        for cell in self.cells.iter_mut() {
            cell.clear_highlight();
        }
    }

    pub fn clear(&mut self) {
        self.clear_highlight();
        self.selected_index = None;
        self.selected_number = None;
    }

    pub fn number(&mut self, number: u32) {
        if self.selected_index.is_none() {
            return;
        }

        self.cells[self.selected_index.unwrap()].set_number(number);
        self.selected_number = Some(number);
        self.highlight();
    }

    pub fn click(&mut self, x: f32, y: f32) {
        let mut clicked = (false, None);
        self.selected_index = None;

        // perform a click on each cell to see which one
        // gets selected
        for (i, cell) in self.cells.iter_mut().enumerate() {
            clicked = cell.click(x, y);
            if clicked.0 {
                self.selected_index = Some(i);
                self.selected_number = cell.number;
                break;
            }
        }

        // no cell was clicked
        if !clicked.0 {
            return;
        }

        self.highlight();
    }

    fn highlight(&mut self) {
        self.clear_highlight();

        if self.selected_index.is_none() {
            return;
        }

        let mut highlight_list = vec![self.selected_index.unwrap()];

        // only highlight numbers if the selected cell has a number
        if self.selected_number.is_some() {
            for (i, cell) in self.cells.iter_mut().enumerate() {
                if cell.number == self.selected_number {
                    cell.emphasize = true;
                    highlight_list.push(i);
                }
            }
        }

        for index in highlight_list {
            self.highlight_areas(BOX_INDEXES, index);
            self.highlight_areas(ROW_INDEXES, index);
            self.highlight_areas(COLUMN_INDEXES, index);
        }
    }

    fn highlight_areas(&mut self, area: &[[usize; 9]; 9], selected_index: usize) {
        for index_row in area.iter() {
            if index_row.contains(&selected_index) {
                for index in index_row.iter() {
                    self.cells[*index].highlighted = true;
                }
    
                break;
            }
        }
    }
    
    pub fn update(&mut self, board_size: f32) -> bool {
        if self.board_size as i32 == board_size as i32 {
            return false;
        }

        self.board_size = board_size;
        self.cell_size = self.board_size / 9.0;

        for (i, cell) in self.cells.iter_mut().enumerate() {
            let (x, y) = index_to_xy(i);
            let x_pos = PADDING + (x as f32 * self.cell_size);
            let y_pos = PADDING + (y as f32 * self.cell_size);

            cell.update(x_pos, y_pos, self.cell_size);
        }

        true
    }
}