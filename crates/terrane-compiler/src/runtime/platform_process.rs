fn terrane_hex(data: &[u8]) -> String {
    const DIGITS: &[u8; 16] = b"0123456789abcdef";
    let mut encoded = String::with_capacity(data.len() * 2);
    for byte in data {
        encoded.push(DIGITS[usize::from(byte >> 4)] as char);
        encoded.push(DIGITS[usize::from(byte & 0x0f)] as char);
    }
    encoded
}

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

#[cfg(unix)]
fn terrane_platform_value(value: std::ffi::OsString) -> String {
    use std::os::unix::ffi::OsStrExt as _;
    value.into_string().map_or_else(
        |raw| format!("raw:{}", terrane_hex(raw.as_bytes())),
        |text| format!("text:{text}"),
    )
}

#[cfg(windows)]
fn terrane_platform_value(value: std::ffi::OsString) -> String {
    use std::os::windows::ffi::OsStrExt as _;
    value.into_string().map_or_else(
        |raw| {
            let units = raw.encode_wide().flat_map(u16::to_le_bytes).collect::<Vec<_>>();
            format!("raw:{}", terrane_hex(&units))
        },
        |text| format!("text:{text}"),
    )
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
