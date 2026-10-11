// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/main.trn
// Namespace: app
fn main() {
    let output: ByteWriter;
    output = stdout();
    output.write_all(Vec::from([97, 108, 105, 97, 115, 45, 111, 107, 10]));
    output.flush();
    output.close();
}
