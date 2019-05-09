//     Copyright 2019 Haoran Wang
//
//     Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
//     You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
//     distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//     See the License for the specific language governing permissions and
//     limitations under the License.

use alpha_sim::define::*;
use alpha_sim::sim_chip::SimChip;

fn print_info() {
    println!("{}(v {}): {}", PROJ, VER, NAME);
    println!("Released under {}", LICENSE);
    println!("Copyright 2019 {}", AUTHORS);
}

fn print_usage() {
    print_info();
    println!();
    println!("Usage:");
    println!("        cargo run -- [obj_name]");
    println!();
    // print the actual usage
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() == 2 {
        let prog_name = args.get(1).unwrap();
        print_info();
        println!("Hello, alpha_sim!");
        println!("Executable file: {}", prog_name);

        // create a chip and loop to execute
        let mut sim_chip = SimChip::new(prog_name, &args);
        sim_chip.r#loop();

        // print some information about the simulation
        println!("");
        for _ in 0..64 {
            print!("-");
        }
        println!("");
        println!("Simulation Stats for file: {}", prog_name);
    } else {
        print_usage();
    }
}
