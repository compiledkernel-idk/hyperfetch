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
pub struct GpuInfo {
    pub gpus: Vec<GpuDevice>,
}

#[derive(Debug, Clone)]
pub struct GpuDevice {
    pub vendor: String,
    pub model: String,
    pub driver: Option<String>,
    pub vram_mb: Option<u64>,
}

pub fn get_info() -> GpuInfo {
    let mut gpus = Vec::new();
    
    if let Some(lspci_gpus) = parse_lspci() {
        gpus.extend(lspci_gpus);
    }
    
    if gpus.is_empty() {
        if let Some(sysfs_gpus) = parse_sysfs() {
            gpus.extend(sysfs_gpus);
        }
    }
    
    if gpus.is_empty() {
        gpus.push(GpuDevice {
            vendor: "Unknown".to_string(),
            model: "Unknown GPU".to_string(),
            driver: None,
            vram_mb: None,
        });
    }
    
    GpuInfo { gpus }
}

fn parse_lspci() -> Option<Vec<GpuDevice>> {
    let output = Command::new("lspci")
        .args(["-mm", "-nn"])
        .output()
        .ok()?;
    
    if !output.status.success() {
        return None;
    }
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut gpus = Vec::new();
    
    for line in stdout.lines() {
        let lower = line.to_lowercase();
        if lower.contains("vga") || lower.contains("3d") || lower.contains("display") {
            if let Some(gpu) = parse_lspci_line(line) {
                gpus.push(gpu);
            }
        }
    }
    
    if gpus.is_empty() {
        None
    } else {
        Some(gpus)
    }
}

fn parse_lspci_line(line: &str) -> Option<GpuDevice> {
    
    let parts: Vec<&str> = line.split('"').collect();
    
    if parts.len() >= 6 {
        let vendor = parts.get(3).unwrap_or(&"Unknown").to_string();
        let model = parts.get(5).unwrap_or(&"Unknown GPU").to_string();
        
        let model = model.replace("Device ", "").trim().to_string();
        let model = if model.is_empty() || model.chars().all(|c| c.is_ascii_hexdigit()) {
            format!("{} Graphics", vendor)
        } else {
            model
        };
        
        return Some(GpuDevice {
            vendor,
            model,
            driver: get_gpu_driver(),
            vram_mb: None,
        });
    }
    
    None
}

fn parse_sysfs() -> Option<Vec<GpuDevice>> {
    let mut gpus = Vec::new();
    
    if let Ok(entries) = fs::read_dir("/sys/class/drm") {
        for entry in entries.flatten() {
            let path = entry.path();
            let name = entry.file_name().to_string_lossy().to_string();
            
            if name.starts_with("card") && !name.contains('-') {
                
                let device_path = path.join("device");
                if device_path.exists() {
                    let vendor = fs::read_to_string(device_path.join("vendor"))
                        .map(|s| parse_pci_vendor(s.trim()))
                        .unwrap_or_else(|_| "Unknown".to_string());
                    
                    gpus.push(GpuDevice {
                        vendor: vendor.clone(),
                        model: format!("{} Graphics", vendor),
                        driver: get_gpu_driver(),
                        vram_mb: None,
                    });
                }
            }
        }
    }
    
    if gpus.is_empty() {
        None
    } else {
        Some(gpus)
    }
}

fn parse_pci_vendor(vendor_id: &str) -> String {
    match vendor_id {
        "0x8086" => "Intel".to_string(),
        "0x10de" => "NVIDIA".to_string(),
        "0x1002" => "AMD".to_string(),
        _ => "Unknown".to_string(),
    }
}

fn get_gpu_driver() -> Option<String> {
    
    if fs::metadata("/sys/module/nvidia").is_ok() {
        return Some("nvidia".to_string());
    }
    if fs::metadata("/sys/module/amdgpu").is_ok() {
        return Some("amdgpu".to_string());
    }
    if fs::metadata("/sys/module/i915").is_ok() {
        return Some("i915".to_string());
    }
    if fs::metadata("/sys/module/nouveau").is_ok() {
        return Some("nouveau".to_string());
    }
    None
}

impl GpuInfo {
    pub fn display(&self) -> String {
        self.gpus.iter()
            .map(|g| g.display())
            .collect::<Vec<_>>()
            .join(", ")
    }
}

impl GpuDevice {
    pub fn display(&self) -> String {
        if let Some(driver) = &self.driver {
            format!("{} [{}]", self.model, driver)
        } else {
            self.model.clone()
        }
    }
}
