use std::io::{self, Write};

// Виводить підказку та читає один рядок зі stdin; None — EOF або помилка вводу
pub fn read_line(prompt: &str) -> Option<String> {
    print!("{prompt}");
    io::stdout().flush().ok()?;

    let mut input = String::new();
    if io::stdin().read_line(&mut input).ok()? == 0 {
        return None;
    }
    Some(input.trim().to_string())
}
