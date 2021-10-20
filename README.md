# bklt

It as a small utility with purpose of changing screen brightness on laptops running Linux with simpler WMs.

## Usage
Program usage is pretty self explanatory from program help.

```
SAGE:
    bklt [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -B <bri-file>        location of a file containing current brightness value [default:
                         /sys/class/backlight/intel_backlight/brightness]
    -d <decrease>        amount of percent to decrease brightness for
    -i <increase>        amount of percent to increase brightness for
    -M <max-file>        location of a file containing max brightness value [default:
                         /sys/class/backlight/intel_backlight/max_brightness]
    -s <set>             value in percent to set brightness to
```

## Install
Use `crate` to build from source:
```
crate build --release
```
Or install via crate:
```
crate install --git https://github.com/alicerum/bklt
```

