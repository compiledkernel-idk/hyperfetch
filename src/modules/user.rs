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

#[derive(Debug, Clone)]
pub struct UserInfo {
    pub username: String,
    pub hostname: String,
    pub home_dir: String,
}

pub fn get_info() -> UserInfo {
    let username = whoami::username();
    let hostname = hostname::get()
        .map(|h| h.to_string_lossy().to_string())
        .unwrap_or_else(|_| "unknown".to_string());
    let home_dir = env::var("HOME").unwrap_or_else(|_| "~".to_string());
    
    UserInfo {
        username,
        hostname,
        home_dir,
    }
}

impl UserInfo {
    pub fn display(&self) -> String {
        format!("{}@{}", self.username, self.hostname)
    }
}
