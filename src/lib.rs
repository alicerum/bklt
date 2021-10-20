use std::error::Error;

mod options;

pub fn run() -> Result<(), Box<dyn Error>> {
    let _ = options::get_options()?;

    Ok(())
}
