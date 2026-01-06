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
pub struct KernelInfo {
    pub version: String,
    pub release: String,
    pub arch: String,
}

pub fn get_info() -> KernelInfo {
    let release = fs::read_to_string("/proc/sys/kernel/osrelease")
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|_| "unknown".to_string());
    
    let version = fs::read_to_string("/proc/sys/kernel/version")
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|_| "unknown".to_string());
    
    let arch = std::env::consts::ARCH.to_string();
    
    KernelInfo {
        version,
        release,
        arch,
    }
}

impl KernelInfo {
    pub fn display(&self) -> String {
        self.release.clone()
    }
}
