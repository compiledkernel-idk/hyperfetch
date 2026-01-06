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

use std::process::Command;
use std::fs;

#[derive(Debug, Clone)]
pub struct DisplayInfo {
    pub displays: Vec<Display>,
}

#[derive(Debug, Clone)]
pub struct Display {
    pub name: String,
    pub width: u32,
    pub height: u32,
    pub refresh_rate: Option<f32>,
    pub primary: bool,
}

pub fn get_info() -> DisplayInfo {
    let mut displays = Vec::new();
    
    if let Some(xrandr_displays) = parse_xrandr() {
        displays.extend(xrandr_displays);
    }
    
    if displays.is_empty() {
        if let Some(wlr_displays) = parse_wlr_randr() {
            displays.extend(wlr_displays);
        }
    }
    
    if displays.is_empty() {
        if let Some(drm_displays) = parse_drm() {
            displays.extend(drm_displays);
        }
    }
    
    if displays.is_empty() {
        displays.push(Display {
            name: "Unknown".to_string(),
            width: 0,
            height: 0,
            refresh_rate: None,
            primary: true,
        });
    }
    
    DisplayInfo { displays }
}

fn parse_xrandr() -> Option<Vec<Display>> {
    let output = Command::new("xrandr")
        .arg("--query")
        .output()
        .ok()?;
    
    if !output.status.success() {
        return None;
    }
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut displays = Vec::new();
    
    for line in stdout.lines() {
        if line.contains(" connected") {
            if let Some(display) = parse_xrandr_line(line) {
                displays.push(display);
            }
        }
    }
    
    if displays.is_empty() {
        None
    } else {
        Some(displays)
    }
}

fn parse_xrandr_line(line: &str) -> Option<Display> {
    let parts: Vec<&str> = line.split_whitespace().collect();
    let name = parts.first()?.to_string();
    let primary = line.contains("primary");
    
    for part in &parts {
        if part.contains('x') && part.contains('+') {
            let res_part = part.split('+').next()?;
            let dims: Vec<&str> = res_part.split('x').collect();
            if dims.len() == 2 {
                let width: u32 = dims[0].parse().ok()?;
                let height: u32 = dims[1].parse().ok()?;
                
                let refresh_rate = extract_refresh_rate(&parts);
                
                return Some(Display {
                    name,
                    width,
                    height,
                    refresh_rate,
                    primary,
                });
            }
        }
    }
    
    None
}

fn extract_refresh_rate(parts: &[&str]) -> Option<f32> {
    for part in parts {
        if part.ends_with("Hz") || part.ends_with('*') {
            let cleaned = part.trim_end_matches(|c| c == '*' || c == '+');
            if let Ok(rate) = cleaned.parse::<f32>() {
                return Some(rate);
            }
        }
    }
    None
}

fn parse_wlr_randr() -> Option<Vec<Display>> {
    let output = Command::new("wlr-randr")
        .output()
        .ok()?;
    
    if !output.status.success() {
        return None;
    }
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut displays = Vec::new();
    let mut current_name = String::new();
    let mut current_width = 0u32;
    let mut current_height = 0u32;
    let mut current_refresh: Option<f32> = None;
    
    for line in stdout.lines() {
        if !line.starts_with(' ') && !line.is_empty() {
            
            if !current_name.is_empty() && current_width > 0 {
                displays.push(Display {
                    name: current_name.clone(),
                    width: current_width,
                    height: current_height,
                    refresh_rate: current_refresh,
                    primary: displays.is_empty(),
                });
            }
            current_name = line.split_whitespace().next()?.to_string();
            current_width = 0;
            current_height = 0;
            current_refresh = None;
        } else if line.contains("current") {
            
            let trimmed = line.trim();
            if let Some(res) = trimmed.split(" px").next() {
                let dims: Vec<&str> = res.split('x').collect();
                if dims.len() == 2 {
                    current_width = dims[0].trim().parse().unwrap_or(0);
                    current_height = dims[1].trim().parse().unwrap_or(0);
                }
            }
            if let Some(hz_part) = trimmed.split(" Hz").next() {
                if let Some(rate_str) = hz_part.split(',').nth(1) {
                    current_refresh = rate_str.trim().parse().ok();
                }
            }
        }
    }
    
    if !current_name.is_empty() && current_width > 0 {
        displays.push(Display {
            name: current_name,
            width: current_width,
            height: current_height,
            refresh_rate: current_refresh,
            primary: displays.is_empty(),
        });
    }
    
    if displays.is_empty() {
        None
    } else {
        Some(displays)
    }
}

fn parse_drm() -> Option<Vec<Display>> {
    let mut displays = Vec::new();
    
    if let Ok(entries) = fs::read_dir("/sys/class/drm") {
        for entry in entries.flatten() {
            let path = entry.path();
            let name = entry.file_name().to_string_lossy().to_string();
            
            if name.starts_with("card") && name.contains('-') {
                let modes_path = path.join("modes");
                if let Ok(modes) = fs::read_to_string(modes_path) {
                    if let Some(first_mode) = modes.lines().next() {
                        let dims: Vec<&str> = first_mode.split('x').collect();
                        if dims.len() == 2 {
                            if let (Ok(width), Ok(height)) = (dims[0].parse(), dims[1].parse()) {
                                displays.push(Display {
                                    name: name.replace("card0-", "").replace("card1-", ""),
                                    width,
                                    height,
                                    refresh_rate: None,
                                    primary: displays.is_empty(),
                                });
                            }
                        }
                    }
                }
            }
        }
    }
    
    if displays.is_empty() {
        None
    } else {
        Some(displays)
    }
}

impl DisplayInfo {
    pub fn display(&self) -> String {
        self.displays.iter()
            .map(|d| d.display())
            .collect::<Vec<_>>()
            .join(", ")
    }
    
    pub fn primary(&self) -> Option<&Display> {
        self.displays.iter().find(|d| d.primary)
            .or_else(|| self.displays.first())
    }
}

impl Display {
    pub fn display(&self) -> String {
        if let Some(rate) = self.refresh_rate {
            format!("{}x{} @ {:.0}Hz", self.width, self.height, rate)
        } else {
            format!("{}x{}", self.width, self.height)
        }
    }
}
