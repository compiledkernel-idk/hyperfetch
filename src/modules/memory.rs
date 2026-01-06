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

use sysinfo::System;

#[derive(Debug, Clone)]
pub struct MemoryInfo {
    pub total_bytes: u64,
    pub used_bytes: u64,
    pub free_bytes: u64,
    pub available_bytes: u64,
    pub swap_total_bytes: u64,
    pub swap_used_bytes: u64,
    pub usage_percent: f32,
    pub swap_usage_percent: f32,
}

pub fn get_info(sys: &System) -> MemoryInfo {
    let total = sys.total_memory();
    let used = sys.used_memory();
    let free = sys.free_memory();
    let available = sys.available_memory();
    let swap_total = sys.total_swap();
    let swap_used = sys.used_swap();
    
    let usage_percent = if total > 0 {
        (used as f32 / total as f32) * 100.0
    } else {
        0.0
    };
    
    let swap_usage_percent = if swap_total > 0 {
        (swap_used as f32 / swap_total as f32) * 100.0
    } else {
        0.0
    };
    
    MemoryInfo {
        total_bytes: total,
        used_bytes: used,
        free_bytes: free,
        available_bytes: available,
        swap_total_bytes: swap_total,
        swap_used_bytes: swap_used,
        usage_percent,
        swap_usage_percent,
    }
}

impl MemoryInfo {
    pub fn display(&self) -> String {
        format!(
            "{} / {}",
            format_bytes(self.used_bytes),
            format_bytes(self.total_bytes)
        )
    }
    
    pub fn display_with_percent(&self) -> String {
        format!("{} ({:.0}%)", self.display(), self.usage_percent)
    }
}

pub fn format_bytes(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;
    const TB: u64 = GB * 1024;
    
    if bytes >= TB {
        format!("{:.1} TiB", bytes as f64 / TB as f64)
    } else if bytes >= GB {
        format!("{:.1} GiB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.0} MiB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.0} KiB", bytes as f64 / KB as f64)
    } else {
        format!("{} B", bytes)
    }
}
