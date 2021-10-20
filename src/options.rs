use std::fmt;
use std::fmt::Display;
use std::num::ParseIntError;
use std::error::Error;
use clap::{crate_version, App, Arg};

#[derive(Debug)]
pub struct OptionError {
    err_msg: String,
}

impl OptionError {
    fn from(s: &str) -> OptionError {
        OptionError {
            err_msg: String::from(s),
        }
    }
}

impl From<ParseIntError> for OptionError {
    fn from(_: ParseIntError) -> Self {
        OptionError::from(
            &format!("error parsing numeric argument")
        )
    }
}

impl Display for OptionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.err_msg)
    }
}

impl Error for OptionError {}

#[derive(Debug)]
pub struct Options {
    pub max_file: String,
    pub bri_file: String,

    pub inc: Option<u8>,
    pub dec: Option<u8>,
    pub set: Option<u8>,
}

pub fn get_options() -> Result<Options, OptionError> {
    let matches = App::new("bklt")
        .version(crate_version!())
        .arg(Arg::with_name("max-file")
            .help("location of a file containing max brightness value")
            .short("M")
            .takes_value(true)
            .default_value("/sys/class/backlight/intel_backlight/max_brightness")
        )
        .arg(Arg::with_name("bri-file")
            .help("location of a file containing current brightness value")
            .short("B")
            .takes_value(true)
            .default_value("/sys/class/backlight/intel_backlight/brightness")
        )
        .arg(Arg::with_name("increase")
            .help("amount of percent to increase brightness for")
            .short("i")
            .takes_value(true)
        )
        .arg(Arg::with_name("decrease")
            .help("amount of percent to decrease brightness for")
            .short("d")
            .takes_value(true)
        )
        .arg(Arg::with_name("set")
            .help("value in percent to set brightness to")
            .short("s")
            .takes_value(true)
        )
        .get_matches();

    let max_file = matches.value_of("max-file").unwrap();
    let bri_file = matches.value_of("bri-file").unwrap();
    let inc = matches.value_of("increase");
    let dec = matches.value_of("decrease");
    let set = matches.value_of("set");

    let amount_set: u8 = [inc, dec, set].iter().map(|x| if x.is_some() {1} else {0}).sum();
    if amount_set < 1 {
        return Err(OptionError::from("one of 'i', 'd' or 's' must be set"));
    }
    if amount_set > 1 {
        return Err(OptionError::from("only one of 'i', 'd' or 's' must be set"));
    }
    
    Ok(Options{
        max_file: String::from(max_file),
        bri_file: String::from(bri_file),
        inc: match inc {
            Some(x) => Some(x.parse()?),
            _ => None,
        },
        dec: match dec {
            Some(x) => Some(x.parse()?),
            _ => None,
        },
        set: match set {
            Some(x) => Some(x.parse()?),
            _ => None,
        },
    })
}
