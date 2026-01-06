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

use colored::*;

pub fn show_palette() {
    println!("{}", "Terminal Color Palette".bright_cyan().bold());
    println!("{}", "─".repeat(40).bright_black());
    println!();
    
    print!("{}", "███".black());
    print!("{}", "███".red());
    print!("{}", "███".green());
    print!("{}", "███".yellow());
    print!("{}", "███".blue());
    print!("{}", "███".magenta());
    print!("{}", "███".cyan());
    println!("{}", "███".white());
    
    print!("{}", "███".bright_black());
    print!("{}", "███".bright_red());
    print!("{}", "███".bright_green());
    print!("{}", "███".bright_yellow());
    print!("{}", "███".bright_blue());
    print!("{}", "███".bright_magenta());
    print!("{}", "███".bright_cyan());
    println!("{}", "███".bright_white());
}
