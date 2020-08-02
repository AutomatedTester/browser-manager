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
                println!("Executable {} not found", exe);
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

pub fn get_own_path() -> io::Result<()> {
    loop {
        let mut path_dir = String::new();
        io::stdin().read_line(&mut path_dir)?;
        if Path::new(path_dir.trim()).is_dir() {
            // Set the PATH

            break;
        } else {
            println!("Unfortunately the path given was not a directory. Please enter a directory");
            path_dir.clear();
        }
    }
    Ok(())
}