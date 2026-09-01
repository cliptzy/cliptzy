pub mod date;

use std::path::PathBuf;

pub fn find_executable(name: &str) -> Option<PathBuf> {
    if let Ok(path) = which::which(name) {
        return Some(path);
    }

    let app_dir = crate::paths::app_data_dir();
    let local_bin = if cfg!(target_os = "windows") {
        if name.ends_with(".exe") {
            app_dir.join("bin").join(name)
        } else {
            app_dir.join("bin").join(format!("{}.exe", name))
        }
    } else if name.ends_with(".app") {
        app_dir.join("/Applications").join(name)
    } else {
        app_dir.join("bin").join(name)
    };

    if local_bin.exists() {
        return Some(local_bin);
    }

    None
}

pub fn kill_processes(names: &[&str]) {
    for name in names {
        #[cfg(not(target_os = "windows"))]
        {
            let _ = tokio::process::Command::new("killall").arg(name).spawn();
        }

        #[cfg(target_os = "windows")]
        {
            let exe_name = format!("{}.exe", name);
            let _ = tokio::process::Command::new("taskkill")
                .args(&["/IM", &exe_name, "/F"])
                .spawn();
        }
    }
}
