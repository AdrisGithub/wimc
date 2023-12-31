use std::fs::{File, OpenOptions, read_to_string};
use std::io::Write;

use wimcm::WIMCError;

use crate::constants::{FILE_NAME, HOME, SLASH};

pub fn save(string: &str) -> Result<(), WIMCError> {
    home_file()?
        .write_all(string.as_bytes())
        .map_err(|_err| WIMCError)
}

fn home_file() -> Result<File, WIMCError> {
    OpenOptions::new()
        .create(true)
        .write(true)
        .append(false)
        .open(file_name()?)
        .map_err(|_err| WIMCError)
}

pub fn load() -> Result<String, WIMCError> {
    let result = read_to_string(file_name()?).map_err(|_err| WIMCError);
    result
}

fn file_name() -> Result<String, WIMCError> {
    std::env::var(HOME)
        .map(|string| format!("{}{}{}", string, SLASH, FILE_NAME))
        .map_err(|_err| WIMCError)
}

#[cfg(test)]
mod tests {
    use std::fs::remove_file;

    use crate::saver::{file_name, load, save};

    #[test]
    fn test() {
        const TEST_TEXT: &str = "Hello";
        save(TEST_TEXT).unwrap();
        assert_eq!(load().unwrap(), TEST_TEXT);
        save(TEST_TEXT).unwrap();
        assert_eq!(load().unwrap(), TEST_TEXT);
        let _ = remove_file(file_name().unwrap());
    }

    #[test]
    pub fn tests() {
        let filename = file_name();
        assert_eq!("/home/adri/wimc.json", filename.unwrap())
    }
} 
