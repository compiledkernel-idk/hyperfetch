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

use sysinfo::{System, Disks};
use super::memory::format_bytes;

#[derive(Debug, Clone)]
pub struct DiskInfo {
    pub disks: Vec<DiskDevice>,
    pub total_bytes: u64,
    pub used_bytes: u64,
    pub usage_percent: f32,
}

#[derive(Debug, Clone)]
pub struct DiskDevice {
    pub name: String,
    pub mount_point: String,
    pub fs_type: String,
    pub total_bytes: u64,
    pub used_bytes: u64,
    pub available_bytes: u64,
    pub usage_percent: f32,
}

pub fn get_info(_sys: &System) -> DiskInfo {
    let disks = Disks::new_with_refreshed_list();
    let mut disk_devices = Vec::new();
    let mut total_bytes: u64 = 0;
    let mut used_bytes: u64 = 0;
    
    for disk in disks.list() {
        let mount_point = disk.mount_point().to_string_lossy().to_string();
        
        if mount_point.starts_with("/sys") 
            || mount_point.starts_with("/proc")
            || mount_point.starts_with("/dev")
            || mount_point.starts_with("/run")
            || mount_point.starts_with("/snap") 
        {
            continue;
        }
        
        let total = disk.total_space();
        let available = disk.available_space();
        let used = total.saturating_sub(available);
        
        if total == 0 {
            continue;
        }
        
        let usage_percent = (used as f32 / total as f32) * 100.0;
        
        disk_devices.push(DiskDevice {
            name: disk.name().to_string_lossy().to_string(),
            mount_point,
            fs_type: disk.file_system().to_string_lossy().to_string(),
            total_bytes: total,
            used_bytes: used,
            available_bytes: available,
            usage_percent,
        });
        
        total_bytes += total;
        used_bytes += used;
    }
    
    disk_devices.sort_by(|a, b| {
        if a.mount_point == "/" {
            std::cmp::Ordering::Less
        } else if b.mount_point == "/" {
            std::cmp::Ordering::Greater
        } else {
            a.mount_point.cmp(&b.mount_point)
        }
    });
    
    let usage_percent = if total_bytes > 0 {
        (used_bytes as f32 / total_bytes as f32) * 100.0
    } else {
        0.0
    };
    
    DiskInfo {
        disks: disk_devices,
        total_bytes,
        used_bytes,
        usage_percent,
    }
}

impl DiskInfo {
    pub fn display(&self) -> String {
        
        if let Some(root) = self.disks.iter().find(|d| d.mount_point == "/") {
            root.display()
        } else if let Some(first) = self.disks.first() {
            first.display()
        } else {
            "No disks found".to_string()
        }
    }
    
    pub fn display_all(&self) -> Vec<String> {
        self.disks.iter().map(|d| d.display()).collect()
    }
}

impl DiskDevice {
    pub fn display(&self) -> String {
        format!(
            "{} / {} ({:.0}%)",
            format_bytes(self.used_bytes),
            format_bytes(self.total_bytes),
            self.usage_percent
        )
    }
    
    pub fn display_with_mount(&self) -> String {
        format!("{}: {}", self.mount_point, self.display())
    }
}
