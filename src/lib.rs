use std::error::Error;
use std::fmt;
use std::fmt::Display;

mod options;
mod update;

#[derive(Debug)]
pub struct ProgramError(String);

impl Display for ProgramError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for ProgramError {}

impl From<options::OptionError> for ProgramError {
    fn from(e: options::OptionError) -> Self {
        ProgramError(e.to_string())
    }
}

pub fn run() -> Result<(), ProgramError> {
    let o = options::get_options()?;

    let max = update::read_value_from_file(&o.max_file)?;

    if let Some(s) = o.set {
        if s > 100 {
            return Err(ProgramError("value of 'set' must be between 0 and 100".into()));
        }
        return update::set(&o.bri_file, s, max);
    }

    if let Some(i) = o.inc {
        if i > 100 {
            return Err(ProgramError("value of 'inc' must be between 0 and 100".into()));
        }
        return update::change(&o.bri_file, i as i8, max);
    }

    if let Some(d) = o.dec {
        if d > 100 {
            return Err(ProgramError("value of 'dec' must be between 0 and 100".into()));
        }
        return update::change(&o.bri_file, -(d as i8), max);
    }

    Ok(())
}
