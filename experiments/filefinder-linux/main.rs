mod filesystem;

fn main() {
    let save_files_directory = filesystem::get_saved_games_folder();

    match save_files_directory {
        Ok(path) => println!("{}", path),
        Err(err) => println!("{}", err)
    }
}
