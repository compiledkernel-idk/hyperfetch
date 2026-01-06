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

use colored::*;
use std::time::Instant;

pub fn show_benchmark() {
    println!("{}", "Performance Benchmark".bright_cyan().bold());
    println!("{}", "─".repeat(40).bright_black());
    
    let cpu_score = benchmark_cpu();
    let cpu_rating = get_rating(cpu_score, 1000.0);
    
    println!("CPU Score: {} - {}", 
        format!("{:.0}", cpu_score).bright_yellow(),
        cpu_rating
    );
    
    let mem_score = benchmark_memory();
    let mem_rating = get_rating(mem_score, 5000.0);
    
    println!("Memory Score: {} - {}",
        format!("{:.0}", mem_score).bright_yellow(),
        mem_rating
    );
    
    let overall = (cpu_score + mem_score) / 2.0;
    let overall_rating = get_rating(overall, 3000.0);
    
    println!();
    println!("Overall Performance: {}",
        overall_rating.bright_green().bold()
    );
}

fn benchmark_cpu() -> f64 {
    
    let start = Instant::now();
    let mut iterations = 0;
    
    while start.elapsed().as_millis() < 100 {
        fibonacci(20);
        iterations += 1;
    }
    
    iterations as f64 * 10.0
}

fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

fn benchmark_memory() -> f64 {
    
    let start = Instant::now();
    let mut iterations = 0;
    
    while start.elapsed().as_millis() < 100 {
        let v: Vec<u64> = (0..1000).collect();
        let _sum: u64 = v.iter().sum();
        iterations += 1;
    }
    
    iterations as f64 * 50.0
}

fn get_rating(score: f64, reference: f64) -> String {
    let ratio = score / reference;
    
    if ratio >= 2.0 {
        "★★★★★ Excellent".to_string()
    } else if ratio >= 1.5 {
        "★★★★☆ Very Good".to_string()
    } else if ratio >= 1.0 {
        "★★★☆☆ Good".to_string()
    } else if ratio >= 0.7 {
        "★★☆☆☆ Fair".to_string()
    } else {
        "★☆☆☆☆ Below Average".to_string()
    }
}
