/*
This is the main source file. It doesn't contain much code, so that you can get a high-level overview of 
what is happening just by looking at it.
*/

use crate::shared::*;
use crate::trips::circles;
use crate::trips::center;

use rand::Rng;
use std::f32::consts::PI;
use std::collections::HashMap;

mod shared;
mod trips;

fn main()
{
    // Preparation (these will be used later)
    let args: Vec<String> = std::env::args().collect(); // Collecting the arguments
    let mut duration: f32 = 10.0; // The duration of the drug trip
    let mut no_stop: bool = false; // Whether or not to allow stopping the program with ctrl+c
    let mut trip_type: i8 = rand::thread_rng().gen_range(0..2);

    // Processing the arguments, and adjusting previously defined variables according to the flags
    process_args(&args, &mut duration, &mut no_stop, &mut trip_type);

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


// This function processes the arguments and adjusts the varables, according to them.
// It can also stop the program on a wrong argument, or when a flag like -h has been provided 
fn process_args(args: &Vec<String>, duration: &mut f32, no_stop: &mut bool, trip_type: &mut i8)
{
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
                *no_stop = true; 
            }

            "--duration" | "-d" => {
                if i == args.len()-1 
                { 
                    println!("no duration provided");
                    std::process::exit(1);
                }
                *duration = match args[i+1].parse()
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

                *trip_type = match match args[i+1].parse::<String>()
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

                *trip_type = match args[i+1].parse()
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
}


// The only purpose of this function is to hold the help message, 
// cuz I didn't feel like learning lifetime specifiers for static variables
fn HELP() -> String
{
    String::from("
    Simulate a drug trip whenever you mistype ls for lsd
    Usage: lsd/ls-trip <flag> [...]

    Avilable flags:
        -h, --help              print this menu
        -l, --list              list drug trip types
        -t, --type              drug trip type name
        -T, --type-number       drug trip type number
        -d, --duration          duration of the drug trip in seconds
        -c, --no-cancel         disable the user's ability to stop the program with ctrl-c
    ")
}