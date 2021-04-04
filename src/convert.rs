// MIT License
//
// Copyright (c) 2021 Ferhat Geçdoğan All Rights Reserved.
// Distributed under the terms of the MIT License.
//
//

pub fn to_hex(r: u32, g: u32, b: u32) -> String {
    return format!("#{:>02x}{:>02x}{:>02x}", r, g, b);
}