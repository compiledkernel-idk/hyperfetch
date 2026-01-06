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

use colored::*;

pub struct Theme {
    pub label_color: Color,
    pub value_color: Color,
    pub title_color: Color,
    pub separator_color: Color,
    pub bar_good_color: Color,
    pub bar_warn_color: Color,
    pub bar_bad_color: Color,
}

impl Theme {
    pub fn apply_label(&self, text: &str) -> String {
        text.color(self.label_color).bold().to_string()
    }
    
    pub fn apply_value(&self, text: &str) -> String {
        text.color(self.value_color).to_string()
    }
    
    pub fn apply_title(&self, text: &str) -> String {
        text.color(self.title_color).bold().to_string()
    }
    
    pub fn apply_separator(&self, text: &str) -> String {
        text.color(self.separator_color).to_string()
    }
    
    pub fn apply_color_blocks(&self) -> String {
        let blocks = vec![
            "███".black().to_string(),
            "███".red().to_string(),
            "███".green().to_string(),
            "███".yellow().to_string(),
            "███".blue().to_string(),
            "███".magenta().to_string(),
            "███".cyan().to_string(),
            "███".white().to_string(),
        ];
        blocks.join("")
    }
}

pub fn get_theme(name: &str) -> Theme {
    match name.to_lowercase().as_str() {
        "dracula" => Theme {
            label_color: Color::Magenta,
            value_color: Color::Cyan,
            title_color: Color::BrightMagenta,
            separator_color: Color::BrightBlack,
            bar_good_color: Color::Green,
            bar_warn_color: Color::Yellow,
            bar_bad_color: Color::Red,
        },
        "nord" => Theme {
            label_color: Color::Blue,
            value_color: Color::Cyan,
            title_color: Color::BrightBlue,
            separator_color: Color::BrightBlack,
            bar_good_color: Color::Green,
            bar_warn_color: Color::Yellow,
            bar_bad_color: Color::Red,
        },
        "gruvbox" => Theme {
            label_color: Color::Yellow,
            value_color: Color::BrightYellow,
            title_color: Color::BrightRed,
            separator_color: Color::BrightBlack,
            bar_good_color: Color::Green,
            bar_warn_color: Color::Yellow,
            bar_bad_color: Color::Red,
        },
        "catppuccin" => Theme {
            label_color: Color::Magenta,
            value_color: Color::Blue,
            title_color: Color::BrightMagenta,
            separator_color: Color::BrightBlack,
            bar_good_color: Color::Green,
            bar_warn_color: Color::Yellow,
            bar_bad_color: Color::Red,
        },
        "monokai" => Theme {
            label_color: Color::BrightMagenta,
            value_color: Color::BrightGreen,
            title_color: Color::BrightYellow,
            separator_color: Color::BrightBlack,
            bar_good_color: Color::Green,
            bar_warn_color: Color::Yellow,
            bar_bad_color: Color::Red,
        },
        _ => Theme {
            
            label_color: Color::BrightBlue,
            value_color: Color::White,
            title_color: Color::BrightCyan,
            separator_color: Color::BrightBlack,
            bar_good_color: Color::Green,
            bar_warn_color: Color::Yellow,
            bar_bad_color: Color::Red,
        },
    }
}
