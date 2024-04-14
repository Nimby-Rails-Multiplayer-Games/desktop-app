use std::path::Path;
use std::process::Command;

fn find_saved_games_folder(dir: String) -> Result<String, &'static str> {
    let compatdata_subpath = "/compatdata/1134710/pfx/drive_c/users/steamuser/Saved Games/Weird and Wry";

    let output = match Command::new("find")
        .arg(dir)
        .arg("-type").arg("d")
        .arg("-name").arg("Weird and Wry")
        .arg("-print0")
        .output() {
        Ok(output) => output,
        Err(_) => return Err("failed to find")
    };

    let dirs: Vec<String> = output.stdout
        .split(|&b| b == b'\0')
        .map(|line_slice| String::from_utf8_lossy(line_slice).into_owned())
        .collect();

    let compatdata_finds = dirs.iter()
        .filter(|&dir| dir.contains(compatdata_subpath));
    let compatdata_count = compatdata_finds.clone().count();
    if compatdata_count == 1 {
        Ok(compatdata_finds.last().unwrap_or(&"".to_string()).to_string() + "/NIMBY Rails")
    } else if compatdata_count > 1 {
        //TODO: Couple to UI to allow selection of correct folder
        Err("Too many Saved Games folders found")
    } else {
        Err("No Saved Games folder found")
    }
}

fn is_dir_empty(dir: &str) -> Result<bool, &'static str> {
    let path = Path::new(dir);
    if path.exists() {
        let read_dir = match path.read_dir() {
            Ok(entries) => entries,
            Err(_) => return Err("Cannot read default Path")
        };
        return Ok(read_dir.count() == 0);
    }
    return Err("Path does not exists");
}

pub fn get_saved_games_folder() -> Result<String, &'static str> {
    let home_dir = match std::env::var("HOME") {
        Ok(dir) => dir,
        Err(_) => return Err("Cannot find home directory")
    };
    let compatdata_subpath = "/compatdata/1134710/pfx/drive_c/users/steamuser/Saved Games/Weird and Wry";
    let default_dirs = [
        format!("{home_dir}/.steam/debian-installation/steamapps{compatdata_subpath}/NIMBY Rails"),
        format!("{home_dir}/.steam/steam/steamapps{compatdata_subpath}/NIMBY Rails")];

    let empty_string = "".to_string();
    let default_dir = default_dirs.iter()
        .find_map(|dir| {
            if is_dir_empty(&dir).is_ok_and(|is_empty| !is_empty) {
                Some(dir)
            } else {
                None
            }
        })
        .unwrap_or(&empty_string);

    if !default_dir.is_empty() {
        return Ok(default_dir.to_string());
    }


    let found_dir = match find_saved_games_folder(home_dir) {
        Ok(dir) => dir,
        Err(_) => return Err("Cannot find Saved Games folder")
    };

    let is_empty = is_dir_empty(&found_dir).is_ok_and(|is_empty| is_empty);
    if !is_empty {
        return Ok(found_dir);
    } else if is_empty {
        //TODO: Couple to UI to allow confirmation of correct folder
        return Err("Saved Games folder empty");
    }

    return Err("Cannot get Saved Games folder");
}