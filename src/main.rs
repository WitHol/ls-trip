/*
This is the main source file. It doesn't contain much code, so that you can get a high-level overview of 
what is happening just by looking at it.
*/

use rand::Rng;

use crate::shared::*;
use crate::trips::circles;
use crate::trips::center;
use std::f32::consts::PI;

mod shared;
mod trips;

fn main()
{
    // Preparation (these will be used later)
    let args: Vec<String> = std::env::args().collect(); // Collecting the arguments
    let mut duration: f32 = 10.0; // The duration of the drug trip
    let mut no_stop: bool = false; // Whether or not to allow stopping the program with ctrl+c
    let mut trip_type: i8 = rand::thread_rng().gen_range(0..2);

    // Processing the flags
    // In this ridiculously long loop, we iterate over every argument and either adjust previously defined variables,
    // or print something to the terminal and stopping the program
    let mut i: usize = 1;
    while i < args.len()
    {
        match args[i].as_str()
        {
            "--help" | "-h" => { 
                print!("{}", HELP()); std::process::exit(0); 
            }

            "--list" | "-l" => {
                println!("Avilable drug trip types: \n  1. circles"); // Placeholder
                std::process::exit(0); 
            }

            "--no-cancel" | "-c" => { 
                no_stop = true; 
            }

            "--duration" | "-d" => {
                if i == args.len()-1 
                { 
                    println!("no duration provided");
                    std::process::exit(1);
                }
                duration = match args[i+1].parse()
                { 
                    Ok(T) => {T} 
                    Err(_) => {
                        println!("wrong duration: {}", args[i+1]);
                        std::process::exit(1);
                    }
                };
                i += 1;
            }

            "--type" | "-t" => {
                if i == args.len()-1 
                { 
                    println!("no drug trip type provided");
                    std::process::exit(1);
                }

                trip_type = match match args[i+1].parse::<String>()
                { 
                    Ok(T) => T,
                    Err(_) => {
                        println!("wrong drug trip type: {}", args[i+1]);
                        std::process::exit(1);
                    }
                }.as_str()
                {
                    "circles" => 0,
                    "center" => 1,
                    _ => {
                        println!("wrong drug trip type: {}", args[i+1]);
                        std::process::exit(1);
                    }
                };
                i += 1;
            }

            "--type-number" | "-T" => {
                if i == args.len()-1 
                { 
                    println!("no drug trip type number provided");
                    std::process::exit(1);
                }

                trip_type = match args[i+1].parse()
                {
                    Ok(T) => T,
                    Err(_) => {
                        println!("wrong drig trip type: {}", args[i+1]);
                        std::process::exit(1);
                    }
                };

                i += 1;
            }

            _ => {
                println!("wrong flag: {}", args[i]);
                std::process::exit(1);
            }
        };
        i += 1;
    }

    // Setting up ncurses stuff
    ncurses::initscr();
    ncurses::curs_set(ncurses::CURSOR_VISIBILITY::CURSOR_INVISIBLE);
    ncurses::noecho();
    ncurses::start_color();
    if no_stop { ncurses::raw(); } else {ncurses::cbreak();} // If --no-cancel -c flag was used, change input mode to raw 
    colors_setup(); // Defining a bunch of color groups as well as a few colors, so they are avilable later on

    // The main body of this program
    if trip_type == 0
    {
        circles::main(duration);
    }
    else 
    {
        center::main(duration);
    }

    // Closing the window
    ncurses::endwin();
}