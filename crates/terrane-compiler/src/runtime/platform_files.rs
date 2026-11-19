#[derive(Clone)]
pub struct TerranePlatformOpenResult {
    pub handle: TerranePlatformStreamHandle,
    pub failed: bool,
    pub message: String,
    pub error_kind: &'static str,
}
pub fn terrane_file_open_options(
    readable: bool,
    writable: bool,
    create: bool,
    truncate: bool,
    exclusive: bool,
    append: bool,
) -> std::io::Result<terrane_stream_abi::FileOpenOptions> {
    let access = match (readable, writable) {
        (true, false) => terrane_stream_abi::FileAccess::Read,
        (false, true) => terrane_stream_abi::FileAccess::Write,
        (true, true) => terrane_stream_abi::FileAccess::ReadWrite,
        (false, false) => {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "a file must be opened for reading, writing, or both",
            ));
        }
    };
    use terrane_stream_abi::FileCreation;
    let creation = match (create, truncate, exclusive) {
        (false, false, true) => FileCreation::Exclusive,
        (_, _, true) => {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "exclusive conflicts with create or truncate",
            ));
        }
        (false, false, false) => FileCreation::Existing,
        (true, false, false) => FileCreation::Create,
        (false, true, false) => FileCreation::Truncate,
        (true, true, false) => FileCreation::CreateOrTruncate,
    };
    Ok(terrane_stream_abi::FileOpenOptions { access, creation, append })
}

pub fn terrane_platform_open_result(
    result: std::io::Result<terrane_stream_abi::StreamHandle>,
) -> TerranePlatformOpenResult {
    match result {
        Ok(handle) => TerranePlatformOpenResult {
            handle: TerranePlatformStreamHandle::new(handle),
            failed: false,
            message: String::new(),
            error_kind: "none",
        },
        Err(error) => TerranePlatformOpenResult {
            handle: TerranePlatformStreamHandle::default(),
            failed: true,
            message: error.to_string(),
            error_kind: terrane_stream_abi::io_error_kind(error.kind()),
        },
    }
}

pub fn terrane_platform_open_file(
    path: String,
    readable: bool,
    writable: bool,
    create: bool,
    truncate: bool,
    exclusive: bool,
    append: bool,
) -> TerranePlatformOpenResult {
    terrane_platform_open_result(
        terrane_file_open_options(readable, writable, create, truncate, exclusive, append)
            .and_then(|request| terrane_stream_abi::open_file(&path, request)),
    )
}

pub fn terrane_platform_open_directory_beneath(
    base: String,
    child: String,
    cross_filesystem: bool,
) -> TerranePlatformOpenResult {
    terrane_platform_open_result(terrane_stream_abi::open_directory_beneath(
        &base,
        &child,
        cross_filesystem,
    ))
}

pub fn terrane_platform_open_file_beneath(
    directory: &TerranePlatformStreamHandle,
    child: String,
    readable: bool,
    writable: bool,
    create: bool,
    truncate: bool,
    exclusive: bool,
    append: bool,
) -> TerranePlatformOpenResult {
    terrane_platform_open_result(
        terrane_file_open_options(readable, writable, create, truncate, exclusive, append)
            .and_then(|request| {
                terrane_stream_abi::open_file_beneath(directory.abi_handle(), &child, request)
            }),
    )
}
