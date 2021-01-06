use lazy_static::lazy_static;

use std::collections::HashMap;

const BASE_URL: &str = "https://download.mozilla.org/?";
lazy_static! {
    static ref DEFAULT_FILE_EXTENSIONS: HashMap<String, String> = {
        let mut m = HashMap::new();
        m.insert("android-api-9".to_string(), "apk".to_string());
        m.insert("android-api-11".to_string(), "apk".to_string());
        m.insert("android-api-15".to_string(), "apk".to_string());
        m.insert("android-api-16".to_string(), "apk".to_string());
        m.insert("android-x86".to_string(), "apk".to_string());
        m.insert("linux".to_string(), "tar.bz2".to_string());
        m.insert("linux64".to_string(), "tar.bz2".to_string());
        m.insert("mac".to_string(), "dmg".to_string());
        m.insert("mac64".to_string(), "dmg".to_string());
        m.insert("win32".to_string(), "exe".to_string());
        m.insert("win64".to_string(), "exe".to_string());
        m
    };
}

pub fn parse_for_url(data: HashMap<String, &String>) -> String {
    let application;
    match data.get("application") {
        Some(app) => application = app,
        None => panic!("Should have received an application name"),
    }
    let platform: String;
    match data.get("platform") {
        Some(plat) => platform = plat.to_string(),
        None => panic!("Should have received an application platform"),
    }
    let os: String;
    match data.get("bitness") {
        Some(&bits) => {
            if platform.eq(&"linux".to_string()) {
                if bits.eq(&"x86_64".to_string()) {
                    os = format!("{}{}", platform, "64".to_string());
                } else {
                    os = platform.to_string();
                }
            } else if platform.eq(&"windows".to_string()) {
                if bits.eq(&"x86_64".to_string()) {
                    os = format!("{}{}", "win".to_string(), "64".to_string());
                } else {
                    os = "win".to_string();
                }
            } else {
                os = "osx".to_string();
            }
        }
        None => panic!("Should have received bitness for platform"),
    };

    let version;
    match data.get("version") {
        Some(ver) => version = ver,
        None => panic!("Could not find a valid file extension"),
    };
    let path = format!(
        "{base_url}product={application}-{version}&os={os}&lang=en-US",
        base_url = BASE_URL,
        application = application,
        os = os,
        version = version
    );
    path
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_can_parse_version_to_url_for_linux_x86_64() {
        let mut data = HashMap::new();
        let browser = "firefox".to_string();
        let os = "linux".to_string();
        let bitness = "x86_64".to_string();
        let version = "latest".to_string();
        data.insert("application".to_string(), &browser);
        data.insert("platform".to_string(), &os);
        data.insert("bitness".to_string(), &bitness);
        data.insert("version".to_string(), &version);

        let result = parse_for_url(data);
        let expected = "https://download.mozilla.org/?product=firefox-latest&os=linux64&lang=en-US"
            .to_string();
        assert_eq!(result, expected)
    }

    #[test]
    fn test_can_parse_version_to_url_for_windows_x86_64() {
        let mut data = HashMap::new();
        let firefox = "firefox".to_string();
        let windows = "windows".to_string();
        let bitness = "x86_64".to_string();
        let version = "latest".to_string();
        data.insert("application".to_string(), &firefox);
        data.insert("platform".to_string(), &windows);
        data.insert("bitness".to_string(), &bitness);
        data.insert("version".to_string(), &version);

        let result = parse_for_url(data);
        let expected =
            "https://download.mozilla.org/?product=firefox-latest&os=win64&lang=en-US".to_string();
        assert_eq!(result, expected)
    }

    #[test]
    fn test_can_parse_version_to_url_for_windows_x86() {
        let mut data = HashMap::new();
        let firefox = "firefox".to_string();
        let windows = "windows".to_string();
        let bitness = "x86".to_string();
        let version = "latest".to_string();
        data.insert("application".to_string(), &firefox);
        data.insert("platform".to_string(), &windows);
        data.insert("bitness".to_string(), &bitness);
        data.insert("version".to_string(), &version);

        let result = parse_for_url(data);
        let expected =
            "https://download.mozilla.org/?product=firefox-latest&os=win&lang=en-US".to_string();
        assert_eq!(result, expected)
    }

    #[test]
    fn test_can_parse_version_to_url_for_mac_os() {
        let mut data = HashMap::new();
        let firefox = "firefox".to_string();
        let windows = "mac".to_string();
        let bitness = "x86_64".to_string();
        let version = "latest".to_string();
        data.insert("application".to_string(), &firefox);
        data.insert("platform".to_string(), &windows);
        data.insert("bitness".to_string(), &bitness);
        data.insert("version".to_string(), &version);

        let result = parse_for_url(data);
        let expected =
            "https://download.mozilla.org/?product=firefox-latest&os=osx&lang=en-US".to_string();
        assert_eq!(result, expected)
    }
}
