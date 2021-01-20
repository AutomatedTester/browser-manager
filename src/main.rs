mod browser;
use browser::Browser;

use browser_manager::{find_browser_for, get_project_dir};

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
    let found_browser = find_browser_for(matches.value_of("browser").unwrap().to_string());
}
