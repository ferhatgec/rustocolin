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
    let (mut _r, mut _g, mut _b) = (r as f32, g as f32, b as f32);

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

    (format!("{:.2}", c),
            format!("{:.2}", m),
            format!("{:.2}", y),
            format!("{:.2}", k))
}

pub fn get_min(r: f32, g: f32, b: f32) -> f32 {
    let mut min = r;

    if g < min {
        min = g;
    }

    if b < min {
        min = b;
    }

    min
}

pub fn get_max(r: f32, g: f32, b: f32) -> f32 {
    let mut max = r;

    if g > max {
        max = g;
    }

    if b > max {
        max = b;
    }

    max
}

pub fn to_hsl(r: u32, g: u32, b: u32) -> (String, String, String) {
    let (mut _r, mut _g, mut _b) = (r as f32, g as f32, b as f32);

    let mut h = 0.0;
    let (s, l, min, max): (f32, f32, f32, f32);

    _r = _r / 255.0;
    _g = _g / 255.0;
    _b = _b / 255.0;

    min = get_min(_r, _g, _b);
    max = get_max(_r, _g, _b);

    l = 50.0 * (min + max);

    if min == max {
        s = 0.0;
        h = 0.0;
    }
    else if l < 50.0 {
        s = 100.0 * (max - min) / (max + min);
    }
    else {
        s = 100.0 * (max - min) / (2.0 - max - min);
    }

    if _r - _g == 0.0 || _g - _b == 0.0 {
        h = 0.0;
    }
    else if _r == max {
        h = 60.0 * (_g - _b) / (max - min);
    }
    else if _g == max {
        h = 60.0 * (_b - _r) / (max - min) + 120.0;
    }
    else if _b == max {
        h = 60.0 * (_r - _g) / (max - min) + 240.0;
    }

    if h < 0.0 {
        h = h + 360.0;
    }

    (format!("{:.2}", h),
     format!("{:.2}", s),
     format!("{:.2}", l))
}