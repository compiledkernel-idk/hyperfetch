/*
* Copyright (C) 2026 compiledkernel-idk <https://github.com/compiledkernel-idk>
*
* This program is free software: you can redistribute it and/or modify
* it under the terms of the GNU General Public License as published by
* the Free Software Foundation, either version 3 of the License, or
* (at your option) any later version.
*
* This program is distributed in the hope that it will be useful,
* but WITHOUT ANY WARRANTY; without even the implied warranty of
* MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
* GNU General Public License for more details.
*
* You should have received a copy of the GNU General Public License
* along with this program. If not, see <https://www.gnu.org/licenses/>
*/

use super::colors::Theme;
use colored::*;

pub fn create_bar(percentage: f32, width: usize, theme: &Theme) -> String {
    let filled = (percentage / 100.0 * width as f32) as usize;
    let filled = filled.min(width);
    let empty = width.saturating_sub(filled);
    
    let bar_color = if percentage < 50.0 {
        theme.bar_good_color
    } else if percentage < 80.0 {
        theme.bar_warn_color
    } else {
        theme.bar_bad_color
    };
    
    let filled_str = "█".repeat(filled).color(bar_color).to_string();
    let empty_str = "░".repeat(empty).truecolor(80, 80, 80).to_string();
    
    format!("[{}{}] {:.0}%", filled_str, empty_str, percentage)
}

pub fn create_simple_bar(percentage: f32, width: usize) -> String {
    let filled = (percentage / 100.0 * width as f32) as usize;
    let filled = filled.min(width);
    let empty = width.saturating_sub(filled);
    
    format!("{}{}",
        "█".repeat(filled),
        "░".repeat(empty)
    )
}

pub fn create_custom_bar(percentage: f32, width: usize, filled_char: char, empty_char: char) -> String {
    let filled = (percentage / 100.0 * width as f32) as usize;
    let filled = filled.min(width);
    let empty = width.saturating_sub(filled);
    
    format!("{}{}",
        filled_char.to_string().repeat(filled),
        empty_char.to_string().repeat(empty)
    )
}
