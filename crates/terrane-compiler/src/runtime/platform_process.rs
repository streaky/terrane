
fn terrane_unhex(text: &str) -> Vec<u8> {
    fn digit(byte: u8) -> Option<u8> {
        match byte {
            b'0'..=b'9' => Some(byte - b'0'),
            b'a'..=b'f' => Some(byte - b'a' + 10),
            b'A'..=b'F' => Some(byte - b'A' + 10),
            _ => None,
        }
    }
    text.as_bytes()
        .chunks_exact(2)
        .filter_map(|pair| Some((digit(pair[0])? << 4) | digit(pair[1])?))
        .collect()
}

fn terrane_platform_value(value: std::ffi::OsString) -> String {
    terrane_platform_support::platform_value(value)
}


fn terrane_platform_value_is_text(value: &str) -> bool {
    value.starts_with("text:")
}

fn terrane_platform_value_text(value: &str) -> String {
    value.strip_prefix("text:").unwrap_or("").to_owned()
}

fn terrane_platform_value_bytes(value: &str) -> Vec<u8> {
    value
        .strip_prefix("raw:")
        .map(terrane_unhex)
        .unwrap_or_default()
}

fn terrane_process_arguments() -> Vec<String> {
    std::env::args_os().skip(1).map(terrane_platform_value).collect()
}

fn terrane_environment_entries() -> Vec<String> {
    std::env::vars_os()
        .flat_map(|(name, value)| [terrane_platform_value(name), terrane_platform_value(value)])
        .collect()
}

fn terrane_process_exit(code: terrane_int_support::Int) {
    let code = terrane_int_support::checked_coerce::<i32>(&code).unwrap_or(255);
    std::process::exit(code)
}
