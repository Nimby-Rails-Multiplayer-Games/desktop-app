mod filefinder;

use filefinder::*;

fn main() {
    let current_os = std::env::consts::OS;

    let saved_games_folder: Result<String, &'static str>;
    if current_os == "linux" {
        saved_games_folder = LinuxFileFinder::get_saved_games_folder();
    } else if current_os == "windows" {
        panic!("Windows not yet supported");
    } else {
        panic!("This app only supports Windows and Linux.");
    }

    match saved_games_folder {
        Ok(path) => println!("{}", path),
        Err(err) => panic!("{}", err)
    }
}