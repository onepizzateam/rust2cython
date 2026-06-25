/// Say hello to someone
pub fn greet(name: &str) -> String { format!("hello {}", name) }
pub fn repeat(s: &str, n: i32) -> String { s.repeat(n as usize) }
pub fn byte_length(s: &str) -> usize { s.len() }
pub fn join_strings(words: Vec<String>) -> String { words.join(" ") }
