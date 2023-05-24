use std::error::Error;

pub fn read_file(filename: &str) -> Result<String, Box<dyn Error>> {
    let contents = std::fs::read_to_string(filename)?;
    Ok(contents)
}
