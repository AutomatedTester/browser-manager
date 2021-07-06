mod browser;
use browser::Browser;

use browser_manager::{find_browser_for, get_project_dir};

use clap::{App, Arg};
use log::info;
use std::fs::File;
use std::io::Write;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new("Browser Manager")
        .version("0.1.0")
        .author("David Burns <david.burns@theautomatedtester.co.uk")
        .about("Browser manager for selenium to download browsers and drivers")
        .arg(
            Arg::with_name("browser")
                .short("b")
                .long("browser")
                .value_name("browser_name")
                .help("Select the browser you wish to you with version. E.g. Firefox@69 or Chrome@latest")
                .takes_value(true),
        )
        .get_matches();

    if let Ok(mut project_dir) = get_project_dir() {
        let browser_needed = matches.value_of("browser").unwrap().to_string();
        let found_browser = find_browser_for(browser_needed.to_owned());
        match found_browser {
            Some(mut browser) => {
                // We have found a browser, let's just make sure it is detailed in the project directory
                if browser.driver_path.eq(&"".to_string()) {
                    browser = browser.download()?;
                    project_dir.push(format!("{}_details.json", browser.name));
                    info!("About to write to {}", project_dir.display());
                    File::create(project_dir)?
                        .write_all(serde_json::to_string(&browser)?.as_bytes())?;
                }
            }
            None => {
                // No Browsers found, let's get them downloaded and setup
                let needed = Browser::new(
                    browser_needed,
                    project_dir.display().to_string(),
                    project_dir.display().to_string(),
                    "".to_string(),
                );
                let browser = needed.download()?;
                project_dir.push(format!("{}_details.json", browser.name));
                info!("About to write to {}", project_dir.display());
                File::create(project_dir)?
                    .write_all(serde_json::to_string(&browser)?.as_bytes())?;
            }
        }
    }

    Ok(())
}
