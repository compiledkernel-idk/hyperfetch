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

pub mod os;
pub mod kernel;
pub mod cpu;
pub mod gpu;
pub mod memory;
pub mod disk;
pub mod uptime;
pub mod shell;
pub mod desktop;
pub mod display;
pub mod battery;
pub mod network;
pub mod packages;
pub mod user;

use sysinfo::System;

#[derive(Debug, Clone)]
pub struct SystemInfo {
    pub user: user::UserInfo,
    pub os: os::OsInfo,
    pub kernel: kernel::KernelInfo,
    pub cpu: cpu::CpuInfo,
    pub gpu: gpu::GpuInfo,
    pub memory: memory::MemoryInfo,
    pub disk: disk::DiskInfo,
    pub uptime: uptime::UptimeInfo,
    pub shell: shell::ShellInfo,
    pub desktop: desktop::DesktopInfo,
    pub display: display::DisplayInfo,
    pub battery: battery::BatteryInfo,
    pub network: network::NetworkInfo,
    pub packages: packages::PackageInfo,
}

pub fn collect_all_info() -> SystemInfo {
    let mut sys = System::new_all();
    sys.refresh_all();
    
    SystemInfo {
        user: user::get_info(),
        os: os::get_info(),
        kernel: kernel::get_info(),
        cpu: cpu::get_info(&sys),
        gpu: gpu::get_info(),
        memory: memory::get_info(&sys),
        disk: disk::get_info(&sys),
        uptime: uptime::get_info(),
        shell: shell::get_info(),
        desktop: desktop::get_info(),
        display: display::get_info(),
        battery: battery::get_info(),
        network: network::get_info(),
        packages: packages::get_info(),
    }
}
