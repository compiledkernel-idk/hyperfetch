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

use std::collections::HashMap;
use colored::Colorize;

pub fn get_logo(name: &str) -> Vec<String> {
    let normalized = name.to_lowercase().replace(' ', "_");
    
    LOGOS.get(normalized.as_str())
        .or_else(|| LOGOS.get(name))
        .or_else(|| LOGOS.get("linux"))
        .map(|logo| parse_logo(logo))
        .unwrap_or_else(|| vec!["No logo found".to_string()])
}

fn parse_logo(logo: &str) -> Vec<String> {
    let lines: Vec<String> = logo.lines().map(|line| {
        parse_color_line(line)
    }).collect();
    
    lines
}

fn parse_color_line(line: &str) -> String {
    let mut result = String::new();
    let mut chars = line.chars().peekable();
    let mut current_color = 0;
    
    while let Some(ch) = chars.next() {
        if ch == '$' {
            if let Some(&next_ch) = chars.peek() {
                if next_ch.is_ascii_digit() {
                    chars.next(); 
                    current_color = next_ch.to_digit(10).unwrap_or(0) as usize;
                    continue;
                }
            }
        }
        
        let colored_char = apply_color(ch, current_color);
        result.push_str(&colored_char);
    }
    
    result
}

fn apply_color(ch: char, color_idx: usize) -> String {
    let s = ch.to_string();
    match color_idx {
        1 => s.bright_blue().to_string(),
        2 => s.cyan().to_string(),
        3 => s.bright_cyan().to_string(),
        4 => s.green().to_string(),
        5 => s.yellow().to_string(),
        6 => s.red().to_string(),
        7 => s.magenta().to_string(),
        _ => s,
    }
}

lazy_static::lazy_static! {
    static ref LOGOS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        
        m.insert("linux", include_str!("ascii/linux.txt"));
        
        m.insert("arch", include_str!("ascii/arch.txt"));
        m.insert("manjaro", include_str!("ascii/manjaro.txt"));
        m.insert("endeavouros", include_str!("ascii/endeavouros.txt"));
        m.insert("garuda", include_str!("ascii/garuda.txt"));
        m.insert("cachyos", include_str!("ascii/cachyos.txt"));
        m.insert("artix", include_str!("ascii/artix.txt"));
        
        m.insert("debian", include_str!("ascii/debian.txt"));
        m.insert("ubuntu", include_str!("ascii/ubuntu.txt"));
        m.insert("mint", include_str!("ascii/mint.txt"));
        m.insert("linuxmint", include_str!("ascii/mint.txt"));
        m.insert("pop", include_str!("ascii/pop.txt"));
        m.insert("pop_os", include_str!("ascii/pop.txt"));
        m.insert("elementary", include_str!("ascii/elementary.txt"));
        m.insert("kali", include_str!("ascii/kali.txt"));
        m.insert("mxlinux", include_str!("ascii/mxlinux.txt"));
        
        m.insert("fedora", include_str!("ascii/fedora.txt"));
        m.insert("rhel", include_str!("ascii/rhel.txt"));
        m.insert("centos", include_str!("ascii/centos.txt"));
        m.insert("rocky", include_str!("ascii/rocky.txt"));
        m.insert("almalinux", include_str!("ascii/almalinux.txt"));
        
        m.insert("opensuse", include_str!("ascii/opensuse.txt"));
        m.insert("opensuse-tumbleweed", include_str!("ascii/opensuse.txt"));
        m.insert("opensuse-leap", include_str!("ascii/opensuse.txt"));
        
        m.insert("gentoo", include_str!("ascii/gentoo.txt"));
        m.insert("nixos", include_str!("ascii/nixos.txt"));
        m.insert("void", include_str!("ascii/void.txt"));
        m.insert("alpine", include_str!("ascii/alpine.txt"));
        m.insert("slackware", include_str!("ascii/slackware.txt"));
        
        m
    };
}
