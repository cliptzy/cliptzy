pub mod manager;

pub use manager::{
    check_dependencies, install_dependencies, install_ytdlp, AppDependencies, DependencyProgress,
    DependencyStatus,
};
