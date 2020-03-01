extern crate rand;

use std::io;
use rand::prelude::*;
use wasm_gameoflife::Universe;

fn main() {
    let stdin = io::stdin();
    let mut uni: Universe = Universe::default_with_size(10);
    let mut run_forever = false;
    let mut cont_for = 0;
    let mut rng = rand::thread_rng();
    loop {
        println!("{}", uni);
        if cont_for > 0 {
            cont_for -= 1;
            uni.tick();
            continue;
        }
        if run_forever {
            uni.tick();
            continue;
        }
        loop {
            println!("[n]ew <width> x <height> universe / [d]efault universe with size <s> / [s]tep once / [c]ontinue <n> times / [r]un forever");
            let mut line = String::new();
            stdin.read_line(&mut line).expect("Failed to read input");
            let parts: Vec<&str> = line.trim().split(' ').collect::<Vec<&str>>();
            let cmd = parts[0];
            let args = parts
                .iter()
                .skip(1)
                .map(|arg| arg.parse::<u32>().ok())
                .collect::<Vec<Option<u32>>>();
            let cmd_read_ok = match cmd {
                "n" | "new" => {
                    match (args.get(0).map(|x| x.as_ref()).flatten(), args.get(1).map(|x| x.as_ref()).flatten()) {
                        (Some(&width), Some(&height)) => {
                            println!("New {} x {} random universe", width, height);
                            uni = Universe::new(width, height);
                            for x in 0..width {
                                for y in 0..height {
                                    if rng.gen_bool(0.7) {
                                        uni.toggle(x, y);
                                    }
                                }
                            }
                            true
                        }
                        _ => {
                            println!("Could not parse width and height arguments: {}", parts[1..].join(", "));
                            false
                        },
                    }
                },
                "d" | "default" => {
                    if let Some(&Some(arg)) = args.get(0) {
                        println!("New default universe with side {}", arg);
                        uni = Universe::default_with_size(arg);
                        true
                    } else {
                        println!("Please provide an integer argument!");
                        false
                    }
                },
                "s" | "step" => {
                    uni.tick();
                    true
                },
                "c" | "cont" | "continue" => {
                    if let Some(&Some(arg)) = args.get(0) {
                        cont_for = arg;
                        true
                    } else {
                        println!("Please provide an integer argument!");
                        false
                    }
                },
                "r" | "run" => {
                    run_forever = true;
                    true
                },
                _ => {
                    println!("Please pick one of the given options!");
                    false
                },
            };
            if cmd_read_ok {
                break;
            }
        }
    }
}
