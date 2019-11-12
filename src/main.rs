use std::io;
use std::fs;
use std::fs::File;

fn main() {
    let url = "http://rustymarble.eastus.cloudapp.azure.com:8080/current.jpg";
    let relative_path = "./current.jpg";

    {
        let mut response = reqwest::get(url).expect("request failed");
        let mut out = File::create(relative_path).expect("Failed to create file.");
        io::copy(&mut response, &mut out).expect("Failed to copy content.");
    }

    let absolute_path_buf = fs::canonicalize(relative_path).expect("Could not canonicalize");
    let absolute_path = absolute_path_buf.to_str().expect("Path contains weird characters.");
    wallpaper::set_from_path(absolute_path).expect("File is not an image");
}
