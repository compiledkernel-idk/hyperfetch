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
use std::fs;

#[derive(Debug, Clone)]
pub struct CpuInfo {
    pub model: String,
    pub vendor: String,
    pub cores: usize,
    pub threads: usize,
    pub frequency_mhz: u64,
    pub usage_percent: f32,
    pub temperature: Option<f32>,
}

pub fn get_info(sys: &System) -> CpuInfo {
    let cpus = sys.cpus();
    
    let model = cpus.first()
        .map(|c| c.brand().to_string())
        .unwrap_or_else(|| "Unknown CPU".to_string());
    
    let vendor = cpus.first()
        .map(|c| c.vendor_id().to_string())
        .unwrap_or_else(|| "Unknown".to_string());
    
    let threads = cpus.len();
    let cores = sys.physical_core_count().unwrap_or(threads);
    
    let frequency_mhz = cpus.first()
        .map(|c| c.frequency())
        .unwrap_or(0);
    
    let usage_percent = cpus.iter()
        .map(|c| c.cpu_usage())
        .sum::<f32>() / threads as f32;
    
    let temperature = read_cpu_temperature();
    
    CpuInfo {
        model,
        vendor,
        cores,
        threads,
        frequency_mhz,
        usage_percent,
        temperature,
    }
}

fn read_cpu_temperature() -> Option<f32> {
    
    let thermal_zones = [
        "/sys/class/thermal/thermal_zone0/temp",
        "/sys/class/hwmon/hwmon0/temp1_input",
        "/sys/class/hwmon/hwmon1/temp1_input",
        "/sys/class/hwmon/hwmon2/temp1_input",
    ];
    
    for path in &thermal_zones {
        if let Ok(content) = fs::read_to_string(path) {
            if let Ok(millidegrees) = content.trim().parse::<i64>() {
                return Some(millidegrees as f32 / 1000.0);
            }
        }
    }
    
    None
}

impl CpuInfo {
    pub fn display(&self) -> String {
        let freq_ghz = self.frequency_mhz as f64 / 1000.0;
        format!("{} ({}) @ {:.2}GHz", self.model, self.threads, freq_ghz)
    }
    
    pub fn display_with_temp(&self) -> String {
        match self.temperature {
            Some(temp) => format!("{} [{}°C]", self.display(), temp as i32),
            None => self.display(),
        }
    }
}
