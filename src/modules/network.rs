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
use std::net::IpAddr;
use std::process::Command;

#[derive(Debug, Clone)]
pub struct NetworkInfo {
    pub interfaces: Vec<NetworkInterface>,
    pub local_ip: Option<String>,
}

#[derive(Debug, Clone)]
pub struct NetworkInterface {
    pub name: String,
    pub ip_addresses: Vec<String>,
    pub mac_address: Option<String>,
    pub interface_type: InterfaceType,
    pub is_up: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum InterfaceType {
    Ethernet,
    Wireless,
    Loopback,
    Virtual,
    Unknown,
}

pub fn get_info() -> NetworkInfo {
    let mut interfaces = Vec::new();
    let mut local_ip: Option<String> = None;
    
    if let Ok(entries) = fs::read_dir("/sys/class/net") {
        for entry in entries.flatten() {
            let name = entry.file_name().to_string_lossy().to_string();
            
            if name == "lo" {
                continue; 
            }
            
            let path = entry.path();
            
            let interface_type = detect_interface_type(&name, &path);
            
            let operstate = fs::read_to_string(path.join("operstate"))
                .map(|s| s.trim().to_string())
                .unwrap_or_default();
            let is_up = operstate == "up";
            
            let mac_address = fs::read_to_string(path.join("address"))
                .map(|s| s.trim().to_string())
                .ok()
                .filter(|m| m != "00:00:00:00:00:00");
            
            let ip_addresses = get_ip_addresses(&name);
            
            if is_up && local_ip.is_none() {
                if let Some(ip) = ip_addresses.first() {
                    local_ip = Some(ip.clone());
                }
            }
            
            interfaces.push(NetworkInterface {
                name,
                ip_addresses,
                mac_address,
                interface_type,
                is_up,
            });
        }
    }
    
    interfaces.sort_by(|a, b| {
        b.is_up.cmp(&a.is_up).then_with(|| a.name.cmp(&b.name))
    });
    
    NetworkInfo {
        interfaces,
        local_ip,
    }
}

fn detect_interface_type(name: &str, path: &std::path::Path) -> InterfaceType {
    if name == "lo" {
        return InterfaceType::Loopback;
    }
    
    if path.join("wireless").exists() || name.starts_with("wl") {
        return InterfaceType::Wireless;
    }
    
    if name.starts_with("eth") || name.starts_with("en") {
        return InterfaceType::Ethernet;
    }
    
    if name.starts_with("veth") || name.starts_with("docker") || name.starts_with("br-") 
        || name.starts_with("virbr") || name.starts_with("vnet") {
        return InterfaceType::Virtual;
    }
    
    InterfaceType::Unknown
}

fn get_ip_addresses(interface: &str) -> Vec<String> {
    let mut addresses = Vec::new();
    
    if let Ok(output) = Command::new("ip")
        .args(["addr", "show", interface])
        .output()
    {
        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            for line in stdout.lines() {
                let trimmed = line.trim();
                if trimmed.starts_with("inet ") {
                    
                    if let Some(addr) = trimmed.split_whitespace().nth(1) {
                        let ip = addr.split('/').next().unwrap_or(addr);
                        if let Ok(parsed) = ip.parse::<IpAddr>() {
                            if !parsed.is_loopback() {
                                addresses.push(ip.to_string());
                            }
                        }
                    }
                }
            }
        }
    }
    
    addresses
}

impl NetworkInfo {
    pub fn display(&self) -> String {
        self.local_ip.clone().unwrap_or_else(|| "Not connected".to_string())
    }
    
    pub fn display_detailed(&self) -> Vec<String> {
        self.interfaces
            .iter()
            .filter(|i| i.is_up && !i.ip_addresses.is_empty())
            .map(|i| i.display())
            .collect()
    }
}

impl NetworkInterface {
    pub fn display(&self) -> String {
        let type_str = match self.interface_type {
            InterfaceType::Wireless => "WiFi",
            InterfaceType::Ethernet => "Eth",
            _ => "",
        };
        
        if let Some(ip) = self.ip_addresses.first() {
            if type_str.is_empty() {
                format!("{}: {}", self.name, ip)
            } else {
                format!("{} ({}): {}", self.name, type_str, ip)
            }
        } else {
            format!("{}: No IP", self.name)
        }
    }
}
