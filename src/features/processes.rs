/*

Copyright (C) 2026 compiledkernel-idk https://github.com/compiledkernel-idk

This program is free software: you can redistribute it and/or modify

it under the terms of the GNU General Public License as published by

the Free Software Foundation, either version 3 of the License, or

(at your option) any later version.

This program is distributed in the hope that it will be useful,

but WITHOUT ANY WARRANTY; without even the implied warranty of

MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the

GNU General Public License for more details.

You should have received a copy of the GNU General Public License

along with this program. If not, see https://www.gnu.org/licenses/

*/

use sysinfo::System;
use colored::*;

pub fn show_top_processes() {
    let mut sys = System::new_all();
    sys.refresh_all();
    
    println!("{}", "Top CPU-Consuming Processes".bright_cyan().bold());
    println!("{}", "─".repeat(60).bright_black());
    
    let mut processes: Vec<_> = sys.processes().values().collect();
    processes.sort_by(|a, b| b.cpu_usage().partial_cmp(&a.cpu_usage()).unwrap());
    
    println!("{:>5}  {:30}  {:>8}  {:>10}", 
        "PID".bright_blue().bold(),
        "NAME".bright_blue().bold(),
        "CPU %".bright_blue().bold(),
        "MEMORY".bright_blue().bold()
    );
    
    for process in processes.iter().take(5) {
        let cpu = process.cpu_usage();
        if cpu < 0.1 {
            continue;
        }
        
        let mem_mb = process.memory() / 1024 / 1024;
        let name = process.name().to_string_lossy();
        let name_truncated = if name.len() > 30 {
            format!("{}...", &name[..27])
        } else {
            name.to_string()
        };
        
        println!("{:>5}  {:30}  {:>7.1}%  {:>8} MB",
            process.pid(),
            name_truncated,
            cpu,
            mem_mb
        );
    }
}
