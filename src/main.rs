/*
This is the main source file. It doesn't contain much code, so that you can get a high-level overview of 
what is happening just by looking at it.
*/

use rand::Rng;

mod shared;
mod trips;
mod args;

fn main()
{
    // Preparation, these variables will be set by arguments in a moment, the values assigned to
    // them are the defaults
    let mut duration: f32 = 10.0; // The duration of the drug trip
    let mut no_cancel: bool = false; // Whether or not to allow stopping the program with ctrl+c
    let mut trip_type: i16 = rand::thread_rng().gen_range(1..4); // The drug trip type (number)

    // Collecting and processing the arguments by modifying the previous variables, writing the
    // help message on -h flag or quitting on the wrong flag
    let args: Vec<String> = std::env::args().collect(); // Collecting the arguments
    args::parse(&args, &mut duration, &mut no_cancel, &mut trip_type);

    // Setting up ncurses stuff
    ncurses::initscr();
    ncurses::curs_set(ncurses::CURSOR_VISIBILITY::CURSOR_INVISIBLE);
    ncurses::noecho();
    ncurses::start_color();

    shared::colors_setup(); // Defining a bunch of color groups as well as a few colors, so they are avilable later on
    
    // If --no-cancel -c flag was used, change input mode to raw 
    match no_cancel {
        true => ncurses::raw(),
        false => ncurses::cbreak(),
    };

    match trip_type {
        1 => trips::circles::trip(duration),
        2 => trips::center::trip(duration),
        3 => trips::ellipse::trip(duration),
        _ => {ncurses::endwin(); panic!("No such drug trip!")}
    }

    // Closing the window
    ncurses::endwin();
}
