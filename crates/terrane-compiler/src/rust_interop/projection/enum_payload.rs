use super::{
    BTreeMap, HashMap, Id, Item, ItemEnum, ItemSummary, ProjectedBoundaryCapabilities,
    ProjectedEnumPayload, ProjectedEnumPayloadConversion, ProjectedEnumPayloadOperation,
    ProjectedEnumPayloadStyle, ProjectedEnumVariant, ProjectedEnumVariantStyle, ProjectedField,
    ProjectedFieldConversion, ProjectedGenericParameter, ProjectedItem, ProjectedKind,
    ProjectedType, Type, is_rust_byte_vector_type, is_rust_string_type, project_type,
    render_rust_type, type_implements_deref_target, type_implements_generic_trait,
};

pub(super) fn project_variant(
    name: &str,
    variant: &rustdoc_types::Variant,
    index: &HashMap<Id, Item>,
    paths: &HashMap<Id, ItemSummary>,
    generics: &BTreeMap<String, ProjectedType>,
) -> ProjectedEnumVariant {
    use rustdoc_types::VariantKind;
    let (style, projected) = match &variant.kind {
        VariantKind::Plain => (ProjectedEnumVariantStyle::Unit, Ok(Vec::new())),
        VariantKind::Tuple(fields) => (
            ProjectedEnumVariantStyle::Tuple,
            project_enum_payload_fields(
                fields.iter().enumerate().map(|(slot, field)| {
                    let name = slot.to_string();
                    (name.clone(), name, *field)
                }),
                index,
                paths,
                generics,
            ),
        ),
        VariantKind::Struct {
            fields,
            has_stripped_fields,
        } => (
            ProjectedEnumVariantStyle::Struct,
            if *has_stripped_fields {
                Err("payload enum variant fields are stripped".to_owned())
            } else {
                project_enum_payload_fields(
                    fields.iter().map(|id| {
                        let name = index
                            .get(id)
                            .and_then(|item| item.name.clone())
                            .unwrap_or_default();
                        (name.clone(), name, Some(*id))
                    }),
                    index,
                    paths,
                    generics,
                )
            },
        ),
    };
    let (fields, unavailable_reason) = match projected {
        Ok(fields) => (fields, None),
        Err(reason) => (Vec::new(), Some(reason)),
    };
    ProjectedEnumVariant {
        name: name.to_owned(),
        style,
        fields,
        constructible: unavailable_reason.is_none(),
        unavailable_reason,
    }
}

pub(super) fn project_enum_payload(
    ty: &Type,
    index: &HashMap<Id, Item>,
    paths: &HashMap<Id, ItemSummary>,
    generics: &BTreeMap<String, ProjectedType>,
) -> Result<
    (
        ProjectedType,
        ProjectedEnumPayloadConversion,
        ProjectedType,
        ProjectedEnumPayloadConversion,
        String,
    ),
    String,
> {
    let projected = project_type(ty, index, paths, generics)?;
    let rust_type = render_rust_type(ty, index, paths, generics)?;
    let mut constructor_type = projected.clone();
    let mut constructor_conversion = ProjectedEnumPayloadConversion::Identity;
    let mut extraction_type = projected.clone();
    let mut extraction_conversion = ProjectedEnumPayloadConversion::Identity;
    if matches!(projected, ProjectedType::Foreign { .. }) {
        if type_implements_generic_trait(
            ty,
            index,
            paths,
            "core::convert::From",
            is_rust_string_type,
        ) {
            constructor_type = ProjectedType::String;
            constructor_conversion = ProjectedEnumPayloadConversion::Into;
        } else if type_implements_generic_trait(
            ty,
            index,
            paths,
            "core::convert::From",
            is_rust_byte_vector_type,
        ) {
            constructor_type = ProjectedType::Bytes;
            constructor_conversion = ProjectedEnumPayloadConversion::Into;
        }
        if type_implements_generic_trait(
            ty,
            index,
            paths,
            "core::convert::AsRef",
            |argument, _| matches!(argument, Type::Primitive(name) if name == "str"),
        ) {
            extraction_type = ProjectedType::String;
            extraction_conversion = ProjectedEnumPayloadConversion::AsRefString;
        } else if type_implements_deref_target(
            ty,
            index,
            paths,
            |target| matches!(target, Type::Primitive(name) if name == "str"),
        ) {
            extraction_type = ProjectedType::String;
            extraction_conversion = ProjectedEnumPayloadConversion::DerefString;
        } else if type_implements_generic_trait(
            ty,
            index,
            paths,
            "core::convert::AsRef",
            |argument, _| {
                matches!(
                    argument,
                    Type::Slice(item) if matches!(item.as_ref(), Type::Primitive(name) if name == "u8")
                )
            },
        ) {
            extraction_type = ProjectedType::Bytes;
            extraction_conversion = ProjectedEnumPayloadConversion::AsRefBytes;
        } else if type_implements_deref_target(ty, index, paths, |target| {
            matches!(
                target,

                Type::Slice(item) if matches!(item.as_ref(), Type::Primitive(name) if name == "u8")
            )
        }) {
            extraction_type = ProjectedType::Bytes;
            extraction_conversion = ProjectedEnumPayloadConversion::DerefBytes;
        }
    }
    Ok((
        constructor_type,
        constructor_conversion,
        extraction_type,
        extraction_conversion,
        rust_type,
    ))
}
fn project_enum_payload_fields(
    fields: impl Iterator<Item = (String, String, Option<Id>)>,
    index: &HashMap<Id, Item>,
    paths: &HashMap<Id, ItemSummary>,
    generics: &BTreeMap<String, ProjectedType>,
) -> Result<Vec<ProjectedField>, String> {
    fields
        .map(|(name, rust_name, field)| {
            if name.is_empty() {
                return Err("payload enum variant field metadata is unavailable".to_owned());
            }
            let Some(field) = field.as_ref().and_then(|id| index.get(id)) else {
                return Err("payload enum variant field is stripped".to_owned());
            };
            let ItemEnum::StructField(rustdoc_type) = &field.inner else {
                return Err("payload enum variant field metadata is unavailable".to_owned());
            };
            let ty = project_type(rustdoc_type, index, paths, generics)?;
            let conversion = enum_payload_field_conversion(&ty).ok_or_else(|| {
                format!("payload enum variant field `{rust_name}` has no owned field conversion")
            })?;
            Ok(ProjectedField {
                name: if crate::syntax::is_keyword(&name) {
                    format!("{name}-value")
                } else {
                    name.replace('_', "-")
                },
                rust_name,
                ty,
                rust_type: render_rust_type(rustdoc_type, index, paths, generics)?,
                conversion,
            })
        })
        .collect()
}

fn enum_payload_field_conversion(ty: &ProjectedType) -> Option<ProjectedFieldConversion> {
    if ty.is_terrane_scalar() || matches!(ty, ProjectedType::None | ProjectedType::Foreign { .. }) {
        Some(ProjectedFieldConversion::Identity)
    } else if matches!(ty, ProjectedType::Optional(inner) if enum_payload_field_conversion(inner) == Some(ProjectedFieldConversion::Identity))
    {
        Some(ProjectedFieldConversion::OptionalOwned)
    } else {
        None
    }
}
type ProjectedEnumPayloadProjection = (
    ProjectedType,
    ProjectedEnumPayloadConversion,
    ProjectedType,
    ProjectedEnumPayloadConversion,
    String,
    Option<ProjectedEnumPayloadOperation>,
);

type ProjectedEnumPayloadItem = (ProjectedItem, ProjectedEnumPayloadProjection);

#[expect(
    clippy::too_many_arguments,
    reason = "the payload item preserves its complete enum owner and source identity"
)]
pub(super) fn project_multi_enum_payload(
    namespace: &str,
    enum_name: &str,
    owner_rust_path: &str,
    variant: &str,
    docs: Option<String>,
    style: ProjectedEnumPayloadStyle,
    fields: impl Iterator<Item = (String, String, Option<Id>)>,
    index: &HashMap<Id, Item>,
    paths: &HashMap<Id, ItemSummary>,
    generics: &BTreeMap<String, ProjectedType>,
) -> Result<ProjectedEnumPayloadItem, String> {
    let fields = project_enum_payload_fields(fields, index, paths, generics)?;
    let generic_parameters = generics
        .iter()
        .filter(|(name, ty)| {
            matches!(ty, ProjectedType::Generic(_))
                && fields.iter().any(|field| field.ty.contains_generic(name))
        })
        .map(|(name, _)| ProjectedGenericParameter {
            name: name.clone(),
            input_selected: true,
            rust_bounds: Vec::new(),
        })
        .collect::<Vec<_>>();
    let payload_name = format!("{enum_name}-{variant}");
    let payload_rust_path = format!("{owner_rust_path}::{variant}#payload");
    let payload_type = ProjectedType::Foreign {
        rust_path: payload_rust_path.clone(),
        name: payload_name.clone(),
        base_rust_path: payload_rust_path.clone(),
        arguments: generic_parameters
            .iter()
            .map(|parameter| ProjectedType::Generic(parameter.name.clone()))
            .collect(),
    };
    let operation = ProjectedEnumPayloadOperation {
        style,
        fields: fields.iter().map(|field| field.rust_name.clone()).collect(),
    };
    let item = ProjectedItem {
        namespace: namespace.to_owned(),
        name: payload_name,
        rust_path: payload_rust_path.clone(),
        docs,
        kind: ProjectedKind::ForeignType {
            constructor: None,
            methods: Vec::new(),
            static_methods: Vec::new(),
            constants: Vec::new(),
            boundary: ProjectedBoundaryCapabilities::default(),
            fields,
            borrowed_view: false,
            native_view_type: None,
            enum_payload: Some(ProjectedEnumPayload {
                owner_rust_path: owner_rust_path.to_owned(),
                variant: variant.to_owned(),
                style,
            }),
            generic_parameters,
            displayable: false,
            cloneable: false,
            send: false,
            sync: false,
        },
    };
    Ok((
        item,
        (
            payload_type.clone(),
            ProjectedEnumPayloadConversion::Identity,
            payload_type,
            ProjectedEnumPayloadConversion::Identity,
            payload_rust_path,
            Some(operation),
        ),
    ))
}
