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

    //making an optimistic assumption that there will only be one dir left, should probably check actually
    let saved_games_folder: String = dirs.iter()
        .filter(|&dir| dir.contains(compatdata_subpath))
        .last().unwrap().to_string();

    Ok(saved_games_folder + "/NIMBY Rails")
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

    default_dirs.iter().find_map(|dir| {
        if Self::is_dir_empty(&dir).is_ok_and(|is_empty| !is_empty) {
            Some(dir)
        } else {
            None
        }
    });

    let found_dir = match Self::find_saved_games_folder(home_dir) {
        Ok(dir) => dir,
        Err(_) => return Err("Cannot find Saved Games folder")
    };

    if Self::is_dir_empty(&found_dir).is_ok_and(|is_empty| !is_empty) {
        return Ok(found_dir);
    }

    return Err("Cannot get Saved Games folder");
}