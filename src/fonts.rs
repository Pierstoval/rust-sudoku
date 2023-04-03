use macroquad::prelude::*;
use macroquad::text::TextParams;

use crate::{board::BoardMode, ICON_PENCIL, ICON_PENCIL_SLASH, ICON_UNDO};

fn find_best_font_size(font: &Option<Font>, text: &str, cell_size: f32) -> u16 {
    if cell_size < 1.0 {
        return 1;
    }

    let mut start_size: u16 = 1;
    let mut end_size: u16 = 100;

    while start_size <= end_size {
        let test_size = (start_size + end_size) / 2;
        let measurement = measure_text(text, *font, test_size, 1.0);
        let ratio = measurement.height / cell_size;

        if ratio > 0.6 && ratio < 0.61 {
            return test_size;
        } else if ratio > 0.6 {
            end_size = test_size - 1;
        } else {
            start_size = test_size + 1;
        }
    }

    start_size
}

pub struct CellFont {
    pub params: TextParams,
    pub font: Font,
    pub x_offset: f32,
    pub y_offset: f32,
    pub height: f32,
    pub width: f32,
}

impl CellFont {
    pub async fn new(font_path: &str, color: Color) -> Self {
        let font = load_ttf_font(font_path).await.unwrap();
        let measure = measure_text("9", Some(font), 48, 1.0);
        CellFont {
            font,
            params: TextParams {
                font,
                font_size: 48,
                font_scale: 1.0,
                font_scale_aspect: 1.0,
                rotation: 0.0,
                color,
            },
            x_offset: 0.0,
            y_offset: 0.0,
            height: measure.height,
            width: measure.width,
        }
    }

    pub fn update(&mut self, cell_size: f32) {
        self.params.font_size = find_best_font_size(&Some(self.font), "9", cell_size);
        let measure = measure_text("9", Some(self.font), self.params.font_size, 1.0);
        self.width = measure.width;
        self.height = measure.height;

        self.x_offset = (cell_size / 2.0) - (self.width / 2.0);
        self.y_offset = (cell_size / 2.0) + (self.height / 2.0);
    }
}

pub struct CellPencilFont {
    pub params: TextParams,
    pub font: Font,
    pub x_offset: f32,
    pub y_offset: f32,
    pub height: f32,
    pub width: f32,
    pub box_size: f32,
}

impl CellPencilFont {
    pub async fn new(font_path: &str) -> Self {
        let font = load_ttf_font(font_path).await.unwrap();
        let measure = measure_text("9", Some(font), 48, 1.0);
        CellPencilFont {
            font,
            params: TextParams {
                font,
                font_size: 48,
                font_scale: 1.0,
                font_scale_aspect: 1.0,
                rotation: 0.0,
                color: BLACK,
            },
            x_offset: 0.0,
            y_offset: 0.0,
            height: measure.height,
            width: measure.width,
            box_size: 0.0,
        }
    }

    pub fn update(&mut self, cell_size: f32) {
        let padding = cell_size * 0.1;
        self.box_size = (cell_size - padding) / 3.0;

        self.params.font_size = find_best_font_size(&Some(self.font), "9", self.box_size);
        let measure = measure_text("9", Some(self.font), self.params.font_size, 1.0);
        self.width = measure.width;
        self.height = measure.height;

        self.x_offset = (self.box_size / 2.0) - (self.width / 2.0) + (padding / 2.0);
        self.y_offset = (self.box_size / 2.0) + (self.height / 2.0) + (padding / 2.0);
    }
}

pub struct MenuNumberFont {
    pub params: TextParams,
    pub font: Font,
    pub height: f32,
    pub width: f32,
}

impl MenuNumberFont {
    pub async fn new(font_path: &str) -> Self {
        let font = load_ttf_font(font_path).await.unwrap();
        let measure = measure_text(ICON_UNDO, Some(font), 48, 1.0);
        MenuNumberFont {
            font,
            params: TextParams {
                font,
                font_size: 48,
                font_scale: 1.0,
                font_scale_aspect: 1.0,
                rotation: 0.0,
                color: BLACK,
            },
            height: measure.height,
            width: measure.width,
        }
    }

    pub fn update(&mut self, cell_size: f32) {
        self.params.font_size = find_best_font_size(&Some(self.font), "9", cell_size);

        let measure = measure_text("9", Some(self.font), self.params.font_size, 1.0);
        self.width = measure.width;
        self.height = measure.height;
    }
}

pub struct IconFont {
    pub params: TextParams,
    pub font: Font,
    pub height: f32,
    pub width: f32,
}

impl IconFont {
    pub async fn new(font_path: &str, color: Color) -> Self {
        let font = load_ttf_font(font_path).await.unwrap();
        let measure = measure_text(ICON_PENCIL_SLASH, Some(font), 48, 1.0);
        IconFont {
            font,
            params: TextParams {
                font,
                font_size: 48,
                font_scale: 1.0,
                font_scale_aspect: 1.0,
                rotation: 0.0,
                color,
            },
            height: measure.height,
            width: measure.width,
        }
    }

    pub fn update(&mut self, cell_size: f32) {
        self.params.font_size = find_best_font_size(&Some(self.font), ICON_PENCIL, cell_size);
        let measure = measure_text(
            ICON_PENCIL_SLASH,
            Some(self.font),
            self.params.font_size,
            1.0,
        );
        self.width = measure.width;
        self.height = measure.height;
    }
}
