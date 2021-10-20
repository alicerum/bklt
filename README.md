# bklt

It as a small utility with purpose of changing screen brightness on laptops running Linux with simpler WMs.

## Usage
Program usage is pretty self explanatory from program help.

```
bklt 0.1.0

USAGE:
    This program tries to deduce correct way of setting backlight in an X environment.
    In case it fails to do so, user can set 'M' and 'B' flags manually.
    Only one of 's', 'i' or 'd' flags is allowed

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -B <bri-file>        optional: location of a file containing current brightness value
    -d <decrease>        amount of percent to decrease brightness for
    -i <increase>        amount of percent to increase brightness for
    -M <max-file>        optional: location of a file containing max brightness value
    -s <set>             value in percent to set brightness to
```

## Install
Use `crate` to build from source:
```
cargo build --release
```
Or install via crate:
```
cargo install --git https://github.com/alicerum/bklt
```

