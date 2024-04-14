use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;
use std::path::PathBuf;
use std::ptr::null_mut;
use windows_sys::Win32::UI::Shell::{SHGetKnownFolderPath, FOLDERID_SavedGames};

pub fn get_saved_games_folder() -> Result<String, &'static str> {
    let mut pointer: *mut u16 = null_mut();
    let result: Option<String>;

    unsafe {
        let error_code = SHGetKnownFolderPath(&FOLDERID_SavedGames, 0, 0, &mut pointer);
        if error_code != 0 {
            return Err("Something went wrong while attempting to retrieve the Saved Games folder location from the Win32 API.")
        }

        let pointer_data_length = (0..).take_while(|&i| *pointer.offset(i) != 0).count();
        let pointer_data_slice = std::slice::from_raw_parts(pointer, pointer_data_length);
        result = Some(OsString::from_wide(pointer_data_slice).into_string().unwrap())
    }

    let result = match result {
        Some(path) => path,
        None => return Err("Something went wrong while attempting to retrieve the Saved Games folder location from the Win32 API.")
    };

    let mut path = PathBuf::from(result);
    path.push("Weird And Wry");
    path.push("Nimby Rails");
    return Ok(path.to_str().unwrap().to_owned());
}