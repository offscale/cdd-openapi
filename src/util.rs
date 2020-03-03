pub(crate) fn truncate(s: String, max_width: usize) -> String {
    s.chars().take(max_width).collect()
}

pub(crate) fn read_file(path: &str) -> Result<String, std::io::Error> {
    use std::fs::File;
    use std::io::prelude::*;

    let mut f = File::open(std::path::PathBuf::from(path))?;
    let mut buffer = String::new();

    f.read_to_string(&mut buffer)?;

    Ok(buffer)
}
