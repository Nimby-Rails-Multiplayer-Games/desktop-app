use std::process::Command;

trait FileFinder {
    fn get_saved_games_folder() -> String;
}

struct LinuxFileFinder {}

struct WindowsFileFinder {}


impl FileFinder for LinuxFileFinder {
    fn get_saved_games_folder() -> String {
        let output = Command::new("find")
            .arg("~")
            .arg("-type d")
            .arg("-name \"Weird and Wry\"")
            .arg("-print0")
            .output()
            .expect("failed to find");

        let lines = output.stdout
            .split(|&b| b == b'\n')
            .map(|line| line.unwrap_or(line));

        let clean_lines = lines
            .for_each(|line| -> String { String::from_utf8(line) });

        let s = match String::from_utf8(output.stdout) {
            Ok(v) => v,
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e)
        };

        return "TODO".to_string();
    }
}