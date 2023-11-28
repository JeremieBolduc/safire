use std::path::PathBuf;

use super::constants::APP_NAME;

pub fn get_app_path() -> PathBuf {
    let home_dir = dirs::home_dir().unwrap_or_default();
    let app_path = home_dir.join(format!(".{}", APP_NAME));

    app_path
}
