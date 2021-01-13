use crate::scraper;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufReader, Error};
use std::path::PathBuf;
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
        let _versions = name.split("@").collect::<Vec<&str>>();
        let version;
        if _versions.len() > 1 {
            version = _versions[1].to_string();
        } else {
            version = "latest".to_string();
        }

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

    fn unpack_zip(&self, file: String) -> Result<bool, Error> {
        let zip_file = File::open(file)?;
        let zip_reader = BufReader::new(zip_file);

        let mut _zip = zip::ZipArchive::new(zip_reader)?;
        Ok(true)
    }

    fn _is_installer(&self, file: PathBuf) -> Result<bool, Error> {
        let file_path = file.as_path();
        if self.os.eq(&"linux".to_string()) {
            Ok(file_path.display().to_string().ends_with(".tar.gz"))
        } else if self.os.eq(&"windows".to_string()) {
            Ok(file_path.display().to_string().ends_with(".exe"))
        } else {
            Ok(file_path.display().to_string().ends_with(".dmg"))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::{ErrorKind, Write};

    #[test]
    fn create_new_strut_with_version_included() {
        let browser = Browser::new(
            "firefox@69".to_string(),
            "driver_path".to_string(),
            "browser_path".to_string(),
        );
        assert_eq!(browser.version, "69".to_string());
    }

    #[test]
    fn create_new_strut_with_version_as_latest() {
        let browser = Browser::new(
            "firefox@latest".to_string(),
            "driver_path".to_string(),
            "browser_path".to_string(),
        );
        assert_eq!(browser.version, "latest".to_string());
    }

    #[test]
    fn create_new_strut_with_no_version_passed_in() {
        let browser = Browser::new(
            "firefox".to_string(),
            "driver_path".to_string(),
            "browser_path".to_string(),
        );
        assert_eq!(browser.version, "latest".to_string());
    }

    #[test]
    fn check_is_installer_fails_with_wrong_type() {
        let firefox = Browser::new(
            String::from("firefox"),
            String::from("driver_path"),
            String::from("browser_path"),
        );
        let filez = create_file("invalid_file_type.txt".to_string());
        match filez {
            Ok(_) => {
                let file = PathBuf::from("invalid_file_type.txt");
                let is_installer = firefox._is_installer(file);
                match is_installer {
                    Ok(is_it) => {
                        assert!(!is_it)
                    }
                    Err(e) => assert_eq!(e.kind(), ErrorKind::NotFound),
                }
            }
            Err(_) => assert_ne!(1, 2, "Could no create file for test during setup"),
        }
    }

    #[test]
    fn check_is_installer_finds_file() {
        let firefox = Browser::new(
            String::from("firefox"),
            String::from("driver_path"),
            String::from("browser_path"),
        );

        let mut file_name = "valid_file_type.".to_string().to_owned();
        if firefox.os.eq(&"linux".to_string()) {
            file_name.push_str("tar.gz");
        } else if firefox.os.eq(&"windows".to_string()) {
            file_name.push_str("exe");
        } else {
            file_name.push_str("dmg");
        }
        let fil_name = file_name.clone();
        println!("{}", fil_name);
        let filez = create_file(file_name);
        match filez {
            Ok(_) => {
                let file = PathBuf::from(fil_name);
                let is_installer = firefox._is_installer(file);
                match is_installer {
                    Ok(is_it) => {
                        assert!(is_it)
                    }
                    Err(e) => assert_eq!(e.kind(), ErrorKind::NotFound),
                }
            }
            Err(_) => assert_ne!(1, 2, "Could no create file for test during setup"),
        }
    }

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
