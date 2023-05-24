use std::error::Error;

pub fn read_file(filename: &str) -> Result<String, Box<dyn Error>> {
    let contents = std::fs::read_to_string(filename)?;
    Ok(contents)
}

pub fn graphwiz_to_png(dot_filename: &str, png_filename: &str) -> Result<(), Box<dyn Error>> {
    let dot = std::process::Command::new("dot")
        .arg("-Tpng")
        .arg(dot_filename)
        .arg("-o")
        .arg(png_filename)
        .output()?;
    if !dot.status.success() {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!(
                "Failed to convert dot file to png: {}",
                String::from_utf8_lossy(&dot.stderr)
            ),
        )));
    }
    Ok(())
}
