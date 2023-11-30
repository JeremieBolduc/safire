use std::{
    error::Error,
    fs::{self, File},
    io::{self, BufRead},
    path::{Path, PathBuf},
    process::Command,
};

use colored::Colorize;

use super::{
    constants::{DECRYPTED_FILE_EXT, ENCRYPTED_FILE_EXT, GPG_RECIPIENT_FILENAME},
    paths::app_root,
};

pub fn get_gpg_recipient() -> Result<String, Box<dyn Error>> {
    let app_root = app_root();
    let file_path = app_root.join(GPG_RECIPIENT_FILENAME);

    let file = File::open(&file_path)?;
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let content = line?;

        if !content.trim().is_empty() {
            return Ok(content);
        }
    }

    Err("Could not find the gpg recipient".into())
}

pub struct GpgManager {
    recipient: String,
    decrypted_file_paths: Vec<PathBuf>,
}

impl GpgManager {
    pub fn new(recipient: &str) -> Self {
        GpgManager {
            recipient: recipient.to_owned(),
            decrypted_file_paths: Vec::new(),
        }
    }

    pub fn encrypt_file(&mut self, decrypted_file_path: &Path) -> Result<PathBuf, Box<dyn Error>> {
        let encrypted_file_path = decrypted_file_path.with_extension(ENCRYPTED_FILE_EXT);

        let mut gpg_command = Command::new("gpg");
        gpg_command
            .arg("--yes")
            .arg("--quiet")
            .arg("--recipient")
            .arg(&self.recipient)
            .arg("--output")
            .arg(&encrypted_file_path)
            .arg("--encrypt")
            .arg(decrypted_file_path.to_str().expect("Invalid path"));

        let status = gpg_command.status()?;

        if status.success() {
            fs::remove_file(decrypted_file_path)?;

            self.decrypted_file_paths
                .retain(|path| path != &decrypted_file_path);

            Ok(encrypted_file_path)
        } else {
            Err("GPG process exited with an error".into())
        }
    }

    pub fn decrypt_file(&mut self, encrypted_file_path: &Path) -> Result<PathBuf, Box<dyn Error>> {
        let decrypted_file_path = encrypted_file_path.with_extension(DECRYPTED_FILE_EXT);

        let mut gpg_command = Command::new("gpg");
        gpg_command
            .arg("--quiet")
            .arg("--recipient")
            .arg(&self.recipient)
            .arg("--output")
            .arg(&decrypted_file_path)
            .arg("--decrypt")
            .arg(encrypted_file_path.to_str().expect("Invalid path"));

        let status = gpg_command.status()?;

        if status.success() {
            fs::remove_file(encrypted_file_path)?;

            self.decrypted_file_paths.push(decrypted_file_path.clone());

            Ok(decrypted_file_path)
        } else {
            Err("GPG process exited with an error".into())
        }
    }

    fn reencrypt_all_files(&mut self) -> Result<Vec<PathBuf>, Box<dyn Error>> {
        let mut encrypted_file_paths = Vec::new();

        let decrypted_file_paths_clone: Vec<PathBuf> = self.decrypted_file_paths.clone();

        for decrypted_file_path in &decrypted_file_paths_clone {
            let encrypted_file_path = self.encrypt_file(decrypted_file_path.as_path())?;

            encrypted_file_paths.push(encrypted_file_path);
        }

        Ok(encrypted_file_paths)
    }
}

impl Drop for GpgManager {
    fn drop(&mut self) {
        if let Err(err) = self.reencrypt_all_files() {
            eprintln!("Error during files reencryption {}", err.to_string().red());
        }
    }
}
