// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: conformance/lexical-paths
fn main() {
    let relative: Path;
    let rooted: Path;
    let base: Path;
    let child: Path;
    let relative_normal: Path;
    let rooted_normal: Path;
    let relative_text: String;
    let rooted_text: String;
    let rooted_name: String;
    let rooted_stem: String;
    let rooted_extension: String;
    let rooted_parent: Path;
    let rooted_parent_text: String;
    let resolved: Path;
    let resolved_text: String;
    let components: terrane_collection_support::List<String>;
    let hidden: Path;
    relative = Path::terrane_construct(
        String::from("alpha/./beta/../gamma/../../delta"),
    );
    rooted = Path::terrane_construct(String::from("/alpha/../../beta/file.tar.gz"));
    base = Path::terrane_construct(String::from("work/root"));
    child = Path::terrane_construct(String::from("../next"));
    relative_normal = normalise_path(relative);
    rooted_normal = normalise_path(rooted);
    relative_text = relative_normal.text.clone();
    rooted_text = rooted_normal.text.clone();
    rooted_name = path_name(rooted_normal.clone());
    rooted_stem = path_stem(rooted_normal.clone());
    rooted_extension = path_extension(rooted_normal.clone());
    rooted_parent = path_parent(rooted_normal.clone());
    rooted_parent_text = rooted_parent.text.clone();
    println!("{}", terrane_scalar_support::scalar_text(&relative_text));
    println!("{}", terrane_scalar_support::scalar_text(&rooted_text));
    println!("{}", terrane_scalar_support::scalar_text(&rooted_name));
    println!("{}", terrane_scalar_support::scalar_text(&rooted_stem));
    println!("{}", terrane_scalar_support::scalar_text(&rooted_extension));
    println!("{}", terrane_scalar_support::scalar_text(&rooted_parent_text));
    resolved = join_path(base, child);
    resolved_text = resolved.text.clone();
    println!("{}", terrane_scalar_support::scalar_text(&resolved_text));
    components = path_components(rooted_normal.clone());
    println!(
        "{}{}{}",
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(components
        .length())),
        terrane_scalar_support::scalar_text(&path_is_absolute(rooted_normal)),
        terrane_scalar_support::scalar_text(&path_is_absolute(relative_normal))
    );
    hidden = Path::terrane_construct(String::from(".profile"));
    println!("{}", terrane_scalar_support::scalar_text(&path_stem(hidden.clone())));
    println!("{}", terrane_scalar_support::scalar_text(&path_extension(hidden)));
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&path_parent(Path::terrane_construct(String::from("child")))
        .text)
    );
}
