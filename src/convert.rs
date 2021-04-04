// MIT License
//
// Copyright (c) 2021 Ferhat Geçdoğan All Rights Reserved.
// Distributed under the terms of the MIT License.
//
//

pub fn to_hex(r: u32, g: u32, b: u32) -> String {
    return format!("#{:>02x}{:>02x}{:>02x}", r, g, b);
}

pub fn to_cmyk(r: u32, g: u32, b: u32) -> (String, String, String, String) {
    let (mut _r, mut _g, mut _b):
        (f32, f32, f32) = (r as f32, g as f32, b as f32);

    let (w, c, m, y, k, mut max):
        (f32, f32, f32, f32, f32, f32);


    if _r == 0.0 && _g == 0.0 && _b == 0.0 {
        // See: https://github.com/rust-lang/rust/issues/71126
        // (c, m, y) = (0.0, 0.0, 0.0);
        c = 0.0;
        m = 0.0;
        y = 0.0;

        k = 100.0;
    }
    else {
        // Scroll to line: 20
        // (_r, _g, _b) = (_r / 255.0, _g / 255.0, _b / 255.0);
        _r = _r / 255.0;
        _g = _g / 255.0;
        _b = _b / 255.0;

        max = _r;

        // Get max value
        if _g > max {
            max = _g;
        }

        if _b > max {
            max = _b;
        }

        w = max;

        c = ((w - _r) / w) * 100.0;
        m = ((w - _g) / w) * 100.0;
        y = ((w - _b) / w) * 100.0;

        k = (1.0 - w) * 100.0;
    }

    return (format!("{:.2}", c),
            format!("{:.2}", m),
            format!("{:.2}", y),
            format!("{:.2}", k));
}