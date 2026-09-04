mod call_support;
mod calls;
mod context;
mod expressions;
mod items;
mod members;
pub(super) mod pipeline;
mod statements;

use std::{cell::RefCell, collections::BTreeMap};

use num_bigint::BigInt;

use crate::{
    SourceFile,
    semantics::{ObjectIdentity, SemanticPackage, SemanticUnit, ValueType},
};

// Covers the 80 MiB scientific benchmark lists while bounding a mistaken speculative hint.
const LIST_PREALLOCATION_LIMIT_BYTES: usize = 256 * 1024 * 1024;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct LoweringSite {
    pub(super) function: u32,
    pub(super) file: u32,
    pub(super) line: u32,
    pub(super) column: u32,
    pub(super) end_line: u32,
    pub(super) end_column: u32,
}

#[derive(Default)]
pub(super) struct LoweringRegistry {
    pub(super) files: RefCell<Vec<String>>,
    pub(super) functions: RefCell<Vec<String>>,
    pub(super) sites: RefCell<Vec<LoweringSite>>,
    pub(super) descriptors: RefCell<Vec<(String, String)>>,
    pub(super) descriptor_ids: RefCell<BTreeMap<String, u32>>,
}

impl LoweringRegistry {
    pub(super) fn intern(values: &RefCell<Vec<String>>, value: &str) -> u32 {
        let mut values = values.borrow_mut();
        if let Some(index) = values.iter().position(|candidate| candidate == value) {
            return u32::try_from(index).expect("lowering registry index must fit u32");
        }
        let index = u32::try_from(values.len()).expect("lowering registry index must fit u32");
        values.push(value.to_owned());
        index
    }

    pub(super) fn register_site(
        &self,
        source_path: &str,
        function: &str,
        source: &SourceFile,
        span: crate::Span,
    ) -> u32 {
        let file = Self::intern(&self.files, source_path);
        let function = Self::intern(&self.functions, function);
        let (line, column) = source.line_column(span.start);
        let (end_line, end_column) = source.line_column(span.end);
        let candidate = LoweringSite {
            function,
            file,
            line: u32::try_from(line).expect("source line must fit u32"),
            column: u32::try_from(column).expect("source column must fit u32"),
            end_line: u32::try_from(end_line).expect("source line must fit u32"),
            end_column: u32::try_from(end_column).expect("source column must fit u32"),
        };
        let mut sites = self.sites.borrow_mut();
        if let Some(site) = sites.iter().position(|existing| existing == &candidate) {
            return u32::try_from(site).expect("lowering site index must fit u32");
        }
        let site = u32::try_from(sites.len()).expect("lowering site index must fit u32");
        assert_ne!(
            site,
            u32::MAX,
            "u32::MAX is reserved for an unattributed site"
        );
        sites.push(candidate);
        site
    }

    pub(super) fn register_descriptor(&self, identity: &str, source_name: &str) -> u32 {
        if let Some(id) = self.descriptor_ids.borrow().get(identity).copied() {
            return id;
        }
        let mut descriptors = self.descriptors.borrow_mut();
        let id = u32::try_from(descriptors.len()).expect("descriptor index must fit u32");
        descriptors.push((identity.to_owned(), source_name.to_owned()));
        self.descriptor_ids
            .borrow_mut()
            .insert(identity.to_owned(), id);
        id
    }
}

#[derive(Clone, Debug)]
pub(super) struct BoundedIntegerRange {
    pub(super) binding: crate::Span,
    pub(super) lower: BigInt,
    pub(super) upper: BigInt,
}

#[derive(Clone, Debug)]
pub(super) struct ListAppendBorrow {
    pub(super) binding: crate::Span,
    pub(super) vector: String,
}

#[expect(
    clippy::struct_excessive_bools,
    reason = "these independent lexical control contexts are saved and restored separately"
)]
pub(super) struct Emitter<'a> {
    registry: &'a LoweringRegistry,
    package: &'a SemanticPackage,
    unit: &'a SemanticUnit,
    source: &'a SourceFile,
    output: String,
    indent: usize,
    continue_label: Option<String>,
    loop_counter: usize,
    list_append_counter: usize,
    return_type: Option<ValueType>,
    parameter_types: Vec<(String, ValueType)>,
    namespace_initializer: Option<(String, String)>,
    propagate_errors: bool,
    discarded_call: Option<crate::Span>,
    function_errors: bool,
    try_counter: usize,
    current_error: Option<String>,
    current_function: Option<String>,
    current_object: Option<ObjectIdentity>,
    try_completion: bool,
    in_loop: bool,
    closure_depth: usize,
    assignment_target: bool,
    bounded_integer_ranges: Vec<BoundedIntegerRange>,
    list_append_borrows: Vec<ListAppendBorrow>,
}

impl<'a> Emitter<'a> {
    pub(in crate::lowering) fn new(
        registry: &'a LoweringRegistry,
        package: &'a SemanticPackage,
        unit: &'a SemanticUnit,
    ) -> Self {
        Self {
            registry,
            package,
            unit,
            source: &unit.source,
            output: String::new(),
            indent: 0,
            continue_label: None,
            loop_counter: 0,
            list_append_counter: 0,
            return_type: None,
            parameter_types: Vec::new(),
            namespace_initializer: None,
            propagate_errors: false,
            discarded_call: None,
            function_errors: false,
            try_counter: 0,
            current_error: None,
            current_function: None,
            current_object: None,
            try_completion: false,
            in_loop: false,
            closure_depth: 0,
            assignment_target: false,
            bounded_integer_ranges: Vec::new(),
            list_append_borrows: Vec::new(),
        }
    }
}
