use directories::ProjectDirs;
use std::fs;
use std::io;
use std::path::Path;
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

pub fn need_own_path() -> io::Result<bool> {
    let mut own_path: bool = false;
    if let Some(proj_dirs) = ProjectDirs::from("org", "webdriver", "browser-manager") {
        let selenium_dir = proj_dirs.config_dir();
        if selenium_dir.is_dir() {
            println!("Selenium dir is here at {:?}", selenium_dir.to_str());
        } else {
            loop {
                let mut sel_response = String::new();
                println!(
                    "You don't seem to have directory {:?}",
                    selenium_dir.to_str()
                );
                println!("Would you like to create it? [Y/n]");
                io::stdin().read_line(&mut sel_response)?;
                if sel_response.to_lowercase().trim() == "y" {
                    println!("Creating path {:?}", selenium_dir.to_str());
                    let created = fs::create_dir_all(selenium_dir);
                    match created {
                        Ok(_) => println!("Path created"),
                        Err(_) => {
                            println!("Could not create path. You will need to create your own directory and pass it in");
                            own_path = true;
                        }
                    }
                    break;
                } else if sel_response.to_lowercase().trim() == "n" {
                    println!("You will need to enter in your own path for Selenium to download and install items");
                    own_path = true;
                    break;
                } else {
                    sel_response.clear();
                }
            }
        }
    }
    Ok(own_path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cant_find_drivers() {
        let need_path = can_find_drivers();
        assert!(need_path);
    }
}
