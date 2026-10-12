// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: standard-text-streams
fn main() {
    let bytes_input: ByteReader;
    let input: TextReader;
    let read: TextReadResult;
    let bytes_output: ByteWriter;
    let output: TextWriter;
    let copied: WriteResult;
    let first: WriteResult;
    let second: WriteResult;
    let flushed: StreamOperationResult;
    let input_closed: StreamOperationResult;
    let output_closed: StreamOperationResult;
    bytes_input = stdin();
    input = bytes_input.text(terrane_string_support::Encoding::Utf8);
    read = __terrane_traced(
        input.read_all(terrane_int_support::Int::from(100_i128)),
        0 /* terrane-site: case.trn:7:12-7:31 */,
    );
    if read.failed {
        println!("{}", terrane_scalar_support::scalar_text(&read.message));
    }
    if read.completed.clone() != terrane_int_support::Int::from(3_i128) || !read.end {
        println!("{}", String::from("unexpected text read"));
    }
    bytes_output = stdout();
    output = bytes_output.text(terrane_string_support::Encoding::Utf8);
    copied = output.write_all(read.text.clone());
    if copied.failed {
        println!("{}", terrane_scalar_support::scalar_text(&copied.message));
    }
    if copied.completed.clone() != terrane_int_support::Int::from(3_i128) {
        println!("{}", String::from("unexpected copied count"));
    }
    first = output.write(String::from("one\r"));
    if first.failed {
        println!("{}", terrane_scalar_support::scalar_text(&first.message));
    }
    if first.completed.clone() != terrane_int_support::Int::from(4_i128) {
        println!("{}", String::from("unexpected text write count"));
    }
    second = output.line(String::from("two"));
    if second.failed {
        println!("{}", terrane_scalar_support::scalar_text(&second.message));
    }
    if second.completed.clone() != terrane_int_support::Int::from(4_i128) {
        println!("{}", String::from("unexpected line count"));
    }
    flushed = output.flush();
    if flushed.failed {
        println!("{}", terrane_scalar_support::scalar_text(&flushed.message));
    }
    input_closed = input.close();
    if input_closed.failed {
        println!("{}", terrane_scalar_support::scalar_text(&input_closed.message));
    }
    output_closed = output.close();
    if output_closed.failed {
        println!("{}", terrane_scalar_support::scalar_text(&output_closed.message));
    }
}
