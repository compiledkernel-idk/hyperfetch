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
use std::fs;

#[derive(Debug, Clone)]
pub struct ShellInfo {
    pub name: String,
    pub version: Option<String>,
    pub path: String,
    pub terminal: String,
}

pub fn get_info() -> ShellInfo {
    let shell_path = env::var("SHELL").unwrap_or_else(|_| "/bin/sh".to_string());
    let shell_name = shell_path.split('/').last()
        .unwrap_or("sh")
        .to_string();
    
    let version = get_shell_version(&shell_name);
    let terminal = detect_terminal();
    
    ShellInfo {
        name: shell_name,
        version,
        path: shell_path,
        terminal,
    }
}

fn get_shell_version(shell: &str) -> Option<String> {
    let output = match shell {
        "bash" => Command::new("bash").arg("--version").output().ok()?,
        "zsh" => Command::new("zsh").arg("--version").output().ok()?,
        "fish" => Command::new("fish").arg("--version").output().ok()?,
        "nu" | "nushell" => Command::new("nu").arg("--version").output().ok()?,
        "pwsh" | "powershell" => Command::new("pwsh").arg("--version").output().ok()?,
        _ => return None,
    };
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    let first_line = stdout.lines().next()?;
    
    let version = match shell {
        "bash" => {
            
            first_line.split("version ").nth(1)?
                .split(|c: char| !c.is_numeric() && c != '.').next()?
                .to_string()
        }
        "zsh" => {
            
            first_line.split_whitespace().nth(1)?.to_string()
        }
        "fish" => {
            
            first_line.split("version ").nth(1)?.to_string()
        }
        "nu" | "nushell" => {
            first_line.trim().to_string()
        }
        _ => first_line.to_string(),
    };
    
    Some(version)
}

fn detect_terminal() -> String {
    
    if let Ok(term_program) = env::var("TERM_PROGRAM") {
        return term_program;
    }
    
    let possible_terms = [
        ("KITTY_WINDOW_ID", "kitty"),
        ("ALACRITTY_LOG", "Alacritty"),
        ("WEZTERM_PANE", "WezTerm"),
        ("GNOME_TERMINAL_SCREEN", "GNOME Terminal"),
        ("KONSOLE_VERSION", "Konsole"),
        ("TERMINATOR_UUID", "Terminator"),
        ("TILIX_ID", "Tilix"),
        ("ITERM_SESSION_ID", "iTerm2"),
    ];
    
    for (env_var, term_name) in &possible_terms {
        if env::var(env_var).is_ok() {
            return term_name.to_string();
        }
    }
    
    if let Some(ppid) = get_parent_pid() {
        if let Some(name) = get_process_name(ppid) {
            let known_terms = ["kitty", "alacritty", "konsole", "gnome-terminal", 
                              "xterm", "urxvt", "terminator", "tilix", "wezterm",
                              "st", "foot", "contour", "hyper"];
            for term in &known_terms {
                if name.to_lowercase().contains(term) {
                    return name;
                }
            }
        }
    }
    
    env::var("TERM").unwrap_or_else(|_| "unknown".to_string())
}

fn get_parent_pid() -> Option<u32> {
    let status = fs::read_to_string("/proc/self/status").ok()?;
    for line in status.lines() {
        if line.starts_with("PPid:") {
            return line.split_whitespace().nth(1)?.parse().ok();
        }
    }
    None
}

fn get_process_name(pid: u32) -> Option<String> {
    let comm = fs::read_to_string(format!("/proc/{}/comm", pid)).ok()?;
    Some(comm.trim().to_string())
}

impl ShellInfo {
    pub fn display(&self) -> String {
        match &self.version {
            Some(v) => format!("{} {}", self.name, v),
            None => self.name.clone(),
        }
    }
    
    pub fn display_terminal(&self) -> String {
        self.terminal.clone()
    }
}
