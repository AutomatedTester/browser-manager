struct Browser {
    name: String,
    driver_path: String,
    browser_path: String,
}

impl Browser {
    fn new(name: String, driver_path: String, browser_path: String) -> Self {
        Self { name: name,
               driver_path: driver_path,
               browser_path:browser_path
            }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_browser() {
        let firefox = Browser::new(String::from("firefox"), String::from("driver_path"), String::from("browser_path"));
        assert_eq!(firefox.name, String::from("firefox"));
        assert_eq!(firefox.driver_path, String::from("driver_path"));
        assert_eq!(firefox.browser_path, String::from("browser_path"));
    }

}
