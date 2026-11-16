use super::ValueType;
use ValueType::{
    AsyncFunction, AsyncIterationStep, AsyncSinkOutcome, ChannelOverflowPolicy, ChannelPair,
    ChannelReceiveOutcome, ChannelReceiver, ChannelSendOutcome, ChannelSender, Descriptor,
    DivRemResult, DocumentDecodeOutcome, DocumentDiagnostic, Encoding, Entry, FilesystemAuthority,
    FloatDecomposition, Function, InlineRust, InvocationScopedNative, IterationEnd, IterationStep,
    Iterator, List, Map, MutableReference, Object, Optional, OverflowResult, PlatformCapability,
    PlatformDataResult, PlatformFilesystemResult, PlatformOpenResult, PlatformReadResult,
    PlatformResourceHandle, PlatformResult, PlatformStreamHandle, PlatformUnitResult,
    PlatformUrlResult, PlatformWriteResult, ProjectedAssociated, ProjectedGeneric, Range,
    Reference, Scalar, ScopedTask, Set, SharedReference, StringList, StringView, Task, TaskGroup,
    TaskOutcome, TaskScope, TextRange, TextRangeList, TextRangeView, Tuple, TypeParameter, Union,
    UnorderedMap, UnorderedSet,
};
use std::collections::BTreeMap;

#[derive(Clone)]
pub struct ObjectIdentity(std::sync::Arc<ObjectIdentityData>);

#[derive(Clone)]
pub struct ObjectIdentityData {
    pub namespace: String,
    pub name: String,
    pub is_unsafe: bool,
    pub(crate) application: Option<Box<ValueType>>,
    pub(crate) application_key: Option<String>,
    pub type_arguments: Vec<ValueType>,
    pub(crate) type_arguments_key: Option<String>,
    pub(crate) native_projection: Option<String>,
    pub(crate) native_arguments: BTreeMap<String, ValueType>,
    pub(crate) native_arguments_key: Option<String>,
    pub(crate) native_parameters: Vec<String>,
}

impl std::ops::Deref for ObjectIdentity {
    type Target = ObjectIdentityData;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for ObjectIdentity {
    fn deref_mut(&mut self) -> &mut Self::Target {
        std::sync::Arc::make_mut(&mut self.0)
    }
}

impl From<ObjectIdentityData> for ObjectIdentity {
    fn from(data: ObjectIdentityData) -> Self {
        Self(std::sync::Arc::new(data))
    }
}

impl ObjectIdentity {
    pub(crate) fn into_namespace(self) -> String {
        match std::sync::Arc::try_unwrap(self.0) {
            Ok(data) => data.namespace,
            Err(data) => data.namespace.clone(),
        }
    }

    pub(crate) fn into_native_projection(self) -> Option<String> {
        match std::sync::Arc::try_unwrap(self.0) {
            Ok(data) => data.native_projection,
            Err(data) => data.native_projection.clone(),
        }
    }

    pub(crate) fn into_native_arguments(self) -> BTreeMap<String, ValueType> {
        match std::sync::Arc::try_unwrap(self.0) {
            Ok(data) => data.native_arguments,
            Err(data) => data.native_arguments.clone(),
        }
    }
}

enum NativeArgumentDebug<'a> {
    Positional(usize),
    Value(&'a ValueType),
}

impl std::fmt::Debug for NativeArgumentDebug<'_> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Positional(index) => formatter.debug_tuple("Positional").field(index).finish(),
            Self::Value(value) => formatter.debug_tuple("Value").field(value).finish(),
        }
    }
}

struct NativeArgumentsDebug<'a>(&'a ObjectIdentity);

impl std::fmt::Debug for NativeArgumentsDebug<'_> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut map = formatter.debug_map();
        for (name, value) in &self.0.native_arguments {
            let reference = self
                .0
                .type_arguments
                .iter()
                .position(|argument| argument == value);
            let debug = reference.map_or(
                NativeArgumentDebug::Value(value),
                NativeArgumentDebug::Positional,
            );
            map.entry(name, &debug);
        }
        map.finish()
    }
}

#[derive(serde::Serialize)]
enum NativeBindingRef<'a> {
    Positional(usize),
    Value(&'a ValueType),
}

// Wire compaction must preserve non-identity metadata and must not depend on
// allocation/sharing history. Memoize object pairs so repeated named/positional
// children are compared once, even for independently constructed equal trees.
#[derive(Default)]
struct ValuePayloadComparison {
    objects: std::collections::BTreeSet<(usize, usize)>,
}

impl ValuePayloadComparison {
    fn objects_equal(&mut self, left: &ObjectIdentity, right: &ObjectIdentity) -> bool {
        if std::sync::Arc::ptr_eq(&left.0, &right.0) {
            return true;
        }
        if left.native_parameters != right.native_parameters
            || left.type_arguments.len() != right.type_arguments.len()
            || left.native_arguments.len() != right.native_arguments.len()
        {
            return false;
        }
        let pair = (
            std::sync::Arc::as_ptr(&left.0) as usize,
            std::sync::Arc::as_ptr(&right.0) as usize,
        );
        if !self.objects.insert(pair) {
            return true;
        }
        let applications_equal = match (&left.application, &right.application) {
            (Some(left), Some(right)) => self.equal(left, right),
            (None, None) => true,
            _ => false,
        };
        applications_equal
            && left
                .type_arguments
                .iter()
                .zip(&right.type_arguments)
                .all(|(left, right)| self.equal(left, right))
            && left
                .native_arguments
                .iter()
                .zip(&right.native_arguments)
                .all(|((left_name, left), (right_name, right))| {
                    left_name == right_name && self.equal(left, right)
                })
    }

    fn equal(&mut self, left: &ValueType, right: &ValueType) -> bool {
        if left != right {
            return false;
        }
        match (left, right) {
            (Object(left), Object(right))
            | (
                InvocationScopedNative { family: left, .. },
                InvocationScopedNative { family: right, .. },
            ) => self.objects_equal(left, right),
            (Union(left), Union(right)) => left
                .iter()
                .zip(right)
                .all(|(left, right)| self.equal(left, right)),
            (Optional(left), Optional(right)) => self.equal(left, right),
            (Iterator(left), Iterator(right))
            | (IterationStep(left), IterationStep(right))
            | (AsyncIterationStep(left), AsyncIterationStep(right))
            | (ChannelPair(left), ChannelPair(right))
            | (ChannelSender(left), ChannelSender(right))
            | (ChannelReceiver(left), ChannelReceiver(right))
            | (ChannelSendOutcome(left), ChannelSendOutcome(right))
            | (ChannelReceiveOutcome(left), ChannelReceiveOutcome(right))
            | (DocumentDecodeOutcome(left), DocumentDecodeOutcome(right))
            | (List(left), List(right))
            | (Set(left), Set(right))
            | (Tuple(left, _), Tuple(right, _))
            | (UnorderedSet(left), UnorderedSet(right))
            | (Task(left, _), Task(right, _))
            | (ScopedTask(left, _), ScopedTask(right, _))
            | (TaskGroup(left), TaskGroup(right))
            | (TaskOutcome(left), TaskOutcome(right))
            | (Reference(left), Reference(right))
            | (MutableReference(left), MutableReference(right))
            | (SharedReference(left), SharedReference(right)) => {
                self.equal(left.value_type_ref(), right.value_type_ref())
            }
            (Map(left_key, left_value), Map(right_key, right_value))
            | (Entry(left_key, left_value), Entry(right_key, right_value))
            | (UnorderedMap(left_key, left_value), UnorderedMap(right_key, right_value)) => {
                self.equal(left_key.value_type_ref(), right_key.value_type_ref())
                    && self.equal(left_value.value_type_ref(), right_value.value_type_ref())
            }
            (
                Function(left_parameters, left_result, left_effects),
                Function(right_parameters, right_result, right_effects),
            )
            | (
                AsyncFunction(left_parameters, left_result, _, left_effects),
                AsyncFunction(right_parameters, right_result, _, right_effects),
            ) => {
                left_parameters
                    .iter()
                    .zip(right_parameters)
                    .all(|(left, right)| self.equal(left.value_type_ref(), right.value_type_ref()))
                    && self.equal(left_result.value_type_ref(), right_result.value_type_ref())
                    && match (&left_effects.upper_bound, &right_effects.upper_bound) {
                        (Some(left), Some(right)) => self.equal(left, right),
                        (None, None) => true,
                        _ => false,
                    }
            }
            // These variants carry no object identities or non-identity data.
            (
                Scalar(_)
                | OverflowResult(_)
                | DivRemResult(_)
                | FloatDecomposition(_)
                | StringView(_)
                | StringList
                | TextRange
                | TextRangeView(_)
                | TextRangeList
                | IterationEnd
                | AsyncSinkOutcome
                | ProjectedGeneric(_)
                | TypeParameter(_)
                | InlineRust
                | ChannelOverflowPolicy
                | DocumentDiagnostic
                | Range
                | Encoding
                | Descriptor(_)
                | TaskScope
                | FilesystemAuthority
                | PlatformFilesystemResult
                | PlatformStreamHandle
                | PlatformOpenResult
                | PlatformReadResult
                | PlatformWriteResult
                | PlatformUnitResult
                | PlatformDataResult
                | PlatformUrlResult
                | ProjectedAssociated
                | PlatformCapability
                | PlatformResourceHandle
                | PlatformResult,
                _,
            ) => true,
            _ => false,
        }
    }
}

pub(crate) fn shares_value_payload(left: &ValueType, right: &ValueType) -> bool {
    ValuePayloadComparison::default().equal(left, right)
}

struct NativeBindingsWire<'a>(&'a ObjectIdentity);

impl serde::Serialize for NativeBindingsWire<'_> {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(Some(self.0.native_arguments.len()))?;
        for (name, value) in &self.0.native_arguments {
            let index = self
                .0
                .type_arguments
                .iter()
                .position(|argument| shares_value_payload(argument, value));
            let binding =
                index.map_or(NativeBindingRef::Value(value), NativeBindingRef::Positional);
            map.serialize_entry(name, &binding)?;
        }
        map.end()
    }
}

impl serde::Serialize for ObjectIdentity {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("ObjectIdentity", 8)?;
        state.serialize_field("namespace", &self.namespace)?;
        state.serialize_field("name", &self.name)?;
        state.serialize_field("is_unsafe", &self.is_unsafe)?;
        state.serialize_field("application", &self.application)?;
        state.serialize_field("type_arguments", &self.type_arguments)?;
        state.serialize_field("native_projection", &self.native_projection)?;
        state.serialize_field("native_arguments", &NativeBindingsWire(self))?;
        state.serialize_field("native_parameters", &self.native_parameters)?;
        state.end()
    }
}

#[derive(serde::Deserialize)]
enum NativeBindingOwned {
    Positional(usize),
    Value(ValueType),
}

impl<'de> serde::Deserialize<'de> for ObjectIdentity {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[derive(serde::Deserialize)]
        struct Wire {
            namespace: String,
            name: String,
            is_unsafe: bool,
            application: Option<Box<ValueType>>,
            type_arguments: Vec<ValueType>,
            native_projection: Option<String>,
            native_arguments: BTreeMap<String, NativeBindingOwned>,
            native_parameters: Vec<String>,
        }
        let wire = Wire::deserialize(deserializer)?;
        let mut native_arguments = BTreeMap::new();
        for (name, binding) in wire.native_arguments {
            let value = match binding {
                NativeBindingOwned::Positional(index) => {
                    wire.type_arguments.get(index).cloned().ok_or_else(|| {
                        serde::de::Error::custom("native binding positional index out of range")
                    })?
                }
                NativeBindingOwned::Value(value) => value,
            };
            native_arguments.insert(name, value);
        }
        let application_key = wire
            .application
            .as_ref()
            .map(|application| format!("{application:?}"));
        let type_arguments_key =
            (!wire.type_arguments.is_empty()).then(|| format!("{:?}", wire.type_arguments));
        let native_arguments_key =
            (!native_arguments.is_empty()).then(|| format!("{native_arguments:?}"));
        Ok(ObjectIdentityData {
            namespace: wire.namespace,
            name: wire.name,
            is_unsafe: wire.is_unsafe,
            application: wire.application,
            application_key,
            type_arguments: wire.type_arguments,
            type_arguments_key,
            native_projection: wire.native_projection,
            native_arguments,
            native_arguments_key,
            native_parameters: wire.native_parameters,
        }
        .into())
    }
}

// Identity keys use semantic values, never cached key strings or derived metadata.
impl std::fmt::Debug for ObjectIdentity {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("ObjectIdentity")
            .field("namespace", &self.namespace)
            .field("name", &self.name)
            .field("is_unsafe", &self.is_unsafe)
            .field("application", &self.application)
            .field("type_arguments", &self.type_arguments)
            .field("native_projection", &self.native_projection)
            .field("native_arguments", &NativeArgumentsDebug(self))
            .finish()
    }
}

impl ObjectIdentity {
    pub(crate) fn new(namespace: impl Into<String>, name: impl Into<String>) -> Self {
        ObjectIdentityData {
            namespace: namespace.into(),
            name: name.into(),
            is_unsafe: false,
            application: None,
            application_key: None,
            type_arguments: Vec::new(),
            type_arguments_key: None,
            native_projection: None,
            native_arguments: BTreeMap::new(),
            native_arguments_key: None,
            native_parameters: Vec::new(),
        }
        .into()
    }

    pub(crate) fn qualified(&self) -> String {
        format!("{}::{}", self.namespace, self)
    }

    pub(crate) fn qualified_base(&self) -> String {
        format!(
            "{}::{}{}",
            self.namespace,
            if self.is_unsafe { "unsafe " } else { "" },
            self.name
        )
    }

    pub(crate) fn base(&self) -> Self {
        Self::new(&self.namespace, &self.name).with_safety(self.is_unsafe)
    }

    pub(crate) fn with_safety(mut self, is_unsafe: bool) -> Self {
        self.is_unsafe = is_unsafe;
        self
    }

    pub(crate) fn with_application(mut self, application: ValueType) -> Self {
        self.application_key = Some(format!("{application:?}"));
        self.application = Some(Box::new(application));
        self
    }
    pub(crate) fn with_type_arguments(mut self, arguments: Vec<ValueType>) -> Self {
        self.type_arguments_key = (!arguments.is_empty()).then(|| format!("{arguments:?}"));
        self.type_arguments = arguments;
        self
    }

    pub(crate) fn with_native_projection(mut self, rust_path: impl Into<String>) -> Self {
        self.native_projection = Some(rust_path.into());
        self
    }

    pub(crate) fn with_native_arguments(mut self, arguments: BTreeMap<String, ValueType>) -> Self {
        self.native_arguments_key = (!arguments.is_empty()).then(|| format!("{arguments:?}"));
        self.native_arguments = arguments;
        self
    }

    pub(crate) fn with_native_parameters(mut self, parameters: Vec<String>) -> Self {
        self.native_parameters = parameters;
        self
    }

    /// Native selections not already represented by source argument slots.
    pub(crate) fn additional_native_arguments(
        &self,
    ) -> impl std::iter::Iterator<Item = &ValueType> {
        self.native_arguments.values().filter(|argument| {
            !self.type_arguments.contains(argument)
                && self.application.as_deref() != Some(*argument)
        })
    }

    pub(crate) fn corresponding_native_argument<'a>(
        &self,
        other: &'a Self,
        name: &str,
    ) -> Option<&'a ValueType> {
        if let Some(argument) = other.native_arguments.get(name) {
            return Some(argument);
        }
        let index = self
            .native_parameters
            .iter()
            .position(|parameter| parameter == name)?;
        if self.type_arguments.get(index) != self.native_arguments.get(name)
            || (!other.native_parameters.is_empty()
                && other.native_parameters.get(index).map(String::as_str) != Some(name))
        {
            return None;
        }
        other.type_arguments.get(index)
    }
}

impl std::fmt::Display for ObjectIdentity {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_unsafe {
            formatter.write_str("unsafe ")?;
        }
        formatter.write_str(&self.name)?;
        if !self.type_arguments.is_empty() {
            formatter.write_str(" of ")?;
            if self.type_arguments.len() > 1 {
                formatter.write_str("(")?;
            }
            for (index, argument) in self.type_arguments.iter().enumerate() {
                if index > 0 {
                    formatter.write_str(", ")?;
                }
                argument.fmt(formatter)?;
            }
            if self.type_arguments.len() > 1 {
                formatter.write_str(")")?;
            }
        }
        if let Some(application) = &self.application {
            write!(formatter, " of {application}")?;
        }
        let mut payloads = self
            .native_arguments
            .values()
            .filter(|value| !self.type_arguments.contains(value))
            .peekable();
        if payloads.peek().is_some() {
            formatter.write_str(" (payload types: ")?;
            for (index, value_type) in payloads.enumerate() {
                if index > 0 {
                    formatter.write_str(", ")?;
                }
                value_type.fmt(formatter)?;
            }
            formatter.write_str(")")?;
        }
        Ok(())
    }
}
impl PartialEq for ObjectIdentity {
    fn eq(&self, other: &Self) -> bool {
        (
            &self.namespace,
            &self.name,
            self.application_key.as_deref(),
            self.type_arguments_key.as_deref(),
            self.is_unsafe,
            self.native_projection.as_deref(),
            self.native_arguments_key.as_deref(),
        ) == (
            &other.namespace,
            &other.name,
            other.application_key.as_deref(),
            other.type_arguments_key.as_deref(),
            other.is_unsafe,
            other.native_projection.as_deref(),
            other.native_arguments_key.as_deref(),
        )
    }
}

impl Eq for ObjectIdentity {}

impl Ord for ObjectIdentity {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (
            &self.namespace,
            &self.name,
            self.application_key.as_deref(),
            self.type_arguments_key.as_deref(),
            self.is_unsafe,
            self.native_projection.as_deref(),
            self.native_arguments_key.as_deref(),
        )
            .cmp(&(
                &other.namespace,
                &other.name,
                other.application_key.as_deref(),
                other.type_arguments_key.as_deref(),
                other.is_unsafe,
                other.native_projection.as_deref(),
                other.native_arguments_key.as_deref(),
            ))
    }
}

impl PartialOrd for ObjectIdentity {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
