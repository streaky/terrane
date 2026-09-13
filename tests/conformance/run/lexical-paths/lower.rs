// Generated deterministically by Terrane <version>.
// Runtime support: 
// Vendored support crates: terrane-int-support, terrane-collection-support, terrane-scalar-support, terrane-string-support
// Source: case.trn
// Namespace: conformance/lexical-paths
fn main() {
    let relative: Path = Path::terrane_construct(
        String::from("alpha/./beta/../gamma/../../delta"),
    );
    let rooted: Path = Path::terrane_construct(
        String::from("/alpha/../../beta/file.tar.gz"),
    );
    let base: Path = Path::terrane_construct(String::from("work/root"));
    let child: Path = Path::terrane_construct(String::from("../next"));
    let relative_normal: Path = normalise_path(relative);
    let rooted_normal: Path = normalise_path(rooted);
    let relative_text: String = relative_normal.text.clone();
    let rooted_text: String = rooted_normal.text.clone();
    let rooted_name: String = path_name(rooted_normal.clone());
    let rooted_stem: String = path_stem(rooted_normal.clone());
    let rooted_extension: String = path_extension(rooted_normal.clone());
    let rooted_parent: Path = path_parent(rooted_normal.clone());
    let rooted_parent_text: String = rooted_parent.text.clone();
    println!("{}", terrane_scalar_support::scalar_text(&relative_text));
    println!("{}", terrane_scalar_support::scalar_text(&rooted_text));
    println!("{}", terrane_scalar_support::scalar_text(&rooted_name));
    println!("{}", terrane_scalar_support::scalar_text(&rooted_stem));
    println!("{}", terrane_scalar_support::scalar_text(&rooted_extension));
    println!("{}", terrane_scalar_support::scalar_text(&rooted_parent_text));
    let resolved: Path = join_path(base, child);
    let resolved_text: String = resolved.text.clone();
    println!("{}", terrane_scalar_support::scalar_text(&resolved_text));
    let components: terrane_collection_support::List<String> = path_components(
        rooted_normal.clone(),
    );
    println!(
        "{}{}{}",
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(components
        .length())), terrane_scalar_support::scalar_text(&path_is_absolute(rooted_normal
        .clone())), terrane_scalar_support::scalar_text(&path_is_absolute(relative_normal
        .clone()))
    );
    let hidden: Path = Path::terrane_construct(String::from(".profile"));
    println!("{}", terrane_scalar_support::scalar_text(&path_stem(hidden.clone())));
    println!("{}", terrane_scalar_support::scalar_text(&path_extension(hidden.clone())));
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&path_parent(Path::terrane_construct(String::from("child")))
        .text)
    );
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
                            0 /* terrane-site: core/paths.trn:16:16-16:28 */,
                        ),
                    )
                    .cloned()
                    .ok_or_else(|| terrane_collection_support::IndexError::from_usize(
                        __terrane_raised(
                            terrane_collection_support::index_from_int(&index.clone()),
                            0 /* terrane-site: core/paths.trn:16:16-16:28 */,
                        ),
                    )),
                0 /* terrane-site: core/paths.trn:16:16-16:28 */,
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
                        1 /* terrane-site: core/paths.trn:32:16-32:33 */,
                    ),
                )
                .cloned()
                .ok_or_else(|| terrane_collection_support::IndexError::from_usize(
                    __terrane_raised(
                        terrane_collection_support::index_from_int(&part_index.clone()),
                        1 /* terrane-site: core/paths.trn:32:16-32:33 */,
                    ),
                )),
            1 /* terrane-site: core/paths.trn:32:16-32:33 */,
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
                                    2 /* terrane-site: core/paths.trn:35:34-35:49 */,
                                ),
                            ),
                        2 /* terrane-site: core/paths.trn:35:34-35:49 */,
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
                                            3 /* terrane-site: core/paths.trn:40:29-40:50 */,
                                        ),
                                        part,
                                    ),
                                3 /* terrane-site: core/paths.trn:40:29-40:50 */,
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
                                    4 /* terrane-site: core/paths.trn:46:21-46:42 */,
                                ),
                                part,
                            ),
                        4 /* terrane-site: core/paths.trn:46:21-46:42 */,
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
            .clone()), 5 /* terrane-site: core/paths.trn:56:33-56:44 */)),
            5 /* terrane-site: core/paths.trn:56:33-56:44 */))
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
                    6 /* terrane-site: core/paths.trn:69:12-69:35 */,
                ),
            ),
        6 /* terrane-site: core/paths.trn:69:12-69:35 */,
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
            .clone()), 7 /* terrane-site: core/paths.trn:83:33-83:45 */)),
            7 /* terrane-site: core/paths.trn:83:33-83:45 */))
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
                        8 /* terrane-site: core/paths.trn:95:31-95:40 */,
                    ),
                )
                .cloned()
                .ok_or_else(|| terrane_collection_support::IndexError::from_usize(
                    __terrane_raised(
                        terrane_collection_support::index_from_int(
                            &terrane_int_support::Int::from(0_i128),
                        ),
                        8 /* terrane-site: core/paths.trn:95:31-95:40 */,
                    ),
                )),
            8 /* terrane-site: core/paths.trn:95:31-95:40 */,
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
            .clone()), 9 /* terrane-site: core/paths.trn:102:33-102:46 */)).cloned()
            .ok_or_else(| |
            terrane_collection_support::IndexError::from_usize(__terrane_raised(terrane_collection_support::index_from_int(&index
            .clone()), 9 /* terrane-site: core/paths.trn:102:33-102:46 */))),
            9 /* terrane-site: core/paths.trn:102:33-102:46 */))
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
                        10 /* terrane-site: core/paths.trn:111:31-111:40 */,
                    ),
                )
                .cloned()
                .ok_or_else(|| terrane_collection_support::IndexError::from_usize(
                    __terrane_raised(
                        terrane_collection_support::index_from_int(
                            &terrane_int_support::Int::from(0_i128),
                        ),
                        10 /* terrane-site: core/paths.trn:111:31-111:40 */,
                    ),
                )),
            10 /* terrane-site: core/paths.trn:111:31-111:40 */,
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
                    11 /* terrane-site: core/paths.trn:113:12-113:37 */,
                ),
            )
            .cloned()
            .ok_or_else(|| terrane_collection_support::IndexError::from_usize(
                __terrane_raised(
                    terrane_collection_support::index_from_int(
                        &(terrane_int_support::Int::from(pieces.len() as i128)
                            - terrane_int_support::Int::from(1_i128)),
                    ),
                    11 /* terrane-site: core/paths.trn:113:12-113:37 */,
                ),
            )),
        11 /* terrane-site: core/paths.trn:113:12-113:37 */,
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
