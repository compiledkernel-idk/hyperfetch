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
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct OsInfo {
    pub name: String,
    pub distro_id: String,
    pub version: String,
    pub codename: String,
    pub arch: String,
    pub pretty_name: String,
}

pub fn get_info() -> OsInfo {
    let os_release = parse_os_release();
    let arch = std::env::consts::ARCH.to_string();
    
    let name = os_release.get("NAME")
        .cloned()
        .unwrap_or_else(|| std::env::consts::OS.to_string());
    
    let distro_id = os_release.get("ID")
        .cloned()
        .unwrap_or_else(|| "linux".to_string())
        .to_lowercase();
    
    let version = os_release.get("VERSION_ID")
        .or_else(|| os_release.get("VERSION"))
        .cloned()
        .unwrap_or_default();
    
    let codename = os_release.get("VERSION_CODENAME")
        .cloned()
        .unwrap_or_default();
    
    let pretty_name = os_release.get("PRETTY_NAME")
        .cloned()
        .unwrap_or_else(|| format!("{} {}", name, version));
    
    OsInfo {
        name,
        distro_id,
        version,
        codename,
        arch,
        pretty_name,
    }
}

fn parse_os_release() -> HashMap<String, String> {
    let mut map = HashMap::new();
    
    let content = fs::read_to_string("/etc/os-release")
        .or_else(|_| fs::read_to_string("/usr/lib/os-release"))
        .unwrap_or_default();
    
    for line in content.lines() {
        if let Some((key, value)) = line.split_once('=') {
            let value = value.trim_matches('"').to_string();
            map.insert(key.to_string(), value);
        }
    }
    
    map
}

impl OsInfo {
    pub fn display(&self) -> String {
        format!("{} {}", self.pretty_name, self.arch)
    }
}
