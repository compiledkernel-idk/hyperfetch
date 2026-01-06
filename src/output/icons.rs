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

pub const ICON_OS: &str = "󰣇";
pub const ICON_KERNEL: &str = "";
pub const ICON_CPU: &str = "󰌽";
pub const ICON_GPU: &str = "";
pub const ICON_MEMORY: &str = "";
pub const ICON_DISK: &str = "󰋊";
pub const ICON_UPTIME: &str = "";
pub const ICON_SHELL: &str = "󰆍";
pub const ICON_DESKTOP: &str = "";
pub const ICON_DISPLAY: &str = "󰍹";
pub const ICON_BATTERY: &str = "󰂄";
pub const ICON_BATTERY_CHARGING: &str = "󰂄";
pub const ICON_BATTERY_FULL: &str = "󰁹";
pub const ICON_BATTERY_LOW: &str = "󰂃";
pub const ICON_PACKAGES: &str = "";
pub const ICON_TERMINAL: &str = "";
pub const ICON_NETWORK: &str = "";
pub const ICON_THEME: &str = "";

pub fn get_battery_icon(percentage: f32, charging: bool) -> &'static str {
    if charging {
        ICON_BATTERY_CHARGING
    } else if percentage > 80.0 {
        ICON_BATTERY_FULL
    } else if percentage < 20.0 {
        ICON_BATTERY_LOW
    } else {
        ICON_BATTERY
    }
}
