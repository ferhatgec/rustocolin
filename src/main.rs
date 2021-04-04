// MIT License
//
// Copyright (c) 2021 Ferhat Geçdoğan All Rights Reserved.
// Distributed under the terms of the MIT License.
//
//

mod table;
mod convert;

fn main() {
    let args: Vec<_> = std::env::args().collect();

    if args.len() < 4 {
        print!("Fegeya Colin : CLI Color info tool\n\
            {sep}\n\
            {command} {{r}} {{g}} {{b}}\n\
            {sep}\n\
            {command} 255 255 255",
            command = args.get(0).unwrap(),
            sep = "----");

        std::process::exit(1);
    }

    let mut colin = crate::table::Colin {
        color: "\x1b[48;2;".to_string(),
        fg_color: "\x1b[38;2;".to_string(),

        color_data: "\x1b[48;2;".to_string(),

        table_item: "░░".to_string(),

        reset: "\x1b[0m".to_string(),

        light_gray: "".to_string(),
        white: "".to_string(),

        red: "".to_string(),
        orange: "".to_string(),
        yellow: "".to_string(),
        green: "".to_string(),
        blue: "".to_string(),
        purple: "".to_string(),
        pink: "".to_string(),

        info_table: vec![
            "name: ".to_string(),
            "-----".to_string(),
            "\x1b[0;31mr\x1b[0;32mg\x1b[0;34mb\x1b[0m  : ".to_string(),
            "hex  : ".to_string(),
            "cmyk : ".to_string(),
            "hsl  : ".to_string(),
            "hsv  : ".to_string(),
            "-----".to_string(),
            "ascii: ".to_string(),
            "esc  : ".to_string()
        ],

        line: 0,

        r: 0,
        g: 0,
        b: 0,

        hex: "".to_string(),
        cmyk: ("".to_string(), "".to_string(), "".to_string(), "".to_string())
    };

    colin.light_gray = colin.set_color(171u32, 171u32, 171u32);
    colin.white      = colin.set_color(255u32, 255u32, 255u32);

    // Rainbow colors
    colin.red        = colin.set_fg_color(255, 0  , 0);
    colin.orange     = colin.set_fg_color(255, 165, 0);
    colin.yellow     = colin.set_fg_color(255, 255, 0);
    colin.green      = colin.set_fg_color(0  , 128, 0);
    colin.blue       = colin.set_fg_color(0  , 0, 255);
    colin.purple     = colin.set_fg_color(75 , 0, 130);
    colin.pink       = colin.set_fg_color(238,130,238);

    let (r, g, b): (u32, u32, u32) = (
        args.get(1).unwrap().parse::<u32>().unwrap(),
        args.get(2).unwrap().parse::<u32>().unwrap(),
        args.get(3).unwrap().parse::<u32>().unwrap());


    colin.init(r, g, b);
    colin.print_box();
}