// MIT License
//
// Copyright (c) 2021 Ferhat Geçdoğan All Rights Reserved.
// Distributed under the terms of the MIT License.
//
//

pub struct Colin<'a> {
    pub color: String,
    pub color_data: String,
    pub table_item: String,
    pub reset: String,

    pub light_gray: String,
    pub white: String,

    pub info_table: Vec<&'a str>,

    pub line: usize
}

impl Colin<'_> {
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

    pub fn newline(&mut self) {
        if (self.line as usize) < self.info_table.len() {
            print!("{}{}", "   ", self.info_table.get(self.line).unwrap());

            match self.line {
                1 => self.name_function(),
                2 => self.hmm_function(),
                3 => self.rgb_function(),
                4 => self.hex_function(),
                5 => self.cmyk_function(),
                6 => self.hsl_function(),
                7 => self.hsv_function(),
                _ => {}
            }

            self.line += 1;
        }

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

        for _ in 0..8 {
            self.table_color();
        }

        if !split {
            self.table_light_gray();
        }
        else {
            self.table_white();
        }

        self.newline();
    }

    pub fn print_box(&mut self) {
        let mut split = false;

        for _ in 0..6 {
            self.table_light_gray();
        }

        self.newline();

        for _ in 0..6 {
            self.table_white();
        }

        self.newline();

        for _ in 0..6 {
            self.print_color_box(split);
            split = !split;
        }

        for _ in 0..6 {
            self.table_light_gray();
        }

        self.newline();

        for _ in 0..6 {
            self.table_white();
        }

        self.newline();
    }

    pub fn name_function(&mut self) {}
    pub fn hmm_function(&mut self) {}
    pub fn rgb_function(&mut self) {}
    pub fn hex_function(&mut self) {}
    pub fn cmyk_function(&mut self){}
    pub fn hsl_function(&mut self) {}
    pub fn hsv_function(&mut self) {}
}