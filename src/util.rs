use std::io::{Read, Write};
use reqwest::Error;
use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderValue, COOKIE};
use reqwest::redirect::Policy;
use std::fs::OpenOptions;

const PROJECT_ROOT: &'static str = env!("CARGO_MANIFEST_DIR");

fn load_session() -> Result<HeaderValue, Error> {
    let session_path = std::path::Path::new(PROJECT_ROOT).join(".aoc_session");
    if !session_path.exists() {
        panic!("no .aoc_session file found, can't download the input")
    }
    let session = std::fs::read_to_string(session_path).unwrap();
    Ok(HeaderValue::from_str(&format!("session={}", session)).unwrap())
}

fn download_input(year: u32, day: u32, path: &std::path::PathBuf) {
    let cookie_value = load_session().unwrap();

    let mut headers = HeaderMap::new();
    headers.insert(COOKIE, cookie_value);
    let client = Client::builder()
        .default_headers(headers)
        .redirect(Policy::none())
        .build().unwrap();
    
    let input = client
        .get(&format!("https://adventofcode.com/{year}/day/{day}/input"))
        .send()
        .and_then(|response| response.error_for_status())
        .and_then(|response| response.text())
        .map_err(|err| err.to_string()).unwrap();
    
    let mut file = OpenOptions::new();
    file.create(true);
    file.write(true)
        .open(path)
        .map_err(|err| format!("Failed to create file: {}", err)).unwrap()
        .write(input.as_bytes())
        .map_err(|err| format!("Failed to write to file: {}", err)).unwrap();
}

pub fn get_input(year: u32, day: u32) -> String {
    let path = std::path::Path::new(PROJECT_ROOT).join("input").join(format!("{year}-{day}.txt"));

    let mut input = String::new();
    if !path.exists() {
        println!("file-missing, downloading input..");
        download_input(year, day, &path);
    }

    let mut file = std::fs::File::open(path).unwrap();
    file.read_to_string(&mut input).unwrap();
    return input;
}