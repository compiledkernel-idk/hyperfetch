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

use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "hyperfetch")]
#[command(author = "compiledkernel-idk")]
#[command(version = "1.0.0")]
#[command(about = "HyperFetch", long_about = None)]
pub struct Args {
    
    #[arg(short, long)]
    pub logo: Option<String>,
    
    #[arg(short, long)]
    pub color: Option<String>,
    
    #[arg(short, long)]
    pub small: bool,
    
    #[arg(short, long)]
    pub all: bool,
    
    #[arg(long)]
    pub no_logo: bool,
    
    #[arg(long)]
    pub benchmark: bool,
    
    #[arg(long)]
    pub processes: bool,
    
    #[arg(long)]
    pub colors: bool,
    
    #[arg(long)]
    pub json: bool,
    
    #[arg(long)]
    pub no_icons: bool,
}
