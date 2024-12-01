// This is the main source file
// First, we collect the flags and iterate upon them, then we set up ncurses, 
// execute the drug trip, which is contained inside of another file, and end the program

use crate::shared::*;
use crate::circles::*;

mod circles;
mod shared;

fn main()
{
    // Preparation (these will be used later)
    let args: Vec<String> = std::env::args().collect(); // Collecting the arguments
    let mut duration: f32 = 10.0; // The duration of the drug trip
    let mut no_stop: bool = false; // Whether or not to allow stopping the program with ctrl+c

    // Checking the flags
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

            // --type and --type-number are not really implemented yet, since there is only one drug trip type
            "--type" | "-t" => {}

            "--type-number" | "-T" => {}

            _ => {
                println!("wrong flag: {}", args[i]);
                std::process::exit(1);
            }
        };

        i += 1;
    }

    // Setting things up
    ncurses::initscr();
    ncurses::curs_set(ncurses::CURSOR_VISIBILITY::CURSOR_INVISIBLE);
    if no_stop { ncurses::raw(); } else {ncurses::cbreak();} // If --no-cancel -c flag was used change input mode to raw
    ncurses::noecho();
    ncurses::start_color();
    colors_setup(); // From shared.rs

    circles(duration); // From circles.rs

    // Closing the window
    ncurses::endwin();
    std::process::exit(0);
}

