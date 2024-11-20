**ls-trip**
-
Ls-trip is a shell program, that simulates a drug trip every time you mistype "ls" for "lsd" in the terminal. It can also be activated with the "ls-trip" command.

**Warning**
-
This program's binary is called lsd, which means it's mutually exclusive with the LSD (LSDeluxe) utility: https://github.com/lsd-rs/lsd.

**Installation**
-
Because this project is in developement, there are no pre-made packages, so you'll need to build it on your own.
To install it ensure you have cargo installed and avilable as root and use make.

To install ls-trip:
```
git clone https://github.com/WitHol/ls-trip
cd ls-trip
make
make install
```
To uninstall:
```
make uninstall
```
To update:
```
make update
```
