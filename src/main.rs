// MIT License
//
// Copyright (c) 2021 Ferhat Geçdoğan All Rights Reserved.
// Distributed under the terms of the MIT License.
//
//

mod table;

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
        color_data: "\x1b[48;2;".to_string(),
        table_item: "░░".to_string(),
        reset: "\x1b[0m".to_string(),

        light_gray: "".to_string(),
        white: "".to_string()
    };

    colin.light_gray = colin.set_color(171u32, 171u32, 171u32);
    colin.white      = colin.set_color(255u32, 255u32, 255u32);

    let (r, g, b): (u32, u32, u32) = (
        args.get(1).unwrap().parse::<u32>().unwrap(),
        args.get(2).unwrap().parse::<u32>().unwrap(),
        args.get(3).unwrap().parse::<u32>().unwrap());


    colin.init(r, g, b);
    colin.print_box();
}