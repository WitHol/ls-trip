<div align=center>
  <img src=../assets/logo.png width=50% height=auto>
</div>

Ls-trip is a joke shell utility, that simulates a drug trip every time you mistype "ls" for "lsd". It can also be activated with the "ls-trip" command.

# Installation
This program is currently only avilable on Linux.
Since ls-trip is in developement, there are no pre-made packages, so the only way to install it, is to build it from source.

### Warning
Ls-trip creates a symlink at /usr/bin/lsd, which means it will interfere with lsd (https://github.com/lsd-rs/lsd).

### Building from source
Requrements:
- make
- cargo (make sure it is usable as root)

To install ls-trip:
```
git clone https://github.com/WitHol/ls-trip -b stable
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
git pull
make
make install
```
