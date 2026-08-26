pub fn evaluate(line: &[u8]) -> String {
    let mut result = String::new();
    for &byte in line {
        match byte {
            b'\n' => result.push('\n'),
            b'\r' => result.push_str("^M"),
            b'\t' => result.push_str("^I"),
            0 => result.push_str("^@"),
            1..=26 => result.push(char::from(b'A' + byte - 1)),
            127 => result.push_str("^?"),
            128..=255 => {
                result.push_str("M-");
            }
            b => result.push(b as char),
        }
    }
    result
}
