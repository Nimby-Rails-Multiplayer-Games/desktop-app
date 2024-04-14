#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "linux")]
mod linux;

pub fn get_saved_games_folder() -> Result<String, &'static str> {
    #[cfg(target_os = "windows")]
    {
        windows::get_saved_games_folder()
    }
    #[cfg(target_os = "linux")]
    {
        linux::get_saved_games_folder()
    }
}