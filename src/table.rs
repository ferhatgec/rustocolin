// MIT License
//
// Copyright (c) 2021 Ferhat Geçdoğan All Rights Reserved.
// Distributed under the terms of the MIT License.
//
//

pub struct Colin {
    pub color: String,
    pub fg_color: String,

    pub color_data: String,

    pub table_item: String,

    pub reset: String,

    pub light_gray: String,
    pub white: String,

    pub red: String,
    pub orange: String,
    pub yellow: String,
    pub green: String,
    pub blue: String,
    pub purple: String,
    pub pink: String,

    pub info_table: Vec<String>,

    pub line: usize,
    pub hex: String
}

impl Colin {
    pub fn set_color(&mut self, r: u32, g: u32, b: u32) -> String {
        return format!("{}{};{};{}m", &self.color, r, g, b);
    }

    pub fn set_fg_color(&mut self, r: u32, g: u32, b: u32) -> String {
        return format!("{}{};{};{}m", &self.fg_color, r, g, b);
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
                0 => self.name_function(),
                1 => self.hmm_function(),
                2 => self.rgb_function(),
                3 => self.hex_function(),
                4 => self.cmyk_function(),
                5 => self.hsl_function(),
                6 => self.hsv_function(),
                7 => self.hmm2_function(),
                8 => self.ascii_function(),
                9 => self.esc_function(),
                _ => {}
            }

            self.line += 1;
        }

        println!();
    }

    pub fn init(&mut self, r: u32, g: u32, b: u32) {
        self.color_data = self.set_color(r, g, b);

        self.hex = crate::convert::to_hex(r, g, b);

        // Name
        self.info_table[0] = format!("{}color{}: ", self.set_fg_color(r, g, b), self.reset);

        // Hex
        self.info_table[3] = format!("{}hex  : {}{}", self.red, self.orange, self.hex);

        // Cmyk
        self.info_table[4] = format!("{}cmyk : {}{}", self.orange, self.yellow, "work-in-progress");

        // Hsl
        self.info_table[5] = format!("{}hsl  : {}{}", self.yellow, self.green, "work-in-progress");

        // Hsv
        self.info_table[6] = format!("{}hsv  : {}{}", self.green, self.blue, "work-in-progress");

        // Ascii
        self.info_table[8] = format!("{}ascii: {}{}", self.blue, self.purple, "work-in-progress");

        // Esc
        self.info_table[9] = format!("{}esc  : {}{}", self.purple, self.pink, "work-in-progress");
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
    pub fn hmm2_function(&mut self) {}
    pub fn ascii_function(&mut self) {}
    pub fn esc_function(&mut self) {}
}