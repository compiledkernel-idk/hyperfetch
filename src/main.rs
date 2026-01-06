/**

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

mod cli;
mod modules;
mod logos;
mod output;
mod features;

use cli::Args;
use clap::Parser;

fn main() {
    let args = Args::parse();
    
    let info = modules::collect_all_info();
    
    let logo = if args.no_logo {
        Vec::new()
    } else {
        let logo_name = args.logo.as_deref().unwrap_or(&info.os.distro_id);
        logos::get_logo(logo_name)
    };
    
    let theme = output::colors::get_theme(&args.color.as_deref().unwrap_or("default"));
    
    output::render(&logo, &info, &theme, &args);
    
    if args.benchmark {
        println!();
        features::benchmark::show_benchmark();
    }
    
    if args.processes {
        println!();
        features::processes::show_top_processes();
    }
    
    if args.colors {
        println!();
        features::colors_preview::show_palette();
    }
}
