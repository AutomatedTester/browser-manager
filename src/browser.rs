use crate::scraper;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufReader, Error};
use zip::{read, ZipArchive};

struct Browser {
    name: String,
    driver_path: String,
    browser_path: String,
    version: String,
    bitness: String,
    os: String,
}

impl Browser {
    pub fn new(name: String, driver_path: String, browser_path: String) -> Self {
        let os = env::consts::OS.to_string();
        let bitness = env::consts::ARCH.to_string();
        let version = "latest".to_string();
        Self {
            name,
            driver_path,
            browser_path,
            version,
            bitness,
            os,
        }
    }

    pub fn get_download_url(&self) -> String {
        let mut browser_detail = HashMap::new();
        browser_detail.insert("application".to_string(), &self.name);
        browser_detail.insert("platform".to_string(), &self.os);
        browser_detail.insert("version".to_string(), &self.version);
        browser_detail.insert("bitness".to_string(), &self.bitness);
        scraper::parse_for_url(browser_detail)
    }

    pub fn unpack_zip(&self, file: String) -> Result<bool, Error> {
        let zip_file = File::open(file)?;
        let zip_reader = BufReader::new(zip_file);

        let mut _zip = zip::ZipArchive::new(zip_reader)?;
        Ok(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::{ErrorKind, Write};

    #[test]
    fn create_browser() {
        let firefox = Browser::new(
            String::from("firefox"),
            String::from("driver_path"),
            String::from("browser_path"),
        );
        assert_eq!(firefox.name, String::from("firefox"));
        assert_eq!(firefox.driver_path, String::from("driver_path"));
        assert_eq!(firefox.browser_path, String::from("browser_path"));
    }

    #[test]
    fn create_browser_get_download() {
        let firefox = Browser::new(
            String::from("firefox"),
            String::from("driver_path"),
            String::from("browser_path"),
        );
        let download_url = firefox.get_download_url();
        assert!(download_url.contains("https://download.mozilla.org/?product=firefox-latest"));
    }

    #[test]
    fn unpack_zip_file_wont_exist() {
        let firefox = Browser::new(
            String::from("firefox"),
            String::from("driver_path"),
            String::from("browser_path"),
        );
        let result = firefox.unpack_zip("file_wont_exist".to_string());
        match result {
            Ok(_) => assert_ne!(
                1, 2,
                "Should not have got an Ok on a file that doesn't exist"
            ),
            Err(e) => assert_eq!(e.kind(), ErrorKind::NotFound),
        }
    }

    #[test]
    fn unpack_zip_file_not_zip() {
        // Setup
        let filz = create_file("cheese.txt".to_string());
        match filz {
            Ok(_) => {
                //Test
                let firefox = Browser::new(
                    String::from("firefox"),
                    String::from("driver_path"),
                    String::from("browser_path"),
                );
                let result = firefox.unpack_zip("cheese.txt".to_string());
                match result {
                    Ok(_) => assert_ne!(
                        1, 2,
                        "Should not have got an Ok on a file that doesn't exist"
                    ),
                    Err(e) => assert_eq!(e.kind(), ErrorKind::Other),
                }
            }
            Err(_) => assert_ne!(1, 2, "Could no create file for test during setup"),
        }
    }

    fn create_file(file: String) -> Result<File, Error> {
        let res_file = File::create(file);
        match res_file {
            Ok(mut file) => {
                let contents = file.write_all(b"Hello, world!");
                match contents {
                    Ok(_) => Ok(file),
                    Err(_) => panic!("Couldn't write to file"),
                }
            }
            Err(_) => panic!("Error when creating file for test"),
        }
    }
}
