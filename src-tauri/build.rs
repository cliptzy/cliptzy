fn main() {
    println!("cargo:rerun-if-changed=../.env");
    println!("cargo:rerun-if-changed=.env");

    let mut loaded = false;

    // Try current dir (.env in src-tauri)
    if let Ok(path) = dotenvy::dotenv() {
        println!("cargo:rerun-if-changed={}", path.display());
        loaded = true;
    }
    // Try parent dir (.env in root cliptzy)
    else if let Ok(path) = dotenvy::from_filename("../.env") {
        println!("cargo:rerun-if-changed={}", path.display());
        loaded = true;
    }

    if loaded {
        for (key, value) in std::env::vars() {
            if key.starts_with("SUPABASE_") {
                println!("cargo:rustc-env={}={}", key, value);
            }
        }
    }

    tauri_build::build()
}
