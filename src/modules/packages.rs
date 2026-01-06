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
use std::path::Path;

#[derive(Debug, Clone)]
pub struct PackageInfo {
    pub managers: Vec<PackageManager>,
    pub total: usize,
}

#[derive(Debug, Clone)]
pub struct PackageManager {
    pub name: String,
    pub count: usize,
}

pub fn get_info() -> PackageInfo {
    let mut managers = Vec::new();
    
    let checks: Vec<(&str, Box<dyn Fn() -> Option<usize>>)> = vec![
        ("pacman", Box::new(count_pacman)),
        ("dpkg", Box::new(count_dpkg)),
        ("rpm", Box::new(count_rpm)),
        ("flatpak", Box::new(count_flatpak)),
        ("snap", Box::new(count_snap)),
        ("nix", Box::new(count_nix)),
        ("cargo", Box::new(count_cargo)),
        ("brew", Box::new(count_brew)),
        ("pip", Box::new(count_pip)),
        ("npm", Box::new(count_npm)),
        ("apk", Box::new(count_apk)),
        ("xbps", Box::new(count_xbps)),
        ("emerge", Box::new(count_emerge)),
        ("eopkg", Box::new(count_eopkg)),
    ];
    
    for (name, count_fn) in checks {
        if let Some(count) = count_fn() {
            if count > 0 {
                managers.push(PackageManager {
                    name: name.to_string(),
                    count,
                });
            }
        }
    }
    
    let total = managers.iter().map(|m| m.count).sum();
    
    PackageInfo { managers, total }
}

fn count_pacman() -> Option<usize> {
    let db_path = Path::new("/var/lib/pacman/local");
    if !db_path.exists() {
        return None;
    }
    
    fs::read_dir(db_path)
        .ok()
        .map(|entries| entries.filter(|e| e.is_ok()).count().saturating_sub(1)) 
}

fn count_dpkg() -> Option<usize> {
    let status_file = Path::new("/var/lib/dpkg/status");
    if !status_file.exists() {
        return None;
    }
    
    let content = fs::read_to_string(status_file).ok()?;
    let count = content.lines()
        .filter(|line| line.starts_with("Status: install ok installed"))
        .count();
    
    Some(count)
}

fn count_rpm() -> Option<usize> {
    if !Path::new("/var/lib/rpm").exists() {
        return None;
    }
    
    let output = Command::new("rpm")
        .args(["-qa", "--last"])
        .output()
        .ok()?;
    
    if !output.status.success() {
        return None;
    }
    
    Some(String::from_utf8_lossy(&output.stdout).lines().count())
}

fn count_flatpak() -> Option<usize> {
    let output = Command::new("flatpak")
        .args(["list", "--app"])
        .output()
        .ok()?;
    
    if !output.status.success() {
        return None;
    }
    
    Some(String::from_utf8_lossy(&output.stdout).lines().count())
}

fn count_snap() -> Option<usize> {
    let output = Command::new("snap")
        .args(["list"])
        .output()
        .ok()?;
    
    if !output.status.success() {
        return None;
    }
    
    Some(String::from_utf8_lossy(&output.stdout).lines().count().saturating_sub(1))
}

fn count_nix() -> Option<usize> {
    
    let user_profile = dirs_next().map(|h| format!("{}/.nix-profile/manifest.nix", h));
    
    if user_profile.is_none() {
        return None;
    }
    
    let output = Command::new("nix-store")
        .args(["-qR", &user_profile.unwrap()])
        .output()
        .ok()?;
    
    if !output.status.success() {
        return None;
    }
    
    Some(String::from_utf8_lossy(&output.stdout).lines().count())
}

fn dirs_next() -> Option<String> {
    std::env::var("HOME").ok()
}

fn count_cargo() -> Option<usize> {
    let home = std::env::var("HOME").ok()?;
    let cargo_bin = Path::new(&home).join(".cargo/bin");
    
    if !cargo_bin.exists() {
        return None;
    }
    
    fs::read_dir(cargo_bin)
        .ok()
        .map(|entries| entries.filter(|e| e.is_ok()).count())
}

fn count_brew() -> Option<usize> {
    let output = Command::new("brew")
        .args(["list", "--formula", "-1"])
        .output()
        .ok()?;
    
    if !output.status.success() {
        return None;
    }
    
    Some(String::from_utf8_lossy(&output.stdout).lines().count())
}

fn count_pip() -> Option<usize> {
    
    let output = Command::new("pip")
        .args(["list", "--format=freeze"])
        .output()
        .ok()?;
    
    if !output.status.success() {
        return None;
    }
    
    let count = String::from_utf8_lossy(&output.stdout).lines().count();
    if count > 5 { 
        Some(count)
    } else {
        None
    }
}

fn count_npm() -> Option<usize> {
    let output = Command::new("npm")
        .args(["list", "-g", "--depth=0"])
        .output()
        .ok()?;
    
    if !output.status.success() {
        return None;
    }
    
    Some(String::from_utf8_lossy(&output.stdout).lines().count().saturating_sub(1))
}

fn count_apk() -> Option<usize> {
    if !Path::new("/etc/apk").exists() {
        return None;
    }
    
    let output = Command::new("apk")
        .args(["info"])
        .output()
        .ok()?;
    
    if !output.status.success() {
        return None;
    }
    
    Some(String::from_utf8_lossy(&output.stdout).lines().count())
}

fn count_xbps() -> Option<usize> {
    let output = Command::new("xbps-query")
        .args(["-l"])
        .output()
        .ok()?;
    
    if !output.status.success() {
        return None;
    }
    
    Some(String::from_utf8_lossy(&output.stdout).lines().count())
}

fn count_emerge() -> Option<usize> {
    let world_path = Path::new("/var/lib/portage/world");
    if !world_path.exists() {
        return None;
    }
    
    fs::read_to_string(world_path)
        .ok()
        .map(|content| content.lines().count())
}

fn count_eopkg() -> Option<usize> {
    let output = Command::new("eopkg")
        .args(["list-installed", "-N"])
        .output()
        .ok()?;
    
    if !output.status.success() {
        return None;
    }
    
    Some(String::from_utf8_lossy(&output.stdout).lines().count())
}

impl PackageInfo {
    pub fn display(&self) -> String {
        if self.managers.is_empty() {
            return "Unknown".to_string();
        }
        
        let details: Vec<String> = self.managers.iter()
            .map(|m| format!("{} ({})", m.count, m.name))
            .collect();
        
        details.join(", ")
    }
    
    pub fn display_total(&self) -> String {
        format!("{}", self.total)
    }
}
