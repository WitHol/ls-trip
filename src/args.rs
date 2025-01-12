/*
This file containst a pure, public main funtion, that parses the arguments and adjusts given
variables, as well as a few other private functions to prevent code duplication
*/

use lazy_static::lazy_static;
use std::collections::HashMap;

macro_rules! exec_if_provided {
    ( $fun:expr, $val:expr, $args:expr, $i:expr, $mess:expr) => {
        $fun($val, $args.get($i+1).expect($mess), &mut $i)
    }
}

macro_rules! exec_if_provided_and_last {
    ($fun:expr, $val:expr, $args:expr, $i:expr, $j:expr, $mess:expr) => {
        if $j == $args[$i].len()-1 {
            exec_if_provided!($fun, $val, $args, $i, $mess);
        } else {
            panic!("flags that requires a value have to be at the end of the combination");
        }
    };
}

// The main function
pub fn parse(args: &Vec<String>, duration: &mut f32, no_cancel: &mut bool, trip_type: &mut i16){
    let mut i: usize = 1;
    while i < args.len() {
        if &args[i][0..2] == "--" { // Long flags starting with --
            match args[i].as_str() {
                "--help" => arg_help(),
                "--list" => arg_list(),
                "--no-cancel" => arg_no_cancel(no_cancel),
                "--duration" => exec_if_provided!(arg_duration, duration, args, i, "duration not provided"),
                "--type" => exec_if_provided!(arg_type, trip_type, args, i, "trip type not provided"),
                "--type-number" => exec_if_provided!(arg_type_number, trip_type, args, i, "trip type number not provided"),
                other_flag => panic!("wrong flag: {}", other_flag)
            };
        }
        else if &args[i][0..1] == "-" { // Short flags starting with -
            for j in 1..args[i].len() {
                let char = &args[i][j..j+1];
                match char {
                    "h" => arg_help(),
                    "l" => arg_list(),
                    "c" => arg_no_cancel(no_cancel),
                    "d" => exec_if_provided_and_last!(arg_duration, duration, args, i, j, "duration not provided"),
                    "t" => exec_if_provided_and_last!(arg_type, trip_type, args, i, j, "trip type not provided"),
                    "T" => exec_if_provided_and_last!(arg_type_number, trip_type, args, i, j, "trip type number not provided"),
                    other_flag => panic!("wrong flag: {}", other_flag)
                }
            }
        }
        else { // Arguments, that aren't flags
            panic!("wrong argument: {}", args[i]);
        }

        i += 1;
    }
}

// This is a hashmap used to assign different trip type names to the corresponding trip type numbers
lazy_static!{
    static ref DRUG_TRIPS: HashMap<String, i16> = HashMap::<String, i16>::from([
        (String::from("circles"), 0),
        (String::from("center"), 1),
    ]);
}

// This is a string, that contains the help message
lazy_static!{
    static ref HELP: String = String::from(
"Simulate a drug trip whenever you mistype ls for lsd
Usage: lsd/ls-trip [flag] [...]

Avilable flags:
    -h, --help              print this message
    -l, --list              list drug trip types
    -t, --type              set the drug trip type by name
    -T, --type-number       set the drug trip type by number
    -d, --duration          set the duration of the drug trip (in seconds)
    -c, --no-cancel         disable the user's ability to stop the program with ctrl-c"
    );
}

// The following functions directly correspond to the flags
pub fn arg_help() {
    println!("{}", *HELP);
    std::process::exit(0);
}

pub fn arg_list() {
    println!("Avilable drug trip types:");
    for (key, value) in DRUG_TRIPS.iter() {
        println!("{} - {}", value, key);
    }
    std::process::exit(0);
}

pub fn arg_no_cancel(no_cancel: &mut bool) {
    *no_cancel = false;
}

pub fn arg_type(trip_type: &mut i16, next_arg: &str, i: &mut usize) {
    *trip_type = *DRUG_TRIPS.get(&next_arg.to_string())
        .expect("no such drug trip");

    *i += 1;
}

pub fn arg_type_number(trip_type: &mut i16, next_arg: &str, i: &mut usize) {
    *trip_type = next_arg.parse::<i16>()
        .expect("not a number")
        -1;

    if *trip_type < 0 || *trip_type > DRUG_TRIPS.len() as i16 {
        panic!("no such drug trip");
    }

    *i += 1;
}

pub fn arg_duration(duration: &mut f32, next_arg: &str, i: &mut usize) {
    *duration = next_arg.parse()
        .expect("wrong duration");

    *i += 1;
}
