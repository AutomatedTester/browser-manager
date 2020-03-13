extern crate directories;

//mod browser;
use directories::ProjectDirs;

use std::io;
use std::path::Path;

use which::which;

fn main() -> io::Result<()> {
    let exes = ["geckodriver", "chromedriver"];
    let mut need_path: bool = false;

    for exe in &exes {
        let result = which(exe);
        match result {
            Ok(path) => {
                println!("We found it at {}", String::from(path.to_str().unwrap()));
            },
            Err(_) => {
                println!("Executable {} not found", exe);
                if !need_path {
                    need_path = true;
                }
            }
        }
    }
    
    
    if need_path {
        let mut own_path: bool = false;
        if let Some(proj_dirs) = ProjectDirs::from("org", "webdriver",  "browser-manager") {
            let selenium_dir = proj_dirs.config_dir();
            if selenium_dir.is_dir() {

            } else {
                let valid_responses = ["y".to_string(), "n".to_string()];
                loop {
                    let mut sel_response = String::new();
                    println!("You don't seem to have directory {:?}", selenium_dir.to_str());
                    println!("Would you like to create it? [Y/n]");
                    io::stdin().read_line(&mut sel_response)?;
                    if valid_responses.contains(&sel_response.to_string()) {
                        if sel_response.trim().to_string() == "Y".to_string() {
                            println!("phew, great");

                        } else {
                            println!("You will need to enter in your own path for Selenium to download and install items");
                            own_path = true;
                        }
                        break;
                    } else {
                        sel_response.clear();
                    }

                }
            }
        }
        if own_path {
            loop {
                let mut path_dir = String::new();
                println!("Please enter the directory:");
                io::stdin().read_line(&mut path_dir)?;
                if Path::new(path_dir.trim()).is_dir() {
                    println!("This is a dir");

                    break;
                } else {
                    println!("Unfortunately the path given was not a directory. Please enter ina a directory");
                    path_dir.clear();
                }
            }
        }
    }
    Ok(())
}
