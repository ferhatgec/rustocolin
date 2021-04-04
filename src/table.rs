// MIT License
//
// Copyright (c) 2021 Ferhat Geçdoğan All Rights Reserved.
// Distributed under the terms of the MIT License.
//
//

pub struct Colin {
    pub color: String,
    pub color_data: String,
    pub table_item: String,
    pub reset: String,

    pub light_gray: String,
    pub white: String
}

impl Colin {
    pub fn set_color(&mut self, r: u32, g: u32, b: u32) -> String {
        return format!("{}{};{};{}m", &self.color, r, g, b);
    }

    pub fn table_light_gray(&mut self) {
        print!("{}{}{}{}{}", self.light_gray, self.table_item, self.white, self.table_item, self.reset);
    }

    pub fn table_white(&mut self) {
        print!("{}{}{}{}{}", self.white, self.table_item, self.light_gray, self.table_item, self.reset);
    }

    pub fn table_color(&mut self) {
        print!("{}{}{}", self.color_data, self.table_item, self.reset);
    }

    pub fn newline() {
        println!();
    }

    pub fn init(&mut self, r: u32, g: u32, b: u32) {
        self.color_data = self.set_color(r, g, b);
    }

    pub fn print_color_box(&mut self, split: bool) {
        if split {
            self.table_light_gray();
        }
        else {
            self.table_white();
        }

        for _ in 0..6 {
            self.table_color();
        }

        if !split {
            self.table_light_gray();
        }
        else {
            self.table_white();
        }

        Colin::newline();
    }

    pub fn print_box(&mut self) {
        let mut split = false;

        for _ in 0..5 {
            self.table_light_gray();
        }

        Colin::newline();

        for _ in 0..5 {
            self.table_white();
        }

        Colin::newline();

        for _ in 0..5 {
            self.print_color_box(split);
            split = !split;
        }

        for _ in 0..5 {
            self.table_light_gray();
        }

        Colin::newline();

        for _ in 0..5 {
            self.table_white();
        }

        Colin::newline();
    }
}