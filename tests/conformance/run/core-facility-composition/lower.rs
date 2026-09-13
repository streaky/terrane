// Generated deterministically by Terrane <version>.
// Runtime support: async.rs, platform_streams.rs, platform_standard_streams.rs, platform_files.rs, platform_system.rs
// Vendored support crates: terrane-int-support, terrane-collection-support, terrane-scalar-support, terrane-string-support, terrane-stream-abi
// Source: case.trn
// Namespace: core-facility-composition
fn main() {
    let filesystem_result: FilesystemOperationResult = FilesystemOperationResult::terrane_construct(
        false,
        String::from(""),
    );
    let stream_result: StreamOperationResult = StreamOperationResult::terrane_construct(
        false,
        String::from(""),
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&(! filesystem_result.failed &&!
        stream_result.failed))
    );
}
// Source: core/filesystem.trn
// Namespace: core/filesystem
#[derive(Clone)]
pub struct FilesystemOperationResult {
    pub failed: bool,
    pub message: String,
}
impl FilesystemOperationResult {
    pub fn terrane_construct(failure: bool, detail: String) -> Self {
        let mut value = Self {
            failed: false,
            message: String::from(""),
        };
        value.construct(failure, detail);
        value
    }
    pub fn construct(&mut self, failure: bool, detail: String) {
        self.failed = failure;
        self.message = detail;
    }
}
#[derive(Clone)]
pub struct ExistenceResult {
    pub exists: bool,
    pub failed: bool,
    pub message: String,
}
impl ExistenceResult {
    pub fn terrane_construct(exists: bool, failure: bool, detail: String) -> Self {
        let mut value = Self {
            exists: false,
            failed: false,
            message: String::from(""),
        };
        value.construct(exists, failure, detail);
        value
    }
    pub fn construct(&mut self, exists: bool, failure: bool, detail: String) {
        self.exists = exists;
        self.failed = failure;
        self.message = detail;
    }
}
#[derive(Clone)]
pub struct PathResult {
    pub resolved: Path,
    pub failed: bool,
    pub message: String,
}
impl PathResult {
    pub fn terrane_construct(target: Path, failure: bool, detail: String) -> Self {
        let mut value = Self {
            resolved: Path::terrane_construct(String::from("")),
            failed: false,
            message: String::from(""),
        };
        value.construct(target, failure, detail);
        value
    }
    pub fn construct(&mut self, target: Path, failure: bool, detail: String) {
        self.resolved = target.clone();
        self.failed = failure;
        self.message = detail;
    }
}
#[derive(Clone)]
pub struct FileMetadata {
    pub kind: String,
    pub size: terrane_int_support::Int,
    pub readonly: bool,
    pub permission_detail: String,
    pub failed: bool,
    pub message: String,
}
impl FileMetadata {
    pub fn terrane_construct(
        kind: String,
        size: terrane_int_support::Int,
        readonly: bool,
        permission_detail: String,
        failure: bool,
        detail: String,
    ) -> Self {
        let mut value = Self {
            kind: String::from("other"),
            size: terrane_int_support::Int::from(0_i128),
            readonly: false,
            permission_detail: String::from(""),
            failed: false,
            message: String::from(""),
        };
        value.construct(kind, size, readonly, permission_detail, failure, detail);
        value
    }
    pub fn construct(
        &mut self,
        kind: String,
        size: terrane_int_support::Int,
        readonly: bool,
        permission_detail: String,
        failure: bool,
        detail: String,
    ) {
        self.kind = kind;
        self.size = size.clone();
        self.readonly = readonly;
        self.permission_detail = permission_detail;
        self.failed = failure;
        self.message = detail;
    }
}
#[derive(Clone)]
pub struct FileData {
    pub data: Vec<u8>,
    pub completed: terrane_int_support::Int,
    pub end: bool,
    pub failed: bool,
    pub message: String,
}
impl FileData {
    pub fn terrane_construct(
        data: Vec<u8>,
        completed: terrane_int_support::Int,
        end: bool,
        failure: bool,
        detail: String,
    ) -> Self {
        let mut value = Self {
            data: Vec::from([]),
            completed: terrane_int_support::Int::from(0_i128),
            end: false,
            failed: false,
            message: String::from(""),
        };
        value.construct(data, completed, end, failure, detail);
        value
    }
    pub fn construct(
        &mut self,
        data: Vec<u8>,
        completed: terrane_int_support::Int,
        end: bool,
        failure: bool,
        detail: String,
    ) {
        self.data = data;
        self.completed = completed.clone();
        self.end = end;
        self.failed = failure;
        self.message = detail;
    }
}
pub struct FileHandle {
    pub handle: TerranePlatformStreamHandle,
    pub failed: bool,
    pub message: String,
}
impl FileHandle {
    pub fn terrane_construct(
        raw: TerranePlatformStreamHandle,
        failure: bool,
        detail: String,
    ) -> Self {
        let mut value = Self {
            handle: Default::default(),
            failed: false,
            message: String::from(""),
        };
        value.construct(raw, failure, detail);
        value
    }
    pub fn construct(
        &mut self,
        raw: TerranePlatformStreamHandle,
        failure: bool,
        detail: String,
    ) {
        self.handle = raw;
        self.failed = failure;
        self.message = detail;
    }
    pub fn destruct(&mut self) {
        terrane_platform_release(&self.handle);
    }
}
impl Drop for FileHandle {
    fn drop(&mut self) {
        self.destruct();
    }
}
pub struct DirectoryHandle {
    pub handle: TerranePlatformStreamHandle,
    pub failed: bool,
    pub message: String,
}
impl DirectoryHandle {
    pub fn terrane_construct(
        raw: TerranePlatformStreamHandle,
        failure: bool,
        detail: String,
    ) -> Self {
        let mut value = Self {
            handle: Default::default(),
            failed: false,
            message: String::from(""),
        };
        value.construct(raw, failure, detail);
        value
    }
    pub fn construct(
        &mut self,
        raw: TerranePlatformStreamHandle,
        failure: bool,
        detail: String,
    ) {
        self.handle = raw;
        self.failed = failure;
        self.message = detail;
    }
    pub fn destruct(&mut self) {
        terrane_platform_release(&self.handle);
    }
}
impl Drop for DirectoryHandle {
    fn drop(&mut self) {
        self.destruct();
    }
}
pub fn open_file(
    capability: Filesystem,
    target: Path,
    readable: bool,
    writable: bool,
    create: bool,
    truncate: bool,
) -> FileHandle {
    let _ = &capability;
    let raw: TerranePlatformOpenResult = terrane_platform_open_file(
        target.text,
        readable,
        writable,
        create,
        truncate,
    );
    let failure: bool = raw.failed;
    let detail: String = raw.message.clone().clone();
    let acquired: TerranePlatformStreamHandle = raw.handle.clone().clone();
    return FileHandle::terrane_construct(acquired, failure, detail);
}
pub fn file_read(
    capability: Filesystem,
    file: &FileHandle,
    limit: terrane_int_support::Int,
) -> FileData {
    let _ = &capability;
    let raw: TerranePlatformReadResult = terrane_platform_read(
        &file.handle.clone(),
        limit,
    );
    return FileData::terrane_construct(
        raw.data.clone().clone(),
        raw.completed.clone(),
        raw.end,
        raw.failed,
        raw.message.clone().clone(),
    );
}
pub fn file_write(
    capability: Filesystem,
    file: &FileHandle,
    data: Vec<u8>,
    offset: terrane_int_support::Int,
) -> FileData {
    let _ = &capability;
    let raw: TerranePlatformWriteResult = terrane_platform_write(
        &file.handle.clone(),
        &data,
        terrane_int_support::Int::from(offset.clone()),
    );
    return FileData::terrane_construct(
        data,
        raw.completed.clone(),
        false,
        raw.failed,
        raw.message.clone().clone(),
    );
}
pub fn file_flush(
    capability: Filesystem,
    file: &FileHandle,
) -> FilesystemOperationResult {
    let _ = &capability;
    let raw: TerranePlatformUnitResult = terrane_platform_flush(&file.handle.clone());
    return FilesystemOperationResult::terrane_construct(
        raw.failed,
        raw.message.clone().clone(),
    );
}
pub fn file_sync_data(
    capability: Filesystem,
    file: &FileHandle,
) -> FilesystemOperationResult {
    let _ = &capability;
    let raw: TerranePlatformUnitResult = terrane_platform_sync_data(
        &file.handle.clone(),
    );
    return FilesystemOperationResult::terrane_construct(
        raw.failed,
        raw.message.clone().clone(),
    );
}
pub fn file_sync_all(
    capability: Filesystem,
    file: &FileHandle,
) -> FilesystemOperationResult {
    let _ = &capability;
    let raw: TerranePlatformUnitResult = terrane_platform_sync_all(&file.handle.clone());
    return FilesystemOperationResult::terrane_construct(
        raw.failed,
        raw.message.clone().clone(),
    );
}
pub fn file_close(
    capability: Filesystem,
    file: FileHandle,
) -> FilesystemOperationResult {
    let _ = &capability;
    let raw: TerranePlatformUnitResult = terrane_platform_close(&file.handle);
    return FilesystemOperationResult::terrane_construct(
        raw.failed,
        raw.message.clone().clone(),
    );
}
#[derive(Clone)]
pub struct Filesystem {
    pub authority: TerraneFilesystemAuthority,
}
impl Filesystem {
    pub fn terrane_construct(authority: TerraneFilesystemAuthority) -> Self {
        let mut value = Self {
            authority: Default::default(),
        };
        value.construct(authority);
        value
    }
    pub fn construct(&mut self, authority: TerraneFilesystemAuthority) {
        self.authority = authority;
    }
}
pub fn filesystem_capability() -> Filesystem {
    let authority: TerraneFilesystemAuthority = terrane_acquire_filesystem_authority();
    return Filesystem::terrane_construct(authority);
}
pub fn filesystem_exists(capability: Filesystem, target: Path) -> ExistenceResult {
    let _ = &capability;
    let record: TerraneFilesystemResult = terrane_filesystem_exists(target.text);
    return ExistenceResult::terrane_construct(
        terrane_filesystem_result_bool(&record),
        terrane_filesystem_result_failed(&record),
        terrane_filesystem_result_message(&record),
    );
}
pub fn filesystem_metadata(capability: Filesystem, target: Path) -> FileMetadata {
    let _ = &capability;
    let record: TerraneFilesystemResult = terrane_filesystem_metadata(target.text, true);
    return FileMetadata::terrane_construct(
        terrane_filesystem_result_text(&record),
        terrane_filesystem_result_int(&record),
        terrane_filesystem_result_bool(&record),
        terrane_filesystem_result_detail(&record),
        terrane_filesystem_result_failed(&record),
        terrane_filesystem_result_message(&record),
    );
}
pub fn filesystem_symlink_metadata(
    capability: Filesystem,
    target: Path,
) -> FileMetadata {
    let _ = &capability;
    let record: TerraneFilesystemResult = terrane_filesystem_metadata(
        target.text,
        false,
    );
    return FileMetadata::terrane_construct(
        terrane_filesystem_result_text(&record),
        terrane_filesystem_result_int(&record),
        terrane_filesystem_result_bool(&record),
        terrane_filesystem_result_detail(&record),
        terrane_filesystem_result_failed(&record),
        terrane_filesystem_result_message(&record),
    );
}
pub fn filesystem_canonical(capability: Filesystem, target: Path) -> PathResult {
    let _ = &capability;
    let record: TerraneFilesystemResult = terrane_filesystem_realpath(target.text);
    let resolved: Path = Path::terrane_construct(
        terrane_filesystem_result_text(&record),
    );
    return PathResult::terrane_construct(
        resolved,
        terrane_filesystem_result_failed(&record),
        terrane_filesystem_result_message(&record),
    );
}
pub fn filesystem_realpath(capability: Filesystem, target: Path) -> PathResult {
    return filesystem_canonical(capability.clone(), target.clone());
}
pub fn filesystem_read_link(capability: Filesystem, target: Path) -> PathResult {
    let _ = &capability;
    let record: TerraneFilesystemResult = terrane_filesystem_read_link(target.text);
    let linked: Path = Path::terrane_construct(terrane_filesystem_result_text(&record));
    return PathResult::terrane_construct(
        linked,
        terrane_filesystem_result_failed(&record),
        terrane_filesystem_result_message(&record),
    );
}
pub fn filesystem_open_beneath(
    capability: Filesystem,
    directory: Path,
    relative: Path,
    cross_filesystem: bool,
) -> DirectoryHandle {
    let _ = &capability;
    let raw: TerranePlatformOpenResult = terrane_platform_open_directory_beneath(
        directory.text,
        relative.text,
        cross_filesystem,
    );
    let failure: bool = raw.failed;
    let detail: String = raw.message.clone().clone();
    let acquired: TerranePlatformStreamHandle = raw.handle.clone().clone();
    return DirectoryHandle::terrane_construct(acquired, failure, detail);
}
pub fn open_file_beneath(
    capability: Filesystem,
    directory: &DirectoryHandle,
    relative: Path,
    readable: bool,
    writable: bool,
    create: bool,
    truncate: bool,
) -> FileHandle {
    let _ = &capability;
    let raw: TerranePlatformOpenResult = terrane_platform_open_file_beneath(
        &directory.handle.clone(),
        relative.text,
        readable,
        writable,
        create,
        truncate,
    );
    let failure: bool = raw.failed;
    let detail: String = raw.message.clone().clone();
    let acquired: TerranePlatformStreamHandle = raw.handle.clone().clone();
    return FileHandle::terrane_construct(acquired, failure, detail);
}
pub fn filesystem_read_bounded(
    capability: Filesystem,
    target: Path,
    limit: terrane_int_support::Int,
) -> FileData {
    let _ = &capability;
    let record: TerraneFilesystemResult = terrane_filesystem_read_bounded(
        target.text,
        limit,
    );
    return FileData::terrane_construct(
        terrane_filesystem_result_bytes(&record),
        terrane_filesystem_result_int(&record),
        true,
        terrane_filesystem_result_failed(&record),
        terrane_filesystem_result_message(&record),
    );
}
pub fn filesystem_write_atomic(
    capability: Filesystem,
    target: Path,
    data: Vec<u8>,
) -> FilesystemOperationResult {
    let _ = &capability;
    let record: TerraneFilesystemResult = terrane_filesystem_write_atomic(
        target.text,
        data,
    );
    return FilesystemOperationResult::terrane_construct(
        terrane_filesystem_result_failed(&record),
        terrane_filesystem_result_message(&record),
    );
}
pub fn filesystem_rename(
    capability: Filesystem,
    source: Path,
    destination: Path,
) -> FilesystemOperationResult {
    let _ = &capability;
    let record: TerraneFilesystemResult = terrane_filesystem_rename(
        source.text,
        destination.text,
    );
    return FilesystemOperationResult::terrane_construct(
        terrane_filesystem_result_failed(&record),
        terrane_filesystem_result_message(&record),
    );
}
pub fn filesystem_remove(
    capability: Filesystem,
    target: Path,
) -> FilesystemOperationResult {
    let _ = &capability;
    let record: TerraneFilesystemResult = terrane_filesystem_remove(target.text);
    return FilesystemOperationResult::terrane_construct(
        terrane_filesystem_result_failed(&record),
        terrane_filesystem_result_message(&record),
    );
}
// Source: core/streams.trn
// Namespace: core/streams
#[derive(Clone)]
pub struct StreamOperationResult {
    pub failed: bool,
    pub message: String,
}
impl StreamOperationResult {
    pub fn terrane_construct(failed: bool, message: String) -> Self {
        let mut value = Self {
            failed: false,
            message: String::from(""),
        };
        value.construct(failed, message);
        value
    }
    pub fn construct(&mut self, failed: bool, message: String) {
        self.failed = failed;
        self.message = message;
    }
}
#[derive(Clone)]
pub struct ReadResult {
    pub data: Vec<u8>,
    pub completed: terrane_int_support::Int,
    pub end: bool,
    pub failed: bool,
    pub message: String,
}
impl ReadResult {
    pub fn terrane_construct(
        data: Vec<u8>,
        completed: terrane_int_support::Int,
        end: bool,
        failed: bool,
        message: String,
    ) -> Self {
        let mut value = Self {
            data: Vec::from([]),
            completed: terrane_int_support::Int::from(0_i128),
            end: false,
            failed: false,
            message: String::from(""),
        };
        value.construct(data, completed, end, failed, message);
        value
    }
    pub fn construct(
        &mut self,
        data: Vec<u8>,
        completed: terrane_int_support::Int,
        end: bool,
        failed: bool,
        message: String,
    ) {
        self.data = data;
        self.completed = completed.clone();
        self.end = end;
        self.failed = failed;
        self.message = message;
    }
}
#[derive(Clone)]
pub struct TextReadResult {
    pub text: String,
    pub completed: terrane_int_support::Int,
    pub end: bool,
    pub failed: bool,
    pub message: String,
}
impl TextReadResult {
    pub fn terrane_construct(
        text: String,
        completed: terrane_int_support::Int,
        end: bool,
        failed: bool,
        message: String,
    ) -> Self {
        let mut value = Self {
            text: String::from(""),
            completed: terrane_int_support::Int::from(0_i128),
            end: false,
            failed: false,
            message: String::from(""),
        };
        value.construct(text, completed, end, failed, message);
        value
    }
    pub fn construct(
        &mut self,
        text: String,
        completed: terrane_int_support::Int,
        end: bool,
        failed: bool,
        message: String,
    ) {
        self.text = text;
        self.completed = completed.clone();
        self.end = end;
        self.failed = failed;
        self.message = message;
    }
}
#[derive(Clone)]
pub struct WriteResult {
    pub data: Vec<u8>,
    pub completed: terrane_int_support::Int,
    pub failed: bool,
    pub message: String,
}
impl WriteResult {
    pub fn terrane_construct(
        data: Vec<u8>,
        completed: terrane_int_support::Int,
        failed: bool,
        message: String,
    ) -> Self {
        let mut value = Self {
            data: Vec::from([]),
            completed: terrane_int_support::Int::from(0_i128),
            failed: false,
            message: String::from(""),
        };
        value.construct(data, completed, failed, message);
        value
    }
    pub fn construct(
        &mut self,
        data: Vec<u8>,
        completed: terrane_int_support::Int,
        failed: bool,
        message: String,
    ) {
        if completed.clone() == terrane_int_support::Int::from(data.len() as i128)
            && !failed
        {
            self.data = Vec::from([]);
        } else {
            self.data = data;
        }
        self.completed = completed.clone();
        self.failed = failed;
        self.message = message;
    }
}
pub struct ByteReader {
    pub handle: TerranePlatformStreamHandle,
}
impl ByteReader {
    pub fn terrane_construct(handle: TerranePlatformStreamHandle) -> Self {
        let mut value = Self { handle: Default::default() };
        value.construct(handle);
        value
    }
    pub fn construct(&mut self, handle: TerranePlatformStreamHandle) {
        self.handle = handle;
    }
    pub fn read(&self, count: terrane_int_support::Int) -> ReadResult {
        let raw: TerranePlatformReadResult = terrane_platform_read(&self.handle, count);
        return ReadResult::terrane_construct(
            raw.data.clone().clone(),
            raw.completed.clone(),
            raw.end,
            raw.failed,
            raw.message.clone().clone(),
        );
    }
    pub fn read_exact(&self, count: terrane_int_support::Int) -> ReadResult {
        let mut data: Vec<u8> = Vec::from([]);
        let mut completed: terrane_int_support::Int = terrane_int_support::Int::from(
            0_i128,
        );
        let mut end: bool = false;
        let mut failed: bool = false;
        let mut message: String = String::from("");
        while completed.clone() < count.clone() && !end && !failed {
            let part: TerranePlatformReadResult = terrane_platform_read(
                &self.handle,
                count.clone() - completed.clone(),
            );
            data = {
                let mut bytes = data;
                bytes.extend(part.data.clone());
                bytes
            };
            completed = completed.clone() + part.completed.clone();
            end = part.end;
            failed = part.failed;
            message = part.message.clone().clone();
            if part.completed.clone() == terrane_int_support::Int::from(0_i128)
                && !part.end && !part.failed
            {
                failed = true;
                message = String::from("stream read made no progress");
            }
        }
        if end && completed.clone() < count.clone() && !failed {
            failed = true;
            message = String::from("stream ended before exact byte count");
        }
        return ReadResult::terrane_construct(
            data,
            completed.clone(),
            end,
            failed,
            message,
        );
    }
    pub fn read_all(&self, limit: terrane_int_support::Int) -> ReadResult {
        let mut data: Vec<u8> = Vec::from([]);
        let mut completed: terrane_int_support::Int = terrane_int_support::Int::from(
            0_i128,
        );
        let mut end: bool = false;
        let mut failed: bool = false;
        let mut message: String = String::from("");
        while completed.clone() < limit.clone() && !end && !failed {
            let part: TerranePlatformReadResult = terrane_platform_read(
                &self.handle,
                limit.clone() - completed.clone(),
            );
            data = {
                let mut bytes = data;
                bytes.extend(part.data.clone());
                bytes
            };
            completed = completed.clone() + part.completed.clone();
            end = part.end;
            failed = part.failed;
            message = part.message.clone().clone();
            if part.completed.clone() == terrane_int_support::Int::from(0_i128)
                && !part.end && !part.failed
            {
                failed = true;
                message = String::from("stream read made no progress");
            }
        }
        return ReadResult::terrane_construct(
            data,
            completed.clone(),
            end,
            failed,
            message,
        );
    }
    pub async fn read_async(&self, count: terrane_int_support::Int) -> ReadResult {
        let raw: TerranePlatformReadResult = __terrane_await(
                terrane_platform_read_async(&self.handle, count),
            )
            .await;
        return ReadResult::terrane_construct(
            raw.data.clone().clone(),
            raw.completed.clone(),
            raw.end,
            raw.failed,
            raw.message.clone().clone(),
        );
    }
    pub fn text(&self, codec: terrane_string_support::Encoding) -> TextReader {
        return TextReader::terrane_construct(self.handle.clone(), codec);
    }
    pub fn close(self) -> StreamOperationResult {
        let raw: TerranePlatformUnitResult = terrane_platform_close(&self.handle);
        return StreamOperationResult::terrane_construct(
            raw.failed,
            raw.message.clone().clone(),
        );
    }
    pub fn destruct(&mut self) {
        terrane_platform_release(&self.handle);
    }
}
impl Drop for ByteReader {
    fn drop(&mut self) {
        self.destruct();
    }
}
pub struct ByteWriter {
    pub handle: TerranePlatformStreamHandle,
}
impl ByteWriter {
    pub fn terrane_construct(handle: TerranePlatformStreamHandle) -> Self {
        let mut value = Self { handle: Default::default() };
        value.construct(handle);
        value
    }
    pub fn construct(&mut self, handle: TerranePlatformStreamHandle) {
        self.handle = handle;
    }
    pub fn write(&self, data: Vec<u8>) -> WriteResult {
        let offset: i64 = 0;
        let raw: TerranePlatformWriteResult = terrane_platform_write(
            &self.handle,
            &data,
            terrane_int_support::Int::from(offset.clone()),
        );
        return WriteResult::terrane_construct(
            data,
            raw.completed.clone(),
            raw.failed,
            raw.message.clone().clone(),
        );
    }
    pub fn write_all(&self, data: Vec<u8>) -> WriteResult {
        let mut completed: terrane_int_support::Int = terrane_int_support::Int::from(
            0_i128,
        );
        let mut failed: bool = false;
        let mut message: String = String::from("");
        while completed.clone() < terrane_int_support::Int::from(data.len() as i128)
            && !failed
        {
            let part: TerranePlatformWriteResult = terrane_platform_write(
                &self.handle,
                &data,
                terrane_int_support::Int::from(completed.clone()),
            );
            completed = completed.clone() + part.completed.clone();
            failed = part.failed;
            message = part.message.clone().clone();
            if part.completed.clone() == terrane_int_support::Int::from(0_i128)
                && !part.failed
            {
                failed = true;
                message = String::from("stream write made no progress");
            }
        }
        return WriteResult::terrane_construct(data, completed.clone(), failed, message);
    }
    pub fn resume(&self, prior: WriteResult) -> WriteResult {
        if terrane_int_support::Int::from(prior.data.len() as i128)
            == terrane_int_support::Int::from(0_i128)
        {
            return prior.clone();
        }
        let raw: TerranePlatformWriteResult = terrane_platform_write(
            &self.handle,
            &prior.data,
            terrane_int_support::Int::from(prior.completed.clone()),
        );
        return WriteResult::terrane_construct(
            prior.data.clone(),
            prior.completed.clone() + raw.completed.clone(),
            raw.failed,
            raw.message.clone().clone(),
        );
    }
    pub async fn write_async(&self, data: Vec<u8>) -> WriteResult {
        return self.write(data);
    }
    pub fn text(&self, codec: terrane_string_support::Encoding) -> TextWriter {
        return TextWriter::terrane_construct(self.handle.clone(), codec);
    }
    pub fn flush(&self) -> StreamOperationResult {
        let raw: TerranePlatformUnitResult = terrane_platform_flush(&self.handle);
        return StreamOperationResult::terrane_construct(
            raw.failed,
            raw.message.clone().clone(),
        );
    }
    pub fn sync_data(&self) -> StreamOperationResult {
        let raw: TerranePlatformUnitResult = terrane_platform_sync_data(&self.handle);
        return StreamOperationResult::terrane_construct(
            raw.failed,
            raw.message.clone().clone(),
        );
    }
    pub fn sync_all(&self) -> StreamOperationResult {
        let raw: TerranePlatformUnitResult = terrane_platform_sync_all(&self.handle);
        return StreamOperationResult::terrane_construct(
            raw.failed,
            raw.message.clone().clone(),
        );
    }
    pub fn close(self) -> StreamOperationResult {
        let raw: TerranePlatformUnitResult = terrane_platform_close(&self.handle);
        return StreamOperationResult::terrane_construct(
            raw.failed,
            raw.message.clone().clone(),
        );
    }
    pub fn destruct(&mut self) {
        terrane_platform_release(&self.handle);
    }
}
impl Drop for ByteWriter {
    fn drop(&mut self) {
        self.destruct();
    }
}
pub struct TextReader {
    pub handle: TerranePlatformStreamHandle,
    pub codec: terrane_string_support::Encoding,
}
impl TextReader {
    pub fn terrane_construct(
        handle: TerranePlatformStreamHandle,
        codec: terrane_string_support::Encoding,
    ) -> Self {
        let mut value = Self {
            handle: Default::default(),
            codec: terrane_string_support::Encoding::Utf8,
        };
        value.construct(handle, codec);
        value
    }
    pub fn construct(
        &mut self,
        handle: TerranePlatformStreamHandle,
        codec: terrane_string_support::Encoding,
    ) {
        self.handle = handle;
        self.codec = codec;
    }
    pub fn read(
        &self,
        count: terrane_int_support::Int,
    ) -> Result<TextReadResult, TerraneError> {
        let raw: TerranePlatformReadResult = terrane_platform_read(&self.handle, count);
        let text: String = __terrane_raised_err(
            terrane_string_support::decode(&raw.data.clone(), self.codec),
            0 /* terrane-site: core/streams.trn:188:23-188:50 */,
        )?;
        return Ok(
            TextReadResult::terrane_construct(
                text,
                raw.completed.clone(),
                raw.end,
                raw.failed,
                raw.message.clone().clone(),
            ),
        );
    }
    pub fn read_exact(
        &self,
        count: terrane_int_support::Int,
    ) -> Result<TextReadResult, TerraneError> {
        let mut data: Vec<u8> = Vec::from([]);
        let mut completed: terrane_int_support::Int = terrane_int_support::Int::from(
            0_i128,
        );
        let mut end: bool = false;
        let mut failed: bool = false;
        let mut message: String = String::from("");
        while completed.clone() < count.clone() && !end && !failed {
            let part: TerranePlatformReadResult = terrane_platform_read(
                &self.handle,
                count.clone() - completed.clone(),
            );
            data = {
                let mut bytes = data;
                bytes.extend(part.data.clone());
                bytes
            };
            completed = completed.clone() + part.completed.clone();
            end = part.end;
            failed = part.failed;
            message = part.message.clone().clone();
            if part.completed.clone() == terrane_int_support::Int::from(0_i128)
                && !part.end && !part.failed
            {
                failed = true;
                message = String::from("stream read made no progress");
            }
        }
        if end && completed.clone() < count.clone() && !failed {
            failed = true;
            message = String::from("stream ended before exact byte count");
        }
        let text: String = __terrane_raised_err(
            terrane_string_support::decode(&data, self.codec),
            1 /* terrane-site: core/streams.trn:210:23-210:46 */,
        )?;
        return Ok(
            TextReadResult::terrane_construct(
                text,
                completed.clone(),
                end,
                failed,
                message,
            ),
        );
    }
    pub fn read_all(
        &self,
        limit: terrane_int_support::Int,
    ) -> Result<TextReadResult, TerraneError> {
        let mut data: Vec<u8> = Vec::from([]);
        let mut completed: terrane_int_support::Int = terrane_int_support::Int::from(
            0_i128,
        );
        let mut end: bool = false;
        let mut failed: bool = false;
        let mut message: String = String::from("");
        while completed.clone() < limit.clone() && !end && !failed {
            let part: TerranePlatformReadResult = terrane_platform_read(
                &self.handle,
                limit.clone() - completed.clone(),
            );
            data = {
                let mut bytes = data;
                bytes.extend(part.data.clone());
                bytes
            };
            completed = completed.clone() + part.completed.clone();
            end = part.end;
            failed = part.failed;
            message = part.message.clone().clone();
            if part.completed.clone() == terrane_int_support::Int::from(0_i128)
                && !part.end && !part.failed
            {
                failed = true;
                message = String::from("stream read made no progress");
            }
        }
        let text: String = __terrane_raised_err(
            terrane_string_support::decode(&data, self.codec),
            2 /* terrane-site: core/streams.trn:229:23-229:46 */,
        )?;
        return Ok(
            TextReadResult::terrane_construct(
                text,
                completed.clone(),
                end,
                failed,
                message,
            ),
        );
    }
    pub async fn read_async(
        &self,
        count: terrane_int_support::Int,
    ) -> Result<TextReadResult, TerraneError> {
        let raw: TerranePlatformReadResult = __terrane_await(
                terrane_platform_read_async(&self.handle, count),
            )
            .await;
        let text: String = __terrane_raised_err(
            terrane_string_support::decode(&raw.data.clone(), self.codec),
            3 /* terrane-site: core/streams.trn:234:23-234:50 */,
        )?;
        return Ok(
            TextReadResult::terrane_construct(
                text,
                raw.completed.clone(),
                raw.end,
                raw.failed,
                raw.message.clone().clone(),
            ),
        );
    }
    pub fn close(self) -> StreamOperationResult {
        let raw: TerranePlatformUnitResult = terrane_platform_close(&self.handle);
        return StreamOperationResult::terrane_construct(
            raw.failed,
            raw.message.clone().clone(),
        );
    }
    pub fn destruct(&mut self) {
        terrane_platform_release(&self.handle);
    }
}
impl Drop for TextReader {
    fn drop(&mut self) {
        self.destruct();
    }
}
pub struct TextWriter {
    pub handle: TerranePlatformStreamHandle,
    pub codec: terrane_string_support::Encoding,
}
impl TextWriter {
    pub fn terrane_construct(
        handle: TerranePlatformStreamHandle,
        codec: terrane_string_support::Encoding,
    ) -> Self {
        let mut value = Self {
            handle: Default::default(),
            codec: terrane_string_support::Encoding::Utf8,
        };
        value.construct(handle, codec);
        value
    }
    pub fn construct(
        &mut self,
        handle: TerranePlatformStreamHandle,
        codec: terrane_string_support::Encoding,
    ) {
        self.handle = handle;
        self.codec = codec;
    }
    pub fn write(&self, text: String) -> WriteResult {
        let data: Vec<u8> = terrane_string_support::encode(&text, self.codec);
        let offset: i64 = 0;
        let raw: TerranePlatformWriteResult = terrane_platform_write(
            &self.handle,
            &data,
            terrane_int_support::Int::from(offset.clone()),
        );
        return WriteResult::terrane_construct(
            data,
            raw.completed.clone(),
            raw.failed,
            raw.message.clone().clone(),
        );
    }
    pub fn write_all(&self, text: String) -> WriteResult {
        let data: Vec<u8> = terrane_string_support::encode(&text, self.codec);
        let mut completed: terrane_int_support::Int = terrane_int_support::Int::from(
            0_i128,
        );
        let mut failed: bool = false;
        let mut message: String = String::from("");
        while completed.clone() < terrane_int_support::Int::from(data.len() as i128)
            && !failed
        {
            let part: TerranePlatformWriteResult = terrane_platform_write(
                &self.handle,
                &data,
                terrane_int_support::Int::from(completed.clone()),
            );
            completed = completed.clone() + part.completed.clone();
            failed = part.failed;
            message = part.message.clone().clone();
            if part.completed.clone() == terrane_int_support::Int::from(0_i128)
                && !part.failed
            {
                failed = true;
                message = String::from("stream write made no progress");
            }
        }
        return WriteResult::terrane_construct(data, completed.clone(), failed, message);
    }
    pub fn resume(&self, prior: WriteResult) -> WriteResult {
        if terrane_int_support::Int::from(prior.data.len() as i128)
            == terrane_int_support::Int::from(0_i128)
        {
            return prior.clone();
        }
        let raw: TerranePlatformWriteResult = terrane_platform_write(
            &self.handle,
            &prior.data,
            terrane_int_support::Int::from(prior.completed.clone()),
        );
        return WriteResult::terrane_construct(
            prior.data.clone(),
            prior.completed.clone() + raw.completed.clone(),
            raw.failed,
            raw.message.clone().clone(),
        );
    }
    pub fn line(&self, text: String) -> WriteResult {
        return self
            .write_all(
                format!(
                    "{}{}", terrane_scalar_support::scalar_text(&text),
                    terrane_scalar_support::scalar_text(&String::from("\n"))
                ),
            );
    }
    pub async fn write_async(&self, text: String) -> WriteResult {
        return self.write(text);
    }
    pub fn flush(&self) -> StreamOperationResult {
        let raw: TerranePlatformUnitResult = terrane_platform_flush(&self.handle);
        return StreamOperationResult::terrane_construct(
            raw.failed,
            raw.message.clone().clone(),
        );
    }
    pub fn sync_data(&self) -> StreamOperationResult {
        let raw: TerranePlatformUnitResult = terrane_platform_sync_data(&self.handle);
        return StreamOperationResult::terrane_construct(
            raw.failed,
            raw.message.clone().clone(),
        );
    }
    pub fn sync_all(&self) -> StreamOperationResult {
        let raw: TerranePlatformUnitResult = terrane_platform_sync_all(&self.handle);
        return StreamOperationResult::terrane_construct(
            raw.failed,
            raw.message.clone().clone(),
        );
    }
    pub fn close(self) -> StreamOperationResult {
        let raw: TerranePlatformUnitResult = terrane_platform_close(&self.handle);
        return StreamOperationResult::terrane_construct(
            raw.failed,
            raw.message.clone().clone(),
        );
    }
    pub fn destruct(&mut self) {
        terrane_platform_release(&self.handle);
    }
}
impl Drop for TextWriter {
    fn drop(&mut self) {
        self.destruct();
    }
}
pub fn stdin() -> ByteReader {
    return ByteReader::terrane_construct(terrane_platform_acquire_stdin());
}
pub fn stdout() -> ByteWriter {
    return ByteWriter::terrane_construct(terrane_platform_acquire_stdout());
}
pub fn stderr() -> ByteWriter {
    return ByteWriter::terrane_construct(terrane_platform_acquire_stderr());
}
// Source: core/paths.trn
// Namespace: core/filesystem/paths
#[derive(Clone)]
pub struct Path {
    pub text: String,
}
impl Path {
    pub fn terrane_construct(input: String) -> Self {
        let mut value = Self { text: String::from("") };
        value.construct(input);
        value
    }
    pub fn construct(&mut self, input: String) {
        self.text = input;
    }
}
pub fn path_components(subject: Path) -> terrane_collection_support::List<String> {
    let parts: Vec<String> = terrane_string_support::split(
        &subject.text,
        &String::from("/"),
    );
    let mut result: terrane_collection_support::List<String> = terrane_collection_support::List::<
        String,
    >::new(vec![]);
    let mut index: terrane_int_support::Int = terrane_int_support::Int::from(0_i128);
    {
        let __terrane_list_append_0 = result.make_unique();
        while index.clone() < terrane_int_support::Int::from(parts.len() as i128) {
            let part: String = __terrane_raised(
                parts
                    .get(
                        __terrane_raised(
                            terrane_collection_support::index_from_int(&index.clone()),
                            4 /* terrane-site: core/paths.trn:16:16-16:28 */,
                        ),
                    )
                    .cloned()
                    .ok_or_else(|| terrane_collection_support::IndexError::from_usize(
                        __terrane_raised(
                            terrane_collection_support::index_from_int(&index.clone()),
                            4 /* terrane-site: core/paths.trn:16:16-16:28 */,
                        ),
                    )),
                4 /* terrane-site: core/paths.trn:16:16-16:28 */,
            );
            if part != String::from("") {
                __terrane_list_append_0.push(part);
            }
            index = index.clone() + terrane_int_support::Int::from(1_i128);
        }
    }
    return result.clone();
}
pub fn path_is_absolute(subject: Path) -> bool {
    return subject.text.starts_with(&String::from("/"));
}
pub fn normalise_path(subject: Path) -> Path {
    let parts: Vec<String> = terrane_string_support::split(
        &subject.text,
        &String::from("/"),
    );
    let absolute: bool = path_is_absolute(subject.clone());
    let mut kept: terrane_collection_support::List<String> = terrane_collection_support::List::<
        String,
    >::new(vec![]);
    let mut count: terrane_int_support::Int = terrane_int_support::Int::from(0_i128);
    let mut part_index: terrane_int_support::Int = terrane_int_support::Int::from(
        0_i128,
    );
    while part_index.clone() < terrane_int_support::Int::from(parts.len() as i128) {
        let part: String = __terrane_raised(
            parts
                .get(
                    __terrane_raised(
                        terrane_collection_support::index_from_int(&part_index.clone()),
                        5 /* terrane-site: core/paths.trn:32:16-32:33 */,
                    ),
                )
                .cloned()
                .ok_or_else(|| terrane_collection_support::IndexError::from_usize(
                    __terrane_raised(
                        terrane_collection_support::index_from_int(&part_index.clone()),
                        5 /* terrane-site: core/paths.trn:32:16-32:33 */,
                    ),
                )),
            5 /* terrane-site: core/paths.trn:32:16-32:33 */,
        );
        if part != String::from("") && part != String::from(".") {
            if part == String::from("..") {
                if count.clone() > terrane_int_support::Int::from(0_i128)
                    && __terrane_raised(
                        kept
                            .get_or_error(
                                __terrane_raised(
                                    terrane_collection_support::index_from_int(
                                        &(count.clone() - terrane_int_support::Int::from(1_i128)),
                                    ),
                                    6 /* terrane-site: core/paths.trn:35:34-35:49 */,
                                ),
                            ),
                        6 /* terrane-site: core/paths.trn:35:34-35:49 */,
                    ) != String::from("..")
                {
                    count = count.clone() - terrane_int_support::Int::from(1_i128);
                } else {
                    if !absolute {
                        if count.clone()
                            < terrane_int_support::Int::from(
                                terrane_int_support::Int::from(kept.length()),
                            )
                        {
                            __terrane_raised(
                                kept
                                    .set(
                                        __terrane_raised(
                                            terrane_collection_support::index_from_int(&count.clone()),
                                            7 /* terrane-site: core/paths.trn:40:29-40:50 */,
                                        ),
                                        part,
                                    ),
                                7 /* terrane-site: core/paths.trn:40:29-40:50 */,
                            );
                        } else {
                            kept.append(part);
                        }
                        count = count.clone() + terrane_int_support::Int::from(1_i128);
                    }
                }
            } else {
                if count.clone()
                    < terrane_int_support::Int::from(
                        terrane_int_support::Int::from(kept.length()),
                    )
                {
                    __terrane_raised(
                        kept
                            .set(
                                __terrane_raised(
                                    terrane_collection_support::index_from_int(&count.clone()),
                                    8 /* terrane-site: core/paths.trn:46:21-46:42 */,
                                ),
                                part,
                            ),
                        8 /* terrane-site: core/paths.trn:46:21-46:42 */,
                    );
                } else {
                    kept.append(part);
                }
                count = count.clone() + terrane_int_support::Int::from(1_i128);
            }
        }
        part_index = part_index.clone() + terrane_int_support::Int::from(1_i128);
    }
    let mut result: String = String::from("");
    let mut index: terrane_int_support::Int = terrane_int_support::Int::from(0_i128);
    while index.clone() < count.clone() {
        if result != String::from("") {
            result = format!(
                "{}{}", terrane_scalar_support::scalar_text(&result),
                terrane_scalar_support::scalar_text(&String::from("/"))
            );
        }
        result = format!(
            "{}{}", terrane_scalar_support::scalar_text(&result),
            terrane_scalar_support::scalar_text(&__terrane_raised(kept
            .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&index
            .clone()), 9 /* terrane-site: core/paths.trn:56:33-56:44 */)),
            9 /* terrane-site: core/paths.trn:56:33-56:44 */))
        );
        index = index.clone() + terrane_int_support::Int::from(1_i128);
    }
    if absolute {
        result = format!(
            "{}{}", terrane_scalar_support::scalar_text(&String::from("/")),
            terrane_scalar_support::scalar_text(&result)
        );
    }
    if result == String::from("") && absolute {
        result = String::from("/");
    }
    return Path::terrane_construct(result);
}
pub fn path_name(subject: Path) -> String {
    let normal: Path = normalise_path(subject.clone());
    let parts: terrane_collection_support::List<String> = path_components(normal);
    if terrane_int_support::Int::from(terrane_int_support::Int::from(parts.length()))
        == terrane_int_support::Int::from(0_i128)
    {
        return String::from("");
    }
    return __terrane_raised(
        parts
            .get_or_error(
                __terrane_raised(
                    terrane_collection_support::index_from_int(
                        &(terrane_int_support::Int::from(
                            terrane_int_support::Int::from(parts.length()),
                        ) - terrane_int_support::Int::from(1_i128)),
                    ),
                    10 /* terrane-site: core/paths.trn:69:12-69:35 */,
                ),
            ),
        10 /* terrane-site: core/paths.trn:69:12-69:35 */,
    );
}
pub fn path_parent(subject: Path) -> Path {
    let normal: Path = normalise_path(subject.clone());
    let parts: terrane_collection_support::List<String> = path_components(
        normal.clone(),
    );
    if terrane_int_support::Int::from(terrane_int_support::Int::from(parts.length()))
        == terrane_int_support::Int::from(0_i128)
    {
        return normal.clone();
    }
    if terrane_int_support::Int::from(terrane_int_support::Int::from(parts.length()))
        == terrane_int_support::Int::from(1_i128) && !path_is_absolute(normal.clone())
    {
        return Path::terrane_construct(String::from("."));
    }
    let mut result: String = String::from("");
    let mut index: terrane_int_support::Int = terrane_int_support::Int::from(0_i128);
    while index.clone()
        < terrane_int_support::Int::from(terrane_int_support::Int::from(parts.length()))
            - terrane_int_support::Int::from(1_i128)
    {
        if result != String::from("") {
            result = format!(
                "{}{}", terrane_scalar_support::scalar_text(&result),
                terrane_scalar_support::scalar_text(&String::from("/"))
            );
        }
        result = format!(
            "{}{}", terrane_scalar_support::scalar_text(&result),
            terrane_scalar_support::scalar_text(&__terrane_raised(parts
            .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&index
            .clone()), 11 /* terrane-site: core/paths.trn:83:33-83:45 */)),
            11 /* terrane-site: core/paths.trn:83:33-83:45 */))
        );
        index = index.clone() + terrane_int_support::Int::from(1_i128);
    }
    let absolute: bool = path_is_absolute(normal.clone());
    if absolute {
        result = format!(
            "{}{}", terrane_scalar_support::scalar_text(&String::from("/")),
            terrane_scalar_support::scalar_text(&result)
        );
    }
    return Path::terrane_construct(result);
}
pub fn path_stem(subject: Path) -> String {
    let current: String = path_name(subject.clone());
    let pieces: Vec<String> = terrane_string_support::split(
        &current,
        &String::from("."),
    );
    if terrane_int_support::Int::from(pieces.len() as i128)
        <= terrane_int_support::Int::from(1_i128)
    {
        return current.clone();
    }
    if terrane_int_support::Int::from(pieces.len() as i128)
        == terrane_int_support::Int::from(2_i128)
        && __terrane_raised(
            pieces
                .get(
                    __terrane_raised(
                        terrane_collection_support::index_from_int(
                            &terrane_int_support::Int::from(0_i128),
                        ),
                        12 /* terrane-site: core/paths.trn:95:31-95:40 */,
                    ),
                )
                .cloned()
                .ok_or_else(|| terrane_collection_support::IndexError::from_usize(
                    __terrane_raised(
                        terrane_collection_support::index_from_int(
                            &terrane_int_support::Int::from(0_i128),
                        ),
                        12 /* terrane-site: core/paths.trn:95:31-95:40 */,
                    ),
                )),
            12 /* terrane-site: core/paths.trn:95:31-95:40 */,
        ) == String::from("")
    {
        return current;
    }
    let mut result: String = String::from("");
    let mut index: terrane_int_support::Int = terrane_int_support::Int::from(0_i128);
    while index.clone()
        < terrane_int_support::Int::from(pieces.len() as i128)
            - terrane_int_support::Int::from(1_i128)
    {
        if index.clone() > terrane_int_support::Int::from(0_i128) {
            result = format!(
                "{}{}", terrane_scalar_support::scalar_text(&result),
                terrane_scalar_support::scalar_text(&String::from("."))
            );
        }
        result = format!(
            "{}{}", terrane_scalar_support::scalar_text(&result),
            terrane_scalar_support::scalar_text(&__terrane_raised(pieces
            .get(__terrane_raised(terrane_collection_support::index_from_int(&index
            .clone()), 13 /* terrane-site: core/paths.trn:102:33-102:46 */)).cloned()
            .ok_or_else(| |
            terrane_collection_support::IndexError::from_usize(__terrane_raised(terrane_collection_support::index_from_int(&index
            .clone()), 13 /* terrane-site: core/paths.trn:102:33-102:46 */))),
            13 /* terrane-site: core/paths.trn:102:33-102:46 */))
        );
        index = index.clone() + terrane_int_support::Int::from(1_i128);
    }
    return result;
}
pub fn path_extension(subject: Path) -> String {
    let current: String = path_name(subject.clone());
    let pieces: Vec<String> = terrane_string_support::split(
        &current,
        &String::from("."),
    );
    if terrane_int_support::Int::from(pieces.len() as i128)
        <= terrane_int_support::Int::from(1_i128)
    {
        return String::from("");
    }
    if terrane_int_support::Int::from(pieces.len() as i128)
        == terrane_int_support::Int::from(2_i128)
        && __terrane_raised(
            pieces
                .get(
                    __terrane_raised(
                        terrane_collection_support::index_from_int(
                            &terrane_int_support::Int::from(0_i128),
                        ),
                        14 /* terrane-site: core/paths.trn:111:31-111:40 */,
                    ),
                )
                .cloned()
                .ok_or_else(|| terrane_collection_support::IndexError::from_usize(
                    __terrane_raised(
                        terrane_collection_support::index_from_int(
                            &terrane_int_support::Int::from(0_i128),
                        ),
                        14 /* terrane-site: core/paths.trn:111:31-111:40 */,
                    ),
                )),
            14 /* terrane-site: core/paths.trn:111:31-111:40 */,
        ) == String::from("")
    {
        return String::from("");
    }
    return __terrane_raised(
        pieces
            .get(
                __terrane_raised(
                    terrane_collection_support::index_from_int(
                        &(terrane_int_support::Int::from(pieces.len() as i128)
                            - terrane_int_support::Int::from(1_i128)),
                    ),
                    15 /* terrane-site: core/paths.trn:113:12-113:37 */,
                ),
            )
            .cloned()
            .ok_or_else(|| terrane_collection_support::IndexError::from_usize(
                __terrane_raised(
                    terrane_collection_support::index_from_int(
                        &(terrane_int_support::Int::from(pieces.len() as i128)
                            - terrane_int_support::Int::from(1_i128)),
                    ),
                    15 /* terrane-site: core/paths.trn:113:12-113:37 */,
                ),
            )),
        15 /* terrane-site: core/paths.trn:113:12-113:37 */,
    );
}
pub fn join_path(base: Path, child: Path) -> Path {
    let absolute: bool = path_is_absolute(child.clone());
    if absolute {
        return normalise_path(child.clone());
    }
    let mut joined: String = base.text.clone();
    if joined != String::from("") && !joined.ends_with(&String::from("/")) {
        joined = format!(
            "{}{}", terrane_scalar_support::scalar_text(&joined),
            terrane_scalar_support::scalar_text(&String::from("/"))
        );
    }
    joined = format!(
        "{}{}", terrane_scalar_support::scalar_text(&joined),
        terrane_scalar_support::scalar_text(&child.text)
    );
    let combined: Path = Path::terrane_construct(joined);
    return normalise_path(combined);
}
