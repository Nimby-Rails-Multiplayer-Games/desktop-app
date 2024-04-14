pub fn get_saved_games_folder() -> Result<String, &'static str> {
    let home_dir = match std::env::var("HOME") {
        Ok(dir) => dir,
        Err(_) => return Err("Cannot find home directory")
    };

    let output = match std::process::Command::new("find")
        .arg(home_dir)
        .arg("-type").arg("d")
        .arg("-name").arg("Weird and Wry")
        .arg("-print0")
        .output() {
        Ok(output) => output,
        Err(_) => return Err("failed to find")
    };

    let lines: Vec<String> = output.stdout
        .split(|&b| b == b'\0')
        .map(|line_slice| String::from_utf8_lossy(line_slice).into_owned())
        .collect();

    println!("Debug output:");
    lines.clone().into_iter().for_each(|line| println!("{}", line));

    Ok("TODO".to_string())
}