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

#[derive(Debug, Clone)]
pub struct BatteryInfo {
    pub batteries: Vec<Battery>,
    pub has_battery: bool,
}

#[derive(Debug, Clone)]
pub struct Battery {
    pub name: String,
    pub percentage: f32,
    pub state: BatteryState,
    pub health: Option<f32>,
    pub time_remaining: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BatteryState {
    Charging,
    Discharging,
    Full,
    NotCharging,
    Unknown,
}

pub fn get_info() -> BatteryInfo {
    let mut batteries = Vec::new();
    
    if let Ok(manager) = battery::Manager::new() {
        if let Ok(bats) = manager.batteries() {
            for (idx, bat_result) in bats.enumerate() {
                if let Ok(bat) = bat_result {
                    let percentage = bat.state_of_charge().get::<battery::units::ratio::percent>();
                    
                    let state = match bat.state() {
                        battery::State::Charging => BatteryState::Charging,
                        battery::State::Discharging => BatteryState::Discharging,
                        battery::State::Full => BatteryState::Full,
                        battery::State::Empty => BatteryState::Discharging,
                        _ => BatteryState::Unknown,
                    };
                    
                    let health = Some(bat.state_of_health().get::<battery::units::ratio::percent>());
                    
                    let time_remaining = get_time_remaining(&bat);
                    
                    batteries.push(Battery {
                        name: format!("BAT{}", idx),
                        percentage,
                        state,
                        health,
                        time_remaining,
                    });
                }
            }
        }
    }
    
    let has_battery = !batteries.is_empty();
    
    BatteryInfo {
        batteries,
        has_battery,
    }
}

fn get_time_remaining(bat: &battery::Battery) -> Option<String> {
    let time = if bat.state() == battery::State::Charging {
        bat.time_to_full()
    } else {
        bat.time_to_empty()
    };
    
    time.map(|t| {
        let secs = t.get::<battery::units::time::second>() as u64;
        let hours = secs / 3600;
        let mins = (secs % 3600) / 60;
        
        if hours > 0 {
            format!("{}h {}m", hours, mins)
        } else {
            format!("{}m", mins)
        }
    })
}

impl BatteryInfo {
    pub fn display(&self) -> Option<String> {
        self.batteries.first().map(|b| b.display())
    }
}

impl Battery {
    pub fn display(&self) -> String {
        let state_str = match self.state {
            BatteryState::Charging => "⚡ charging",
            BatteryState::Discharging => "🔋 discharging",
            BatteryState::Full => "✓ full",
            BatteryState::NotCharging => "not charging",
            BatteryState::Unknown => "",
        };
        
        let mut result = format!("{:.0}%", self.percentage);
        
        if !state_str.is_empty() {
            result.push_str(&format!(" ({})", state_str));
        }
        
        if let Some(ref time) = self.time_remaining {
            result.push_str(&format!(" - {}", time));
        }
        
        result
    }
    
    pub fn get_bar(&self, width: usize) -> String {
        let filled = (self.percentage / 100.0 * width as f32) as usize;
        let empty = width.saturating_sub(filled);
        
        format!("[{}{}]", "█".repeat(filled), "░".repeat(empty))
    }
}

impl std::fmt::Display for BatteryState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BatteryState::Charging => write!(f, "Charging"),
            BatteryState::Discharging => write!(f, "Discharging"),
            BatteryState::Full => write!(f, "Full"),
            BatteryState::NotCharging => write!(f, "Not Charging"),
            BatteryState::Unknown => write!(f, "Unknown"),
        }
    }
}
