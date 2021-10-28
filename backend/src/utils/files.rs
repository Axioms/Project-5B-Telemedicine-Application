use std::{
    fs::{self, File},
    io::{Read, Result as IOResult, Error as ioError},
    path::Path,
};

pub fn file_exists(path: &str) -> bool {
    Path::new(path).exists()
}

pub fn read_file(path: &str) -> IOResult<Vec<u8>> {
    let mut contents: Vec<u8> = Vec::new();

    let mut file = File::open(Path::new(path))?;
    file.read_to_end(&mut contents)?;

    Ok(contents)
}

pub fn write_file(path: &str, content: &[u8]) -> Result<(), ioError> {
    use std::io::Write;
    let mut f = File::create(path)?;
    f.write_all(content)?;
    f.flush()?;
    Ok(())
}

pub fn read_file_string(path: &str) -> IOResult<String> {
    let mut contents = String::new();

    let mut file = File::open(Path::new(path))?;
    file.read_to_string(&mut contents)?;

    Ok(contents)
}

pub fn delete_file(path: &str) -> IOResult<()> {
    let res = fs::remove_file(path);

    if let Some(parent) = Path::new(path).parent() {
        // If the directory isn't empty, this returns an error, which we ignore
        // We only want to delete the folder if it's empty
        fs::remove_dir(parent).ok();
    }

    res
}