use std::fmt;
use std::fmt::Display;
use std::fs;
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

const BACKLIGHT_DIR: &str = "/sys/class/backlight/";

pub fn get_options() -> Result<Options, OptionError> {
    let matches = App::new("bklt")
        .version(crate_version!())
        .usage("This program tries to deduce correct way of setting backlight in an X \
            environment.\n    In case it fails to do so, user can set 'M' and 'B' flags \
            manually.\n    Only one of 's', 'i' or 'd' flags is allowed")
        .arg(Arg::with_name("max-file")
            .help("optional: location of a file containing max brightness value")
            .short("M")
            .takes_value(true)
        )
        .arg(Arg::with_name("bri-file")
            .help("optional: location of a file containing current brightness value")
            .short("B")
            .takes_value(true)
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

    let max_file = matches.value_of("max-file");
    let bri_file = matches.value_of("bri-file");

    let mut driver = "".to_string();

    if max_file.is_none() || bri_file.is_none() {
        driver = determine_backlight()?;
    }

    let max_file = match max_file {
        Some(s) => s.to_string(),
        None => driver.clone() + "/max_brightness",
    };
    let bri_file = match bri_file {
        Some(s) => s.to_string(),
        None => driver + "/brightness",
    };

    println!("{}", max_file);
    println!("{}", bri_file);

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

fn determine_backlight() -> Result<String, OptionError> {
    let mut contents = match fs::read_dir(BACKLIGHT_DIR) {
        Ok(c) => c,
        Err(_) => return Err(OptionError::from(
                &format!(
                    "could not read directory '{}', consider setting 'M' and 'B' flags",
                    BACKLIGHT_DIR,
                ))),
    };

    let first = match contents.next() {
        Some(f) => f,
        None => return Err(OptionError::from(
                &format!(
                    "empty '{}', consider setting 'M' and 'B' flags",
                    BACKLIGHT_DIR,
                ))),
    };

    Ok(first.unwrap().path().to_str().unwrap().into())
}
