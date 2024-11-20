**ls-trip**
-
Ls-trip is a shell program, that simulates a drug trip every time you mistype "ls" for "lsd" in the terminal. It can also be activated with the "ls-trip" command.

**Warning**
-
This program creates a symlink at /usr/local/bin/lsd to its binary, which means if you have lsd (https://github.com/lsd-rs/lsd) installed, they will interfere with each other. In that case it is recommended, that you remove the symlink created by ls-trip, or you rename the lsd binary to something else.

**Installation**
-
Because this project is in developement, there are no pre-made packages, so you'll need to build it on your own.
In order to install it, ensure you have make cargo installed and avilable as root.

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
