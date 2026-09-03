use super::prelude::*;

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

#[expect(
    clippy::struct_excessive_bools,
    reason = "these independent lexical control contexts are saved and restored separately"
)]
pub(super) struct Emitter<'a> {
    pub(super) registry: &'a LoweringRegistry,
    pub(super) package: &'a SemanticPackage,
    pub(super) unit: &'a SemanticUnit,
    pub(super) source: &'a SourceFile,
    pub(super) output: String,
    pub(super) indent: usize,
    pub(super) continue_label: Option<String>,
    pub(super) loop_counter: usize,
    pub(super) return_type: Option<ValueType>,
    pub(super) parameter_types: Vec<(String, ValueType)>,
    pub(super) namespace_initializer: Option<(String, String)>,
    pub(super) propagate_errors: bool,
    pub(super) discarded_call: Option<crate::Span>,
    pub(super) function_errors: bool,
    pub(super) try_counter: usize,
    pub(super) current_error: Option<String>,
    pub(super) current_function: Option<String>,
    pub(super) current_object: Option<ObjectIdentity>,
    pub(super) try_completion: bool,
    pub(super) in_loop: bool,
    pub(super) closure_depth: usize,
    pub(super) assignment_target: bool,
    pub(super) bounded_integer_ranges: Vec<BoundedIntegerRange>,
}
