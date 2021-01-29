use crate::get_project_dir;

use flate2::read::GzDecoder;
use reqwest;
use std::collections::HashMap;
use std::env;
use std::fs::{create_dir_all, set_permissions, File, Permissions};
use std::io::{copy, Error, Write};
use std::path::PathBuf;
use tar::Archive;
use zip::ZipArchive;

#[derive(Debug, Clone)]
struct DownloadLinks {
    browser_url: String,
    driver_url: String,
    version: String,
}

#[derive(Debug, Clone)]
pub struct Browser {
    pub name: String,
    pub driver_path: String,
    pub browser_path: String,
    version: String,
    bitness: String,
    os: String,
}

impl Browser {
    pub fn new(name: String, driver_path: String, browser_path: String, version: String) -> Self {
        let os = env::consts::OS.to_string();
        let bitness = env::consts::ARCH.to_string();
        let _versions = name.split("@").collect::<Vec<&str>>();

        let _version;
        if version.eq(&"".to_string()) {
            if _versions.len() > 1 {
                _version = _versions[1].to_string();
            } else {
                _version = "latest".to_string();
            }
        } else {
            _version = version;
        }

        Self {
            name,
            driver_path,
            browser_path,
            version: _version,
            bitness,
            os,
        }
    }

    pub fn download(&self) -> Result<Browser, Box<dyn std::error::Error>> {
        let links = self.get_download_urls();
        if !self.browser_path.to_lowercase().contains(&self.name) {
            if let Ok(browser_response) = reqwest::blocking::get(&links.browser_url) {
                if let Ok(data) = browser_response.bytes() {
                    let mut browser_download_path = PathBuf::from(&self.browser_path);
                    browser_download_path.push(format!("{name}_browser.zip", name = &self.name));

                    File::create(browser_download_path)?.write_all(&data)?;
                }
            }
        }

        let mut driver_download_path;
        if self.driver_path.ne(&"".to_string()) {
            driver_download_path = PathBuf::from(&self.driver_path);
        } else {
            driver_download_path = PathBuf::from(get_project_dir()?);
        }
        let display = driver_download_path.clone();
        driver_download_path.push(
            &links
                .driver_url
                .split("/")
                .collect::<Vec<&str>>()
                .last()
                .unwrap(),
        );

        if let Ok(driver_response) = reqwest::blocking::get(&links.driver_url) {
            if let Ok(data) = driver_response.bytes() {
                File::create(&driver_download_path)?.write_all(&data)?;
                self.unpack_zip(driver_download_path.display().to_string())?;
            }
        }

        Ok(Browser::new(
            self.name.to_owned(),
            display.display().to_string(),
            self.browser_path.to_owned(),
            links.version,
        ))
    }

    fn get_download_urls(&self) -> DownloadLinks {
        let mut browser_detail = HashMap::new();
        browser_detail.insert("application".to_string(), &self.name);
        browser_detail.insert("platform".to_string(), &self.os);
        browser_detail.insert("version".to_string(), &self.version);
        browser_detail.insert("bitness".to_string(), &self.bitness);
        parse_for_urls(browser_detail)
    }

    fn unpack_zip(&self, file: String) -> Result<bool, Error> {
        let zip_file = File::open(&file)?;
        let is_tarball = file.ends_with(".tar.gz");
        let mut proj_dir = PathBuf::from(file);
        proj_dir.pop();

        if is_tarball {
            let tar = GzDecoder::new(zip_file);
            let mut archive = Archive::new(tar);
            archive.unpack(proj_dir.to_owned())?;
        } else {
            let mut archive = ZipArchive::new(zip_file)?;

            for i in 0..archive.len() {
                let mut _file = archive.by_index(i).unwrap();
                let mut outpath = proj_dir.to_owned();
                outpath.push(_file.sanitized_name());

                if (&*_file.name()).ends_with('/') {
                    println!("File {} extracted to \"{}\"", i, outpath.display());
                    create_dir_all(&outpath).unwrap();
                } else {
                    println!(
                        "File {} extracted to \"{}\" ({} bytes)",
                        i,
                        outpath.display(),
                        _file.size()
                    );
                    if let Some(p) = outpath.parent() {
                        if !p.exists() {
                            create_dir_all(&p).unwrap();
                        }
                    }
                    let mut outfile = File::create(&outpath).unwrap();
                    copy(&mut _file, &mut outfile).unwrap();
                }

                // Get and Set permissions
                #[cfg(unix)]
                {
                    use std::os::unix::fs::PermissionsExt;

                    if let Some(mode) = _file.unix_mode() {
                        set_permissions(&outpath, Permissions::from_mode(mode)).unwrap();
                    }
                }
            }
        }

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

const FIREFOX_BASE_URL: &str = "https://download.mozilla.org/?";
const FIREFOX_DRIVER_BASE_URL: &str = "https://github.com/mozilla/geckodriver/releases/download/";
const FIREFOX_DRIVER_LATEST: &str = "https://github.com/mozilla/geckodriver/releases/latest";
const CHROMEDRIVER_BASE_URL: &str = "https://chromedriver.storage.googleapis.com/";
const CHROMEDRIVER_LATEST_URL: &str = "https://chromedriver.storage.googleapis.com/LATEST_RELEASE";

fn parse_for_urls(data: HashMap<String, &String>) -> DownloadLinks {
    let application;
    match data.get("application") {
        Some(app) => application = app,
        None => panic!("Should have received an application name"),
    }

    let platform: String;
    match data.get("platform") {
        Some(plat) => platform = plat.to_string(),
        None => panic!("Should have received an application platform"),
    }
    let os: String;
    match data.get("bitness") {
        Some(&bits) => {
            if platform.eq(&"linux".to_string()) {
                if bits.eq(&"x86_64".to_string()) {
                    os = format!("{}{}", platform, "64".to_string());
                } else {
                    os = platform.to_string();
                }
            } else if platform.eq(&"windows".to_string()) {
                if bits.eq(&"x86_64".to_string()) {
                    if application.eq(&&"chrome".to_string()) {
                        os = format!("{}{}", "win".to_string(), "32".to_string());
                    } else {
                        os = format!("{}{}", "win".to_string(), "64".to_string());
                    }
                } else {
                    os = "win".to_string();
                }
            } else {
                if application.eq(&&"chrome".to_string()) {
                    os = format!("{}{}", "mac".to_string(), "64".to_string());
                } else {
                    os = "macos".to_string();
                }
            }
        }
        None => panic!("Should have received bitness for platform"),
    };

    let version;
    match data.get("version") {
        Some(ver) => version = ver,
        None => panic!("Could not find a valid file extension"),
    };

    let browser_path: String;
    let driver_path: String;
    let mut latest_version = String::new();
    if application.eq(&&"firefox".to_string()) {
        let browser_os;
        if os.eq(&"macos".to_string()) {
            browser_os = "osx".to_string();
        } else {
            browser_os = os.clone();
        }
        browser_path = format!(
            "{base_url}product={application}-{version}&os={os}&lang=en-US",
            base_url = FIREFOX_BASE_URL,
            application = application,
            version = version,
            os = browser_os
        );
        if let Ok(response) = reqwest::blocking::get(FIREFOX_DRIVER_LATEST) {
            let url = response.url();
            latest_version = url
                .as_str()
                .split("/")
                .collect::<Vec<&str>>()
                .last()
                .unwrap()
                .to_string();
        }

        let file_ending;
        if platform.eq(&"windows".to_string()) {
            file_ending = ".zip".to_string();
        } else {
            file_ending = ".tar.gz".to_string();
        }

        driver_path = format!(
            "{base_url}{version}/geckodriver-{version}-{os}{file_ending}",
            base_url = FIREFOX_DRIVER_BASE_URL,
            version = latest_version,
            os = os,
            file_ending = file_ending
        );
    } else {
        if let Ok(response) = reqwest::blocking::get(CHROMEDRIVER_LATEST_URL) {
            if let Ok(text) = response.text() {
                latest_version = text;
            }
        }
        if os.eq(&"mac64") {
            browser_path = format!("https://chromeenterprise.google/browser/download/thank-you/?platform={}&channel=stable&usagestats=0", os = "UNIVERSAL_MAC_DMG".to_string());
        } else {
            browser_path = format!("https://chromeenterprise.google/browser/download/thank-you/?platform={}_BUNDLE&channel=stable&usagestats=0", os = os,);
        }

        driver_path = format!(
            "{base_url}{latest_version}/chromedriver_{os}.zip",
            base_url = CHROMEDRIVER_BASE_URL,
            latest_version = latest_version,
            os = os,
        );
    }
    DownloadLinks {
        browser_url: browser_path,
        driver_url: driver_path,
        version: latest_version,
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
            "".to_string(),
        );
        assert_eq!(browser.version, "69".to_string());
    }

    #[test]
    fn create_new_strut_with_version_as_latest() {
        let browser = Browser::new(
            "firefox@latest".to_string(),
            "driver_path".to_string(),
            "browser_path".to_string(),
            "".to_string(),
        );
        assert_eq!(browser.version, "latest".to_string());
    }

    #[test]
    fn create_new_strut_with_no_version_passed_in() {
        let browser = Browser::new(
            "firefox".to_string(),
            "driver_path".to_string(),
            "browser_path".to_string(),
            "".to_string(),
        );
        assert_eq!(browser.version, "latest".to_string());
    }

    #[test]
    fn check_is_installer_fails_with_wrong_type() {
        let firefox = Browser::new(
            String::from("firefox"),
            String::from("".to_string()), // "driver_path"
            String::from("".to_string()), // "browser_path"
            "".to_string(),
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
            Err(_) => assert_ne!(1, 2, "Could not create file for test during setup"),
        }
    }

    #[test]
    fn check_is_installer_finds_file() {
        let firefox = Browser::new(
            String::from("firefox"),
            String::from("driver_path"),
            String::from("browser_path"),
            "".to_string(),
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
            String::from("driver_path".to_string()),
            String::from("browser_path".to_string()),
            "".to_string(),
        );
        assert_eq!(firefox.name, String::from("firefox"));
        assert_eq!(firefox.driver_path, String::from("driver_path"));
        assert_eq!(firefox.browser_path, String::from("browser_path"));
    }

    #[test]
    fn create_browser_get_download() {
        let firefox = Browser::new(
            String::from("firefox"),
            String::from("".to_string()), // "driver_path"
            String::from("".to_string()), // "browser_path"
            "".to_string(),
        );
        let download_url = firefox.get_download_urls();
        assert!(download_url
            .browser_url
            .contains("https://download.mozilla.org/?product=firefox-latest"));
        assert!(
            download_url.driver_url.contains("geckodriver-v"),
            format!("Result returned was {:?}", download_url)
        )
    }

    #[test]
    fn unpack_zip_file_wont_exist() {
        let firefox = Browser::new(
            String::from("firefox"),
            String::from("".to_string()), // "driver_path"
            String::from("".to_string()), // "browser_path"
            "".to_string(),
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
                    String::from("".to_string()), // "driver_path"
                    String::from("".to_string()), // "browser_path"
                    "".to_string(),
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

    #[test]
    fn test_can_parse_version_to_url_for_linux_x86_64() {
        let mut data = HashMap::new();
        let browser = "firefox".to_string();
        let os = "linux".to_string();
        let bitness = "x86_64".to_string();
        let version = "latest".to_string();
        data.insert("application".to_string(), &browser);
        data.insert("platform".to_string(), &os);
        data.insert("bitness".to_string(), &bitness);
        data.insert("version".to_string(), &version);

        let result = parse_for_urls(data);
        let expected = "https://download.mozilla.org/?product=firefox-latest&os=linux64&lang=en-US"
            .to_string();
        assert_eq!(result.browser_url, expected)
    }

    #[test]
    fn test_can_parse_version_to_url_for_windows_x86_64() {
        let mut data = HashMap::new();
        let firefox = "firefox".to_string();
        let windows = "windows".to_string();
        let bitness = "x86_64".to_string();
        let version = "latest".to_string();
        data.insert("application".to_string(), &firefox);
        data.insert("platform".to_string(), &windows);
        data.insert("bitness".to_string(), &bitness);
        data.insert("version".to_string(), &version);

        let result = parse_for_urls(data);
        let expected =
            "https://download.mozilla.org/?product=firefox-latest&os=win64&lang=en-US".to_string();
        assert_eq!(result.browser_url, expected)
    }

    #[test]
    fn test_can_parse_version_to_url_for_windows_x86() {
        let mut data = HashMap::new();
        let firefox = "firefox".to_string();
        let windows = "windows".to_string();
        let bitness = "x86".to_string();
        let version = "latest".to_string();
        data.insert("application".to_string(), &firefox);
        data.insert("platform".to_string(), &windows);
        data.insert("bitness".to_string(), &bitness);
        data.insert("version".to_string(), &version);

        let result = parse_for_urls(data);
        let expected =
            "https://download.mozilla.org/?product=firefox-latest&os=win&lang=en-US".to_string();
        assert_eq!(result.browser_url, expected)
    }

    #[test]
    fn test_can_parse_version_to_url_for_mac_os() {
        let mut data = HashMap::new();
        let firefox = "firefox".to_string();
        let windows = "mac".to_string();
        let bitness = "x86_64".to_string();
        let version = "latest".to_string();
        data.insert("application".to_string(), &firefox);
        data.insert("platform".to_string(), &windows);
        data.insert("bitness".to_string(), &bitness);
        data.insert("version".to_string(), &version);

        let result = parse_for_urls(data);
        let expected =
            "https://download.mozilla.org/?product=firefox-latest&os=osx&lang=en-US".to_string();
        assert_eq!(result.browser_url, expected)
    }

    #[test]
    fn can_parse_mac_url_for_chromedriver() {
        let mut data = HashMap::new();
        let firefox = "chrome".to_string();
        let windows = "mac".to_string();
        let bitness = "x86_64".to_string();
        let version = "latest".to_string();
        data.insert("application".to_string(), &firefox);
        data.insert("platform".to_string(), &windows);
        data.insert("bitness".to_string(), &bitness);
        data.insert("version".to_string(), &version);

        let result = parse_for_urls(data);
        let browser_expected = "UNIVERSAL_MAC_DMG".to_string();
        assert!(
            result.browser_url.contains(&browser_expected),
            format!("Result is {:?}", result)
        )
    }

    #[test]
    fn can_parse_windows_url_for_chromedriver() {
        let mut data = HashMap::new();
        let firefox = "chrome".to_string();
        let windows = "windows".to_string();
        let bitness = "x86_64".to_string();
        let version = "latest".to_string();
        data.insert("application".to_string(), &firefox);
        data.insert("platform".to_string(), &windows);
        data.insert("bitness".to_string(), &bitness);
        data.insert("version".to_string(), &version);

        let result = parse_for_urls(data);
        let expected = "chromedriver_win32.zip".to_string();
        assert!(
            result.driver_url.contains(&expected),
            format!("Result is {:?}", result)
        )
    }

    #[test]
    fn can_parse_linux_url_for_chromedriver() {
        let mut data = HashMap::new();
        let firefox = "chrome".to_string();
        let windows = "linux".to_string();
        let bitness = "x86_64".to_string();
        let version = "latest".to_string();
        data.insert("application".to_string(), &firefox);
        data.insert("platform".to_string(), &windows);
        data.insert("bitness".to_string(), &bitness);
        data.insert("version".to_string(), &version);

        let result = parse_for_urls(data);
        let expected = "chromedriver_linux64.zip".to_string();
        assert!(
            result.driver_url.contains(&expected),
            format!("Result is {:?}", result)
        )
    }
}
