use super::*;
use crate::{Package, semantics::analyze};

fn loan_for<'a>(
    unit: &'a SemanticUnit,
    access: &str,
    receiver: &str,
) -> Option<&'a StatementOwnerLoan> {
    let start = unit.source.text().find(access).unwrap() + access.find(receiver).unwrap();
    unit.statement_owner_loans
        .get(&(unit.tree.root.span.file, start, start + receiver.len()))
}

#[test]
fn loans_accept_reassigned_scalar_reads() {
    let source = "namespace app
class counter
    value int = 0
function assign; owner shared ref counter, value int
    owner.value = value + 1
function main;
    first = shared ref (instance counter;)
    local = first
    local.value = 1
    (local).value = 2
    local = first
    local.value = 3
    amount int = 4
    amount = amount + 1
    local.value = amount + 1
    index int = 0
    while index < 2
        local.value = index + 1
        index = index + 1
";
    let semantic = analyze(&Package::implicit("main.trn", source.to_owned())).unwrap();
    let unit = &semantic.units[0];
    for (access, receiver) in [
        ("owner.value = value + 1", "owner"),
        ("local.value = 1", "local"),
        ("(local).value = 2", "local"),
        ("local.value = 3", "local"),
        ("local.value = amount + 1", "local"),
        ("local.value = index + 1", "local"),
    ] {
        let loan =
            loan_for(unit, access, receiver).unwrap_or_else(|| panic!("missing loan: {access}"));
        assert_eq!(unit.typed_bindings[loan.binding_index].name, receiver);
    }
}

#[test]
fn effects_keep_independent_owners() {
    let source = "namespace app
class counter
    value int = 0
function unknown int; item shared ref counter
    return item.value
function projected; owners list of (shared ref counter)
    owners[0].value = 5
async function pending int;
    return 1
async function main;
    owner = shared ref (instance counter;)
    owner.value = (unknown; owner)
    owner.value = await pending;
    owner.value = owner.value + 1
    moved = owner
    moved.value = (unknown; move moved)
    captured = owner
    reader = function int;
        return captured.value
    captured.value = 7
    native = owner
    native.value = rust
        1
    amount int = 2
    view = ref amount
    owner.value = view + 1
";
    let semantic = analyze(&Package::implicit("main.trn", source.to_owned())).unwrap();
    let unit = &semantic.units[0];
    for (access, receiver) in [
        ("owners[0].value = 5", "owners"),
        ("owner.value = (unknown; owner)", "owner"),
        ("owner.value = await pending;", "owner"),
        ("owner.value = owner.value + 1", "owner"),
        ("moved.value = (unknown; move moved)", "moved"),
        ("captured.value = 7", "captured"),
        ("native.value = rust", "native"),
        ("owner.value = view + 1", "owner"),
    ] {
        assert!(
            loan_for(unit, access, receiver).is_none(),
            "unexpected loan: {access}"
        );
    }
}
