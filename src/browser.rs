use std::fs::File;
use std::io::{BufReader, Error, ErrorKind};
use zip::{read, ZipArchive};

struct Browser {
    name: String,
    driver_path: String,
    browser_path: String,
}

impl Browser {
    pub fn new(name: String, driver_path: String, browser_path: String) -> Self {
        Self {
               name: name,
               driver_path: driver_path,
               browser_path:browser_path
            }
    }

    pub fn unpack_zip(&self, file: String) -> Result<bool, Error> {
        let zip_file = File::open(file)?;
        let zip_reader = BufReader::new(zip_file);

        let mut zip = zip::ZipArchive::new(zip_reader)?;
        Ok(true)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_browser() {
        let firefox = Browser::new(String::from("firefox"), String::from("driver_path"), String::from("browser_path"));
        assert_eq!(firefox.name, String::from("firefox"));
        assert_eq!(firefox.driver_path, String::from("driver_path"));
        assert_eq!(firefox.browser_path, String::from("browser_path"));
    }

    #[test]
    fn unpack_zip_file_wont_exist() {
        let firefox = Browser::new(String::from("firefox"), String::from("driver_path"), String::from("browser_path"));
        let result = firefox.unpack_zip("file_wont_exist".to_string());
        match result {
            Ok(_) => assert_ne!(1, 2, "Should not have got an Ok on a file that doesn't exist"),
            Err(e) => assert_eq!(e.kind(), ErrorKind::NotFound)
        }
    }

}
