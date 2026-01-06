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

pub mod colors;
pub mod progress;
pub mod icons;

use crate::modules::SystemInfo;
use crate::cli::Args;
use colors::Theme;
use unicode_width::UnicodeWidthStr;

pub fn render(logo: &[String], info: &SystemInfo, theme: &Theme, args: &Args) {
    if args.json {
        render_json(info);
        return;
    }
    
    let info_lines = build_info_lines(info, theme, args);
    let max_lines = logo.len().max(info_lines.len());
    
    let logo_width = logo.iter()
        .map(|line| strip_ansi_codes(line).width())
        .max()
        .unwrap_or(0);
    
    for i in 0..max_lines {
        
        if i < logo.len() {
            let line = &logo[i];
            let visible_width = strip_ansi_codes(line).width();
            let padding = logo_width.saturating_sub(visible_width);
            print!("{}{}", line, " ".repeat(padding));
        } else {
            print!("{}", " ".repeat(logo_width));
        }
        
        print!("  ");
        
        if i < info_lines.len() {
            println!("{}", info_lines[i]);
        } else {
            println!();
        }
    }
}

fn build_info_lines(info: &SystemInfo, theme: &Theme, args: &Args) -> Vec<String> {
    let mut lines = Vec::new();
    let use_icons = !args.no_icons;
    
    let title = format!("{}", info.user.display());
    lines.push(theme.apply_title(&title));
    lines.push(theme.apply_separator(&"─".repeat(title.width())));
    
    if use_icons {
        lines.push(format!("{} {}  {}", 
            theme.apply_label("󰣇"), 
            theme.apply_label("OS"), 
            theme.apply_value(&info.os.display())
        ));
    } else {
        lines.push(format!("{}  {}", 
            theme.apply_label("OS"), 
            theme.apply_value(&info.os.display())
        ));
    }
    
    if use_icons {
        lines.push(format!("{} {}  {}", 
            theme.apply_label(""), 
            theme.apply_label("Kernel"), 
            theme.apply_value(&info.kernel.display())
        ));
    } else {
        lines.push(format!("{}  {}", 
            theme.apply_label("Kernel"), 
            theme.apply_value(&info.kernel.display())
        ));
    }
    
    if use_icons {
        lines.push(format!("{} {}  {}", 
            theme.apply_label("󰌽"), 
            theme.apply_label("CPU"), 
            theme.apply_value(&info.cpu.display())
        ));
    } else {
        lines.push(format!("{}  {}", 
            theme.apply_label("CPU"), 
            theme.apply_value(&info.cpu.display())
        ));
    }
    
    if !info.gpu.gpus.is_empty() {
        if use_icons {
            lines.push(format!("{} {}  {}", 
                theme.apply_label(""), 
                theme.apply_label("GPU"), 
                theme.apply_value(&info.gpu.display())
            ));
        } else {
            lines.push(format!("{}  {}", 
                theme.apply_label("GPU"), 
                theme.apply_value(&info.gpu.display())
            ));
        }
    }
    
    let mem_bar = progress::create_bar(info.memory.usage_percent, 15, theme);
    if use_icons {
        lines.push(format!("{} {}  {} {}",
            theme.apply_label(""),
            theme.apply_label("Memory"),
            theme.apply_value(&info.memory.display()),
            mem_bar
        ));
    } else {
        lines.push(format!("{}  {} {}",
            theme.apply_label("Memory"),
            theme.apply_value(&info.memory.display()),
            mem_bar
        ));
    }
    
    let disk_bar = progress::create_bar(info.disk.usage_percent, 15, theme);
    if use_icons {
        lines.push(format!("{} {}  {} {}",
            theme.apply_label("󰋊"),
            theme.apply_label("Disk"),
            theme.apply_value(&info.disk.display()),
            disk_bar
        ));
    } else {
        lines.push(format!("{}  {} {}",
            theme.apply_label("Disk"),
            theme.apply_value(&info.disk.display()),
            disk_bar
        ));
    }
    
    if use_icons {
        lines.push(format!("{} {}  {}",
            theme.apply_label(""),
            theme.apply_label("Uptime"),
            theme.apply_value(&info.uptime.display())
        ));
    } else {
        lines.push(format!("{}  {}",
            theme.apply_label("Uptime"),
            theme.apply_value(&info.uptime.display())
        ));
    }
    
    if use_icons {
        lines.push(format!("{} {}  {}",
            theme.apply_label("󰆍"),
            theme.apply_label("Shell"),
            theme.apply_value(&info.shell.display())
        ));
    } else {
        lines.push(format!("{}  {}",
            theme.apply_label("Shell"),
            theme.apply_value(&info.shell.display())
        ));
    }
    
    if info.desktop.desktop_env != "Unknown" {
        if use_icons {
            lines.push(format!("{} {}  {}",
                theme.apply_label(""),
                theme.apply_label("DE"),
                theme.apply_value(&info.desktop.display())
            ));
        } else {
            lines.push(format!("{}  {}",
                theme.apply_label("DE"),
                theme.apply_value(&info.desktop.display())
            ));
        }
    }
    
    if let Some(primary) = info.display.primary() {
        if primary.width > 0 {
            if use_icons {
                lines.push(format!("{} {}  {}",
                    theme.apply_label("󰍹"),
                    theme.apply_label("Display"),
                    theme.apply_value(&primary.display())
                ));
            } else {
                lines.push(format!("{}  {}",
                    theme.apply_label("Display"),
                    theme.apply_value(&primary.display())
                ));
            }
        }
    }
    
    if info.battery.has_battery {
        if let Some(bat_display) = info.battery.display() {
            if use_icons {
                lines.push(format!("{} {}  {}",
                    theme.apply_label("󰂄"),
                    theme.apply_label("Battery"),
                    theme.apply_value(&bat_display)
                ));
            } else {
                lines.push(format!("{}  {}",
                    theme.apply_label("Battery"),
                    theme.apply_value(&bat_display)
                ));
            }
        }
    }
    
    if info.packages.total > 0 {
        if use_icons {
            lines.push(format!("{} {}  {}",
                theme.apply_label(""),
                theme.apply_label("Packages"),
                theme.apply_value(&info.packages.display())
            ));
        } else {
            lines.push(format!("{}  {}",
                theme.apply_label("Packages"),
                theme.apply_value(&info.packages.display())
            ));
        }
    }
    
    if use_icons {
        lines.push(format!("{} {}  {}",
            theme.apply_label(""),
            theme.apply_label("Terminal"),
            theme.apply_value(&info.shell.display_terminal())
        ));
    } else {
        lines.push(format!("{}  {}",
            theme.apply_label("Terminal"),
            theme.apply_value(&info.shell.display_terminal())
        ));
    }
    
    lines.push(String::new());
    
    lines.push(theme.apply_color_blocks());
    
    lines
}

fn strip_ansi_codes(s: &str) -> String {
    
    let mut result = String::new();
    let mut in_escape = false;
    
    for ch in s.chars() {
        if ch == '\x1b' {
            in_escape = true;
        } else if in_escape && ch == 'm' {
            in_escape = false;
        } else if !in_escape {
            result.push(ch);
        }
    }
    
    result
}

fn render_json(info: &SystemInfo) {
    
    println!("{{");
    println!("  \"user\": \"{}\",", info.user.display());
    println!("  \"os\": \"{}\",", info.os.display());
    println!("  \"kernel\": \"{}\",", info.kernel.display());
    println!("  \"cpu\": \"{}\",", info.cpu.display());
    println!("  \"gpu\": \"{}\",", info.gpu.display());
    println!("  \"memory\": \"{}\",", info.memory.display());
    println!("  \"disk\": \"{}\",", info.disk.display());
    println!("  \"uptime\": \"{}\",", info.uptime.display());
    println!("  \"shell\": \"{}\",", info.shell.display());
    println!("  \"desktop\": \"{}\",", info.desktop.display());
    println!("  \"packages\": \"{}\"", info.packages.display());
    println!("}}");
}
