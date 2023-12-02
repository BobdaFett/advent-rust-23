// This is a collection of functions that I think I'll use in the future.

pub fn substring(string: &String, start: usize, end: usize) -> String {
    string.chars().skip(start).take(end).collect::<String>()
}
