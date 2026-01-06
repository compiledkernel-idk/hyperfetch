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

use std::fs;

#[derive(Debug, Clone)]
pub struct UptimeInfo {
    pub total_seconds: u64,
    pub days: u64,
    pub hours: u64,
    pub minutes: u64,
    pub seconds: u64,
}

pub fn get_info() -> UptimeInfo {
    let uptime_secs = read_uptime().unwrap_or(0);
    
    let days = uptime_secs / 86400;
    let hours = (uptime_secs % 86400) / 3600;
    let minutes = (uptime_secs % 3600) / 60;
    let seconds = uptime_secs % 60;
    
    UptimeInfo {
        total_seconds: uptime_secs,
        days,
        hours,
        minutes,
        seconds,
    }
}

fn read_uptime() -> Option<u64> {
    let content = fs::read_to_string("/proc/uptime").ok()?;
    let uptime_str = content.split_whitespace().next()?;
    let uptime_float: f64 = uptime_str.parse().ok()?;
    Some(uptime_float as u64)
}

impl UptimeInfo {
    pub fn display(&self) -> String {
        let mut parts = Vec::new();
        
        if self.days > 0 {
            parts.push(format!("{} day{}", self.days, if self.days == 1 { "" } else { "s" }));
        }
        if self.hours > 0 {
            parts.push(format!("{} hour{}", self.hours, if self.hours == 1 { "" } else { "s" }));
        }
        if self.minutes > 0 {
            parts.push(format!("{} min{}", self.minutes, if self.minutes == 1 { "" } else { "s" }));
        }
        if parts.is_empty() {
            parts.push(format!("{} sec{}", self.seconds, if self.seconds == 1 { "" } else { "s" }));
        }
        
        parts.join(", ")
    }
    
    pub fn display_short(&self) -> String {
        if self.days > 0 {
            format!("{}d {}h {}m", self.days, self.hours, self.minutes)
        } else if self.hours > 0 {
            format!("{}h {}m", self.hours, self.minutes)
        } else {
            format!("{}m", self.minutes)
        }
    }
}
