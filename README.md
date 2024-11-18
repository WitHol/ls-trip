**This is a release branch, which means it is sometimes behind the master, but is much less likely to contain errors**
-
**ls-trip**
-
Ls-trip is a shell program, that simulates a drug trip every time you mistype "ls" for "lsd" in the terminal. It can also be activated with the "ls-trip" command.

**Warning**
-
This program's binary is called lsd, which means it's mutually exclusive with the LSD (LSDeluxe) utility: https://github.com/lsd-rs/lsd.

**Installation**
-
Because this project is in developement, there are no pre-made packages, so you'll need to build it on your own.
To install it ensure you have cargo installed and avilable as root, and then use these commands:
```
git clone https://github.com/WitHol/ls-trip/tree/release
git checkout
cargo build --release
cp target/release/ls-trip /usr/local/bin/ls-trip
ln -sf /usr/local/bin/lsd /usr/local/bin/ls-trip
```
In order to uninstall it:
```
rm -f /usr/local/bin/lsd /usr/local/bin/ls-trip
```
To update:
```
(at the top of the repo)
git pull origin release
cargo build --release
rm -f /usr/local/bin/lsd /usr/local/bin/ls-trip
cp target/release/ls-trip /usr/local/bin/ls-trip
ln -sf /usr/local/bin/lsd /usr/local/bin/ls-trip
```
