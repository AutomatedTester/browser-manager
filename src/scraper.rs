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

fn parse_for_url(data: HashMap<String, String>) -> String {
    let application;
    match data.get("application") {
        Some(app) => application = app,
        None => panic!("Should have received an application name"),
    }
    let platform;
    match data.get("platform") {
        Some(plat) => platform = plat,
        None => panic!("Should have received aan application platform"),
    }
    let os: String;
    match data.get("bitness") {
        Some(bits) => {
            if platform == "linux" {
                if bits == "x86_64" {
                    os = format!("{}{}", platform, "64".to_string());
                } else {
                    os = platform.to_string();
                }
            } else if platform == "windows" {
                if bits == "x86_64" {
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
        data.insert("application".to_string(), "firefox".to_string());
        data.insert("platform".to_string(), "linux".to_string());
        data.insert("bitness".to_string(), "x86_64".to_string());
        data.insert("version".to_string(), "latest".to_string());

        let result = parse_for_url(data);
        let expected = "https://download.mozilla.org/?product=firefox-latest&os=linux64&lang=en-US"
            .to_string();
        assert_eq!(result, expected)
    }

    #[test]
    fn test_can_parse_version_to_url_for_windows_x86_64() {
        let mut data = HashMap::new();
        data.insert("application".to_string(), "firefox".to_string());
        data.insert("platform".to_string(), "windows".to_string());
        data.insert("bitness".to_string(), "x86_64".to_string());
        data.insert("version".to_string(), "latest".to_string());

        let result = parse_for_url(data);
        let expected =
            "https://download.mozilla.org/?product=firefox-latest&os=win64&lang=en-US".to_string();
        assert_eq!(result, expected)
    }

    #[test]
    fn test_can_parse_version_to_url_for_windows_x86() {
        let mut data = HashMap::new();
        data.insert("application".to_string(), "firefox".to_string());
        data.insert("platform".to_string(), "windows".to_string());
        data.insert("bitness".to_string(), "i686".to_string());
        data.insert("version".to_string(), "latest".to_string());

        let result = parse_for_url(data);
        let expected =
            "https://download.mozilla.org/?product=firefox-latest&os=win&lang=en-US".to_string();
        assert_eq!(result, expected)
    }

    #[test]
    fn test_can_parse_version_to_url_for_mac_os() {
        let mut data = HashMap::new();
        data.insert("application".to_string(), "firefox".to_string());
        data.insert("platform".to_string(), "mac".to_string());
        data.insert("bitness".to_string(), "x86_64".to_string());
        data.insert("version".to_string(), "latest".to_string());

        let result = parse_for_url(data);
        let expected =
            "https://download.mozilla.org/?product=firefox-latest&os=osx&lang=en-US".to_string();
        assert_eq!(result, expected)
    }
}
