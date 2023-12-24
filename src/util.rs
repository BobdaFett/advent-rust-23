// This is a collection of functions that I think I'll use in the future.

pub fn substring(string: &String, start: usize, end: usize) -> String {
    string.chars().skip(start).take(end).collect::<String>()
}

/// Reads input from a file with the given day number.
/// These files must be stored in the %USERPROFILE%\Documents folder, and must be named "input{day}.txt"
pub fn read_input(day: &str) -> std::io::Result<String> {
    std::fs::read_to_string(format!("C:\\Users\\lvas4\\Documents\\advent-rust-23\\input{}.txt", day))
}
