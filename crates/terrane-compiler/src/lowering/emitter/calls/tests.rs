use std::cell::Cell;

struct Reset;
impl Drop for Reset {
    fn drop(&mut self) {
        CALL_WORK.with(|work| work.set(None));
    }
}
thread_local! {
    static CALL_WORK: Cell<Option<usize>> = const { Cell::new(None) };
}

pub(super) fn count_call(file: u32) {
    // compile() assigns the authored input file 0. Bundled support is unrelated
    // fixed work, so the bound measures only calls from this input tree.
    if file == 0 {
        CALL_WORK.with(|work| {
            if let Some(remaining) = work.get() {
                assert!(remaining > 0, "nested call lowering work budget exceeded");
                work.set(Some(remaining - 1));
            }
        });
    }
}

fn compile_nested_calls(callable_value: bool) {
    // This is CLI code generation, not editor-worker analysis. Match the main
    // thread's stack rather than making the 2 MiB libtest stack a lowering limit.
    // The separate LSP regression deliberately keeps the default worker stack.
    std::thread::Builder::new()
        .stack_size(8 * 1024 * 1024)
        .spawn(move || {
    const DEPTH: usize = 40;
    let callee = if callable_value { "action" } else { "wrap" };
    let nested = (0..DEPTH).fold("value".to_owned(), |value, _| {
        format!("({callee}; {value})")
    });
    let binding = if callable_value {
        "    action function from int to int = wrap\n"
    } else {
        ""
    };
    let source = format!(
        "namespace nested-work\nfunction wrap int; value int\n    return value\nfunction probe int; value int\n{binding}    return {nested}\nfunction main;\n    print; (probe; 7)\n"
    );
    CALL_WORK.with(|work| {
        assert!(work.get().is_none());
        work.set(Some(4 * DEPTH + 32));
    });
    let _reset = Reset;
    // A deterministic linear upper bound catches repeated emission of discarded
    // argument trees without a machine-dependent elapsed-time assertion.
    crate::compile("nested-work.trn", source).unwrap();
        })
        .unwrap()
        .join()
        .unwrap();
}

#[test]
fn nested_call_work_is_linear() {
    compile_nested_calls(false);
}

#[test]
fn callable_argument_work_is_linear() {
    compile_nested_calls(true);
}
