// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/main.trn
// Namespace: streams-no-prelude
fn main() {
    let input: ByteReader;
    let output: ByteWriter;
    let errors: ByteWriter;
    input = stdin();
    input.close();
    output = stdout();
    output.close();
    errors = stderr();
    errors.close();
}
