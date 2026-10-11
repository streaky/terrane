// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/library.trn
// Namespace: native-testing-surface
static __TERRANE_F0_AVAILABLE: std::sync::LazyLock<bool> = std::sync::LazyLock::new(|| {
    true
});
// Source: tests/unit/surface.trn
// Namespace: native-testing-surface
fn expected_error() -> Result<(), TerraneError> {
    __terrane_traced_err(
        fail(String::from("expected")),
        0 /* terrane-site: tests/unit/surface.trn:6:5-6:21 */,
    )?;
    return Ok(());
}
fn test_surface() -> Result<(), TerraneError> {
    let fixture: ProcessFixture;
    __terrane_traced_err(
        assert(true),
        1 /* terrane-site: tests/unit/surface.trn:10:5-10:17 */,
    )?;
    __terrane_traced_err(
        deny(false),
        2 /* terrane-site: tests/unit/surface.trn:11:5-11:16 */,
    )?;
    __terrane_traced_err(
        assert_equal_int(
            terrane_int_support::Int::from(4_i128),
            terrane_int_support::Int::from(4_i128),
        ),
        3 /* terrane-site: tests/unit/surface.trn:12:5-12:27 */,
    )?;
    __terrane_traced_err(
        assert_not_equal_string(String::from("left"), String::from("right")),
        4 /* terrane-site: tests/unit/surface.trn:13:5-13:45 */,
    )?;
    __terrane_traced_err(
        assert_present_string(Some(String::from("present"))),
        5 /* terrane-site: tests/unit/surface.trn:14:5-14:37 */,
    )?;
    __terrane_traced_err(
        assert_none_string(None),
        6 /* terrane-site: tests/unit/surface.trn:15:5-15:29 */,
    )?;
    __terrane_traced_err(
        assert_near(1.0, 1.1, 0.2),
        7 /* terrane-site: tests/unit/surface.trn:16:5-16:31 */,
    )?;
    __terrane_traced_err(
        assert_throws(
            std::sync::Arc::new(expected_error),
            TerraneDescriptor {
                identity: "/core/testing::test-failure",
                name: "test-failure",
                kind: "class",
                inherently_identity_bearing: false,
                fields: &[
                    TerraneFieldMetadata {
                        name: "message",
                        external_name: "message",
                        defaulted: true,
                        optional: false,
                        secret: false,
                    },
                    TerraneFieldMetadata {
                        name: "assertion-source",
                        external_name: "assertion-source",
                        defaulted: true,
                        optional: false,
                        secret: false,
                    },
                    TerraneFieldMetadata {
                        name: "details",
                        external_name: "details",
                        defaulted: true,
                        optional: false,
                        secret: false,
                    },
                ],
            }
                .identity
                .to_owned(),
        ),
        8 /* terrane-site: tests/unit/surface.trn:17:5-17:57 */,
    )?;
    fixture = ProcessFixture::terrane_construct(String::from("artifact"));
    __terrane_traced_err(
        assert_equal_string(fixture.artifact.clone(), String::from("artifact")),
        9 /* terrane-site: tests/unit/surface.trn:19:5-19:54 */,
    )?;
    temporary_directory();
    return Ok(());
}
// Source: <compiler-generated test registry>
// Namespace: core/testing/runner
fn __terrane_test_hex(value: &str) -> String {
    use std::fmt::Write as _;
    let mut encoded = String::with_capacity(value.len() * 2);
    for byte in value.as_bytes() {
        write!(encoded, "{byte:02x}").expect("writing to a string cannot fail");
    }
    encoded
}
fn __terrane_test_record(outcome: &str, error: &TerraneError) {
    let Ok(path) = std::env::var("TERRANE_TEST_RESULT") else {
        return;
    };
    let details = error.structured_details();
    let frames = error.source_frames();
    let mut record = format!(
        "1\n{}\n{}\n{}\n{}\n{}\n", outcome, __terrane_test_hex(&error.descriptor_name()),
        __terrane_test_hex(error.message()), details.len(), frames.len(),
    );
    for detail in details {
        record.push_str(&__terrane_test_hex(detail));
        record.push('\n');
    }
    for frame in frames {
        record.push_str(&__terrane_test_hex(&frame));
        record.push('\n');
    }
    let _ = std::fs::write(path, record);
}
fn main() {
    let selected = std::env::args().nth(1).unwrap_or_default();
    match selected.as_str() {
        "0" => {
            if let Err(error) = test_surface() {
                let skipped = error.descriptor_name() == "/core/testing::test-skip";
                __terrane_test_record(
                    if skipped { "skipped" } else { "failed" },
                    &error,
                );
                std::process::exit(if skipped { 102 } else { 101 });
            }
        }
        _ => {
            eprintln!("unknown Terrane test case");
            std::process::exit(103);
        }
    }
}
