pub fn HELP() -> String
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
")
}