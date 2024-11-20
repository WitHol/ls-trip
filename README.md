**ls-trip**
-
Ls-trip is a shell program, that simulates a drug trip every time you mistype "ls" for "lsd" in the terminal. It can also be activated with the "ls-trip" command.

**Warning**
-
Ls-trip creates a symlink at /usr/bin/lsd (/usr/local/bin/lsd if you build from source), which means it will interfere with lsd (https://github.com/lsd-rs/lsd).

**Installation**
-
At the moment this utility is only avilable for Linux.
Because this project is in developement, there are no pre-made packages, so you'll need to build it on your own.
In order to install it, you need cargo and make installed and avilable as root.

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
make install
```
