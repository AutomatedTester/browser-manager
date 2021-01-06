mod browser;
mod scraper;

use clap::{App, Arg};

use browser_manager::{can_find_drivers, need_own_path};

fn main() {
    App::new("Browser Manager")
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

    let need_driver_path = can_find_drivers();

    if need_driver_path {
        let needs_own_path = need_own_path();

        match needs_own_path {
            Ok(own_path) => if own_path {},
            Err(_) => {}
        }
    }
}
