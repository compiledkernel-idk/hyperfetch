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

use std::env;
use std::process::Command;

#[derive(Debug, Clone)]
pub struct DesktopInfo {
    pub desktop_env: String,
    pub display_server: String,
    pub wm: Option<String>,
    pub theme: Option<String>,
    pub icons: Option<String>,
}

pub fn get_info() -> DesktopInfo {
    let desktop_env = detect_desktop_env();
    let display_server = detect_display_server();
    let wm = detect_wm();
    let theme = detect_theme(&desktop_env);
    let icons = detect_icon_theme(&desktop_env);
    
    DesktopInfo {
        desktop_env,
        display_server,
        wm,
        theme,
        icons,
    }
}

fn detect_desktop_env() -> String {
    
    if let Ok(de) = env::var("XDG_CURRENT_DESKTOP") {
        return normalize_de_name(&de);
    }
    
    if let Ok(de) = env::var("DESKTOP_SESSION") {
        return normalize_de_name(&de);
    }
    
    let checks = [
        ("KDE_FULL_SESSION", "KDE Plasma"),
        ("GNOME_DESKTOP_SESSION_ID", "GNOME"),
        ("MATE_DESKTOP_SESSION_ID", "MATE"),
        ("TDE_FULL_SESSION", "Trinity"),
        ("HYPRLAND_INSTANCE_SIGNATURE", "Hyprland"),
        ("SWAYSOCK", "Sway"),
        ("I3SOCK", "i3"),
    ];
    
    for (var, name) in &checks {
        if env::var(var).is_ok() {
            return name.to_string();
        }
    }
    
    if let Some(wm) = detect_wm() {
        return wm;
    }
    
    "Unknown".to_string()
}

fn normalize_de_name(de: &str) -> String {
    match de.to_lowercase().as_str() {
        "kde" | "plasma" | "kde-plasma" => "KDE Plasma".to_string(),
        "gnome" | "gnome-shell" | "ubuntu:gnome" => "GNOME".to_string(),
        "xfce" | "xfce4" => "Xfce".to_string(),
        "mate" => "MATE".to_string(),
        "cinnamon" | "x-cinnamon" => "Cinnamon".to_string(),
        "lxqt" => "LXQt".to_string(),
        "lxde" => "LXDE".to_string(),
        "budgie" | "budgie-desktop" | "budgie:gnome" => "Budgie".to_string(),
        "unity" => "Unity".to_string(),
        "pantheon" => "Pantheon".to_string(),
        "deepin" => "Deepin".to_string(),
        "enlightenment" => "Enlightenment".to_string(),
        "hyprland" => "Hyprland".to_string(),
        "sway" => "Sway".to_string(),
        "i3" => "i3".to_string(),
        "bspwm" => "bspwm".to_string(),
        "dwm" => "dwm".to_string(),
        "awesome" => "Awesome".to_string(),
        "openbox" => "Openbox".to_string(),
        "cosmic" => "COSMIC".to_string(),
        _ => de.to_string(),
    }
}

fn detect_display_server() -> String {
    if env::var("WAYLAND_DISPLAY").is_ok() {
        "Wayland".to_string()
    } else if env::var("DISPLAY").is_ok() {
        "X11".to_string()
    } else {
        "TTY".to_string()
    }
}

fn detect_wm() -> Option<String> {
    
    if let Ok(output) = Command::new("wmctrl").arg("-m").output() {
        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            for line in stdout.lines() {
                if line.starts_with("Name:") {
                    return Some(line.replace("Name:", "").trim().to_string());
                }
            }
        }
    }
    
    let wm_checks = [
        ("HYPRLAND_INSTANCE_SIGNATURE", "Hyprland"),
        ("SWAYSOCK", "Sway"),
        ("I3SOCK", "i3"),
    ];
    
    for (var, name) in &wm_checks {
        if env::var(var).is_ok() {
            return Some(name.to_string());
        }
    }
    
    None
}

fn detect_theme(de: &str) -> Option<String> {
    match de {
        "GNOME" | "Budgie" | "Pantheon" | "Unity" => {
            get_gsettings_value("org.gnome.desktop.interface", "gtk-theme")
        }
        "KDE Plasma" => {
            
            None 
        }
        "Xfce" => {
            get_xfconf_value("xsettings", "/Net/ThemeName")
        }
        _ => None,
    }
}

fn detect_icon_theme(de: &str) -> Option<String> {
    match de {
        "GNOME" | "Budgie" | "Pantheon" | "Unity" => {
            get_gsettings_value("org.gnome.desktop.interface", "icon-theme")
        }
        "Xfce" => {
            get_xfconf_value("xsettings", "/Net/IconThemeName")
        }
        _ => None,
    }
}

fn get_gsettings_value(schema: &str, key: &str) -> Option<String> {
    let output = Command::new("gsettings")
        .args(["get", schema, key])
        .output()
        .ok()?;
    
    if output.status.success() {
        let value = String::from_utf8_lossy(&output.stdout)
            .trim()
            .trim_matches('\'')
            .to_string();
        if !value.is_empty() {
            return Some(value);
        }
    }
    None
}

fn get_xfconf_value(channel: &str, property: &str) -> Option<String> {
    let output = Command::new("xfconf-query")
        .args(["-c", channel, "-p", property])
        .output()
        .ok()?;
    
    if output.status.success() {
        let value = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if !value.is_empty() {
            return Some(value);
        }
    }
    None
}

impl DesktopInfo {
    pub fn display(&self) -> String {
        format!("{} ({})", self.desktop_env, self.display_server)
    }
    
    pub fn display_wm(&self) -> String {
        self.wm.clone().unwrap_or_else(|| self.desktop_env.clone())
    }
    
    pub fn display_theme(&self) -> Option<String> {
        self.theme.clone()
    }
}
