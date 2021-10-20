use std::fs::{File, OpenOptions};
use std::io::prelude::*;

use super::ProgramError;

pub fn read_value_from_file(path: &str) -> Result<u32, ProgramError> {
    let mut f = match File::open(path) {
        Ok(f) => f,
        Err(_) => return Err(ProgramError(
                format!("could not open file '{}'", path))),
    };

    let mut sval = String::new();
    if let Err(_) = f.read_to_string(&mut sval) {
        return Err(ProgramError(
                    format!("error reading from file '{}'", path)));
    }

    match sval.trim_end().parse() {
        Ok(v) => Ok(v),
        Err(_) => Err(ProgramError(
                    format!("malformed contents in file '{}'", path)))
    }
}

fn write_new_value(path: &str, val: u32) -> Result<(), ProgramError> {
    let mut f = match OpenOptions::new().write(true).create(false).open(path) {
        Ok(f) => f,
        Err(_) => return Err(ProgramError(
                format!("could not open file for write: '{}'", path))),
    };
    if let Err(_) = f.write(val.to_string().as_bytes()) {
        return Err(ProgramError(
                format!("could not write to file '{}'", path)));
    }
    if let Err(_) = f.flush() {
        return Err(ProgramError(
                format!("could not flush file '{}'", path)));

    }

    Ok(())
}

pub fn set(path: &str, value: u8, max: u32) -> Result<(), ProgramError> {
    let new_value = max * value as u32 / 100;
    write_new_value(path, new_value)
}

pub fn change(path: &str, delta: i8, max:u32) -> Result<(), ProgramError> {
    let current = read_value_from_file(path)?;

    let delta = (delta as i32) * (max as i32) / 100;

    let mut new;
    if delta < 0 && delta.abs() as u32 > current {
        new = 0;
    } else {
        // new value should be positive after adding delta,
        // since abs(delta) is smaller than current
        // so it's safe to cast to unsigned
        new = (current as i32 + delta) as u32;
        if new > max {
            new = max;
        }
    }

    write_new_value(path, new)
}
