use directories::ProjectDirs;
use std::env;
use std::fs;
use std::io;
use std::path::PathBuf;
use which::which;

mod browser;
use crate::browser::Browser;

pub fn can_find_driver(driver: &str) -> PathBuf {
    let result = which(driver);
    match result {
        Ok(path) => path,
        Err(_) => PathBuf::new(),
    }
}

pub fn get_project_dir() -> io::Result<PathBuf> {
    let proj_dirs = ProjectDirs::from("org", "webdriver", "browser-manager");
    match proj_dirs {
        Some(proj_dir) => {
            let selenium_dir = proj_dir.config_dir();
            if selenium_dir.is_dir() {
                Ok(PathBuf::from(selenium_dir))
            } else {
                let _created = fs::create_dir_all(selenium_dir);
                match _created {
                    Ok(_) => Ok(PathBuf::from(selenium_dir)),
                    Err(_) => panic!("Could not create the project directory"),
                }
            }
        }
        None => {
            panic!("Could not look up project directory")
        }
    }
}

pub fn find_browser_for(browser_name: String) -> Option<Browser> {
    let available_browsers = get_available_browsers();
    let mut found_browser = None;

    for browser in &available_browsers {
        if browser.name.eq(&browser_name) {
            found_browser = Some(browser.to_owned());
            break;
        }
    }
    found_browser
}

fn get_available_browsers() -> Vec<Browser> {
    let browsers: Vec<&str> = vec!["firefox", "Google Chrome"];
    let mut available_browsers: Vec<Browser> = vec![];

    for exe in &browsers {
        let result = which(exe);
        match result {
            Ok(path) => {
                if path.display().to_string().contains("firefox") {
                    let firefox = Browser::new(
                        "firefox".to_string(),
                        can_find_driver("geckodriver").display().to_string(),
                        path.display().to_string(),
                    );
                    available_browsers.push(firefox);
                } else {
                    let chrome = Browser::new(
                        "chrome".to_string(),
                        can_find_driver("chromedriver").display().to_string(),
                        path.display().to_string(),
                    );
                    available_browsers.push(chrome);
                }
            }
            Err(_) => {}
        }
    }

    if available_browsers.len() == 0 {
        // Let's check if they might be available in the usual places on Mac.
        // They should have been caught on other platforms
        if let Some(browser) = check_path(
            "firefox".to_string(),
            "/Applications/Firefox.app/Contents/MacOS/firefox-bin",
            "geckodriver",
        ) {
            available_browsers.push(browser);
        }

        if let Some(browser) = check_path(
            "chrome".to_string(),
            "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome",
            "chromedriver",
        ) {
            available_browsers.push(browser);
        }
    }

    if is_mac() {
        let safari = Browser::new(
            "Safari".to_string(),
            "/usr/bin/safaridriver".to_string(),
            "/Applications/Safari.app/Contents/MacOS/Safari".to_string(),
        );
        available_browsers.push(safari);
    }

    available_browsers
}

fn check_path(name: String, path: &str, driver: &str) -> Option<Browser> {
    let browser_path = PathBuf::from(path);

    if browser_path.is_file() {
        let browser = Browser::new(
            name,
            can_find_driver(driver).display().to_string(),
            browser_path.display().to_string(),
        );
        Some(browser)
    } else {
        None
    }
}

fn is_mac() -> bool {
    env::consts::OS.to_string().eq(&"macos".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn cant_find_drivers() {
        let drivers = which("geckodriver");
        match drivers {
            Ok(path) => match env::var("PATH") {
                Ok(value) => {
                    let paths = env::split_paths(&value);
                    let tmp__ = path.as_path().display().to_string();
                    let mut tmp_path: Vec<&str> = tmp__.split("/").collect();
                    tmp_path.pop();
                    let driver_path = tmp_path.join("/");
                    let mut new_paths: Vec<String> = vec![];
                    for pat in paths {
                        if driver_path.ne(&pat.display().to_string()) {
                            new_paths.push(pat.display().to_string());
                        }
                    }

                    env::set_var("PATH", &new_paths.join(":"));

                    let need_path = can_find_driver("geckodriver");
                    assert_eq!(need_path.display().to_string(), "".to_string());
                }
                Err(_) => {}
            },
            Err(_) => {}
        }
    }

    #[test]
    fn can_find_drivers_on_path() {
        // This test assumes that drivers are already on the path.
        let need_path = can_find_driver("geckodriver");
        assert_ne!(need_path.display().to_string(), "");
    }

    #[test]
    fn browsers_available_on_each_platform() {
        // We need to mostly check that we don't get Safari on other platforms
        let available_browsers = get_available_browsers();

        if is_mac() {
            // Safari is always available on Mac.
            assert!(available_browsers.len() >= 2);
        } else {
            assert!(available_browsers.len() >= 1);
        }
    }

    #[test]
    fn should_be_found_and_returned() {
        // This test assumes that there is a browser available and found

        let found_browser = find_browser_for("firefox".to_string());
        match found_browser {
            Some(browser) => {
                assert_eq!(browser.name, "firefox".to_string())
            }
            None => assert!(false, "Was not able to find browsers on the machine"),
        }
    }
}
