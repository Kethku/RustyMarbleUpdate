#![windows_subsystem = "windows"]

use std::io;
use std::fs;
use std::fs::File;

fn main() {
    let url = "http://rustymarble.eastus.cloudapp.azure.com:8080/current.jpg";
    let path = "c:/dev/Temp/current.jpg";

    {
        let mut response = reqwest::get(url).expect("request failed");
        let mut out = File::create(path).expect("Failed to create file.");
        io::copy(&mut response, &mut out).expect("Failed to copy content.");
    }

    let absolute_path_buf = fs::canonicalize(path).expect("Could not canonicalize");
    let absolute_path = absolute_path_buf.to_str().expect("Path contains weird characters.");
    wallpaper::set_from_path(absolute_path).expect("File is not an image");
}
