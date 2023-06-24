//use directories::ProjectDirs;
use std::{
    fs::{create_dir_all, read_to_string, write},
    path::{PathBuf, Path},
};

/// Manage application config files
#[derive(Debug, Default, Clone)]
pub struct Config {
    //pub project_dirs: ProjectDirs,
    pub config_dir: PathBuf,
    pub path: PathBuf,
}

impl Config {
    pub fn new(config_dir: impl Into<PathBuf>, file: impl Into<String>) -> Self {
        //let path = Config::format_path(project_dirs, file.into());
        //let path = Config::format_path(project_dirs, file.into());
        //Self { config_dir, path }
        let config_dir = config_dir.into();
        let path = Config::format_path(&config_dir, file.into());
        Self { config_dir, path }
    }

    //fn format_path(project_dirs: ProjectDirs, config: impl Into<String>) -> PathBuf {
        //project_dirs.config_dir().to_path_buf().join(config.into())
    //}
    fn format_path(config_dir: &Path, config: impl Into<String>) -> PathBuf {
        config_dir.join(config.into())
    }

    pub fn make_dirs(&self) {
        //create_dir_all(self.project_dirs.config_dir()).expect("Could not create config directory")
        create_dir_all(&self.config_dir).expect("Could not create config directory")
    }

    pub fn read(&self) -> String {
        read_to_string(&self.path).expect("Could not read config file.")
    }

    pub fn write(&self, conts: &str) -> std::io::Result<()> {
        write(&self.path, conts)
    }
}
