/*
This file containst a pure, public main funtion, that parses the arguments and adjusts given
variables, as well as a few other private functions to prevent code duplication
*/

use lazy_static::lazy_static;
use std::collections::HashMap;

pub fn parse(args: &Vec<String>, duration: &mut f32, no_cancel: &mut bool, trip_type: &mut i16)
{
    let mut i: usize = 1;

    while i < args.len()
    {
        if &args[i][0..2] == "--" // Long flags starting with --
        {
            match args[i].as_str()
            {
                "--help" => arg_help(),
                "--list" => arg_list(),
                "--no-cancel" => arg_no_cancel(no_cancel),
                "--duration" => arg_duration(duration, args.get(i+1)
                    .expect("duration not provided"), 
                    &mut i),
                "--type" => arg_type(trip_type, args.get(i+1)
                    .expect("trip type not provided"), 
                    &mut i),
                "--type-number" => arg_type_number(trip_type, args.get(i+1)
                    .expect("trip number not provided"), 
                    &mut i),
                    
                other_flag => {
                    panic!("wrong flag: {}", other_flag);
                }
            };
        }
        else if &args[i][0..1] == "-" // Short flags starting with -
        {
            for j in 1..args[i].len()
            {
                let char = &args[i][j..j+1];
                match char
                {
                    "h" => arg_help(),
                    "l" => arg_list(),
                    "c" => arg_no_cancel(no_cancel),

                    "d" => if j == args[i].len()-1 {
                        arg_duration(duration, args.get(i+1)
                            .expect("duration not provided"), 
                            &mut i)
                    } else {
                        panic!("flags that requires a value have to be at the end of the combination")
                    }

                    "t" => if j == args[i].len()-1 {
                        arg_type(trip_type, args.get(i+1)
                            .expect("trip type not provided"), 
                            &mut i)
                    } else {
                        panic!("flags that requires a value have to be at the end of the combination")
                    }

                    "T" => if j == args[i].len()-1 {
                        arg_type_number(trip_type, args.get(i+1)
                            .expect("trip number not provided"), 
                            &mut i)
                    } else {
                        panic!("flags that requires a value have to be at the end of the combination")
                    }

                    other_flag => {
                        panic!("wrong flag: {}", other_flag);
                    }
                }
            }
        }
        else // Arguments, that aren't flags
        {
            panic!("wrong argument: {}", args[i]);
        }

        i += 1;
    }
}

    
// This is a hashmap, used to assign different trip type names with the trip numbers
lazy_static!{
    static ref DRUG_TRIPS: HashMap<String, i16> = HashMap::<String, i16>::from([
        (String::from("circles"), 0),
        (String::from("center"), 1),
    ]);
}

// This is a string, that contains the help message
lazy_static!{
    static ref HELP: String = String::from("Simulate a drug trip whenever you mistype ls for lsd
Usage: lsd/ls-trip [flag] [...]

Avilable flags:
    -h, --help              print this menu
    -l, --list              list drug trip types
    -t, --type              drug trip type name
    -T, --type-number       drug trip type number
    -d, --duration          duration of the drug trip in seconds
    -c, --no-cancel         disable the user's ability to stop the program with ctrl-c");
}

// The following functions directly correspond to the flags (exept for type_name)
pub fn arg_help()
{
    println!("{}", *HELP);
    std::process::exit(0);
}

pub fn arg_list()
{
    println!("Avilable drug trip types:");
    for (key, value) in DRUG_TRIPS.iter()
    {
        println!("{} - {}", value, key);
    }
    std::process::exit(0);
}

pub fn arg_no_cancel(no_cancel: &mut bool)
{
    *no_cancel = false;
}

pub fn arg_type(trip_type: &mut i16, next_arg: &str, i: &mut usize)
{
    *trip_type = *DRUG_TRIPS.get(&next_arg.to_string())
        .expect("no such drug trip");

    *i += 1;
}

pub fn arg_type_number(trip_type: &mut i16, next_arg: &str, i: &mut usize)
{
    *trip_type = next_arg.parse::<i16>()
        .expect("not a number")
        -1;
    
    if *trip_type < 0 || *trip_type > DRUG_TRIPS.len() as i16
    {
        panic!("no such drug trip");
    }

    *i += 1;
}

pub fn arg_duration(duration: &mut f32, next_arg: &str, i: &mut usize)
{
    *duration = next_arg.parse()
        .expect("wrong duration");

    *i += 1;
}
