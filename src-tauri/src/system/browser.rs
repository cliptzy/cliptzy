use crate::utils::find_executable;

pub fn get_installed_browsers_list() -> Vec<String> {
    let mut browsers = Vec::new();
    let targets = vec![
        (
            "chrome",
            vec!["chrome", "google-chrome", "chrome.exe", "Google Chrome.app"],
        ),
        (
            "edge",
            vec![
                "msedge",
                "msedge.exe",
                "microsoft-edge",
                "Microsoft Edge.app",
            ],
        ),
        ("firefox", vec!["firefox", "firefox.exe", "Firefox.app"]),
        (
            "brave",
            vec!["brave", "brave-browser", "brave.exe", "Brave.app"],
        ),
        ("opera", vec!["opera", "opera.exe", "Opera.app"]),
        ("vivaldi", vec!["vivaldi", "vivaldi.exe", "Vivaldi.app"]),
        ("safari", vec!["Safari.app"]),
    ];

    for (name, aliases) in targets {
        for alias in aliases {
            if find_executable(alias).is_some() {
                browsers.push(name.to_string());
                break;
            }
        }
    }

    #[cfg(target_os = "windows")]
    {
        let local_app_data = std::env::var("LOCALAPPDATA").unwrap_or_default();
        let roaming_app_data = std::env::var("APPDATA").unwrap_or_default();

        let extra_checks = vec![
            (
                "chrome",
                format!("{}\\Google\\Chrome\\User Data", local_app_data),
            ),
            (
                "edge",
                format!("{}\\Microsoft\\Edge\\User Data", local_app_data),
            ),
            (
                "firefox",
                format!("{}\\Mozilla\\Firefox\\Profiles", roaming_app_data),
            ),
            (
                "brave",
                format!(
                    "{}\\BraveSoftware\\Brave-Browser\\User Data",
                    local_app_data
                ),
            ),
            (
                "opera",
                format!("{}\\Opera Software\\Opera Stable", roaming_app_data),
            ),
            ("vivaldi", format!("{}\\Vivaldi\\User Data", local_app_data)),
        ];

        for (name, path_str) in extra_checks {
            if !browsers.contains(&name.to_string()) && std::path::Path::new(&path_str).exists() {
                browsers.push(name.to_string());
            }
        }
    }

    browsers
}
