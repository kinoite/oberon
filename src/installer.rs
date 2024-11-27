use reqwest::blocking::Client;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use flate2::read::GzDecoder;
use tar::Archive;

fn download_package(name: &str, version: &str, registry_url: &str, cache_dir: &Path) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let url = format!("{}/packages/{}/{}/download", registry_url, name, version);
    let client = Client::new();
    let response = client.get(&url).send()?.error_for_status()?;
    let tarball_name = format!("{}-{}.tar.gz", name, version);
    let tarball_path = cache_dir.join(&tarball_name);
    fs::create_dir_all(cache_dir)?;
    let mut file = File::create(&tarball_path)?;
    let mut content = io::Cursor::new(response.bytes()?);
    io::copy(&mut content, &mut file)?;
    Ok(tarball_path)
}

fn unpack_package(tarball_path: &Path, install_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    fs::create_dir_all(install_dir)?;
    let tar_gz = File::open(tarball_path)?;
    let tar = GzDecoder::new(tar_gz);
    let mut archive = Archive::new(tar);
    archive.unpack(install_dir)?;
    Ok(())
}

pub fn install_package(
    name: &str,
    version: &str,
    registry_url: &str,
    cache_dir: &Path,
    install_dir: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    let tarball_path = download_package(name, version, registry_url, cache_dir)?;
    unpack_package(&tarball_path, install_dir)?;
    Ok(())
}
