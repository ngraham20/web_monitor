use crate::errors::*;
use std::io::Read;
use sha2::{Sha256, Digest};

pub fn run() -> Result<()> {
    Ok(())
}

fn compare_snapshots(a: Vec<u8>, b: Vec<u8>) -> Result<()> {
    Ok(())
}

fn save_snapshot(data: Vec<u8>) -> Result<()> {
    use std::fs;

    fs::write("snapshots.txt", data).chain_err(|| format!("Could not write to snapshots.txt"))?;

    Ok(())
}

fn load_snapshot() -> Result<Vec<u8>> {
    let mut body = std::fs::read("snapshots.txt").chain_err(|| "Invalid UTF-8 squence")?;
    Ok(body)
}

pub fn take_snapshot(url: &str) -> Result<Vec<u8>> {
    
    let mut res = reqwest::blocking::get(url).chain_err(|| "Failed to get web page.")?;
    let mut body = String::new();
    res.read_to_string(&mut body).chain_err(|| "Failed to read page to string.")?; // read webpage to string

    let mut hasher = Sha256::new();
    hasher.update(&body);
    let result = hasher.finalize(); // hash the webpage
    println!("Body: {}", &body);
    println!("Hash: {:02X?}", result); // show the snapshot hex

    Ok(result[..].to_vec())
}