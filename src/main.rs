#![windows_subsystem = "windows"]

#[cfg(windows)]
fn main() {
    if let Err(e) = winisland::app::run() {
        eprintln!("winisland: {e}");
    }
}

#[cfg(not(windows))]
fn main() {
    eprintln!("WinIsland targets Windows 11 only.");
}
