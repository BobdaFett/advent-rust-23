// This is a collection of functions that I think I'll use in the future.

pub fn substring(string: &String, start: usize, end: usize) -> String {
    string.chars().skip(start).take(end).collect::<String>()
}

pub fn read_input(day: &str) -> std::io::Result<String> {
    std::fs::read_to_string(format!("C:\\Users\\lvas4\\Documents\\advent-rust-23\\input{}.txt", day))
}
