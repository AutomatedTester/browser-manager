use directories::ProjectDirs;
use std::fs;
use std::io;
use std::path::PathBuf;
use which::which;

pub fn can_find_drivers() -> bool {
    let exes = ["geckodriver", "chromedriver"];
    let mut need_path: bool = false;

    for exe in &exes {
        let result = which(exe);
        match result {
            Ok(path) => {
                println!("We found it at {}", String::from(path.to_str().unwrap()));
            }
            Err(_) => {
                if !need_path {
                    need_path = true;
                }
            }
        }
    }
    need_path
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

                    let need_path = can_find_drivers();
                    assert!(need_path);
                }
                Err(_) => {}
            },
            Err(_) => {}
        }
    }

    #[test]
    fn can_find_drivers_on_path() {
        // This test assumes that drivers are already on the path.
        let need_path = can_find_drivers();
        assert!(!need_path);
    }
}
