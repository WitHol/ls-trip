Overview of the source files:
- main.rs - the main source file, it is meant to be simple and act like a high-level overview
- args.rs - contains a function for parsing arguments moved out of main.rs because of its length
- shared.rs - a file containing different functions that are needed, but are too general to enclose them in specific source files
- trips/ - a module containg the different drug trip files. All of them have one public function, named trip(), that creates a drug trip with of a said type
