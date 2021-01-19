mod browser;
use browser::Browser;

use browser_manager::{can_find_driver, get_available_browsers, get_project_dir};

use clap::{App, Arg};

fn main() {
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
    let project_dir;
    if let Ok(proj_dir) = get_project_dir() {
        project_dir = proj_dir;
    }

    let available_browsers = get_available_browsers();
    let requested_browser = Browser::new(
        matches.value_of("browser").unwrap().to_string(),
        "".to_string(),
        "".to_string(),
    );
    let mut found = false;
    for browser in &available_browsers {
        if browser.name.eq(&requested_browser.name) {
            found = true;
            break;
        }
    }
    if found {
        println!("We found the browser {}", requested_browser.name);
    } else {
        println!("No Browsers were foind");
    }
}
