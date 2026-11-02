// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/main.trn
// Namespace: app
pub trait CommandProtocol<
    TerraneType72657175657374,
    TerraneType726573706F6E7365,
    TerraneType7472616E73706F7274,
>: Send + Sync {
    fn clone_box(
        &self,
    ) -> Box<
        dyn CommandProtocol<
            TerraneType72657175657374,
            TerraneType726573706F6E7365,
            TerraneType7472616E73706F7274,
        >,
    >;
    fn separate_box(
        &self,
    ) -> Box<
        dyn CommandProtocol<
            TerraneType72657175657374,
            TerraneType726573706F6E7365,
            TerraneType7472616E73706F7274,
        >,
    >;
    fn dispatch(
        &self,
        service: &TerraneType7472616E73706F7274,
        input: TerraneType72657175657374,
    ) -> Result<TerraneType726573706F6E7365, TerraneError>;
}
impl<
    TerraneType72657175657374,
    TerraneType726573706F6E7365,
    TerraneType7472616E73706F7274,
> Clone
for Box<
    dyn CommandProtocol<
        TerraneType72657175657374,
        TerraneType726573706F6E7365,
        TerraneType7472616E73706F7274,
    >,
> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}
pub struct Command<
    TerraneType72657175657374,
    TerraneType726573706F6E7365,
    TerraneType7472616E73706F7274,
>(
    Box<
        dyn CommandProtocol<
            TerraneType72657175657374,
            TerraneType726573706F6E7365,
            TerraneType7472616E73706F7274,
        >,
    >,
);
impl<
    TerraneType72657175657374,
    TerraneType726573706F6E7365,
    TerraneType7472616E73706F7274,
> Clone
for Command<
    TerraneType72657175657374,
    TerraneType726573706F6E7365,
    TerraneType7472616E73706F7274,
> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl<
    TerraneType72657175657374,
    TerraneType726573706F6E7365,
    TerraneType7472616E73706F7274,
> Command<
    TerraneType72657175657374,
    TerraneType726573706F6E7365,
    TerraneType7472616E73706F7274,
> {
    pub fn dispatch(
        &self,
        service: &TerraneType7472616E73706F7274,
        input: TerraneType72657175657374,
    ) -> Result<TerraneType726573706F6E7365, TerraneError> {
        self.0.dispatch(service, input)
    }
}
#[derive(Clone)]
pub struct AlphaCommand {}
impl AlphaCommand {
    pub fn terrane_construct() -> Self {
        Self {}
    }
    pub fn dispatch(
        &self,
        service: &Executor,
        input: Alpha,
    ) -> Result<AlphaReply, TerraneError> {
        return Ok(
            __terrane_raised_err(
                match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| (&*service).call(input)),
                ) {
                    Ok(Ok(value)) => Ok(value),
                    Ok(Err(error)) => {
                        Err(
                            crate::TerraneForeignError(
                                crate::TerraneError::custom_raised(
                                    crate::DescriptorId(2),
                                    error.to_string(),
                                    crate::TERRANE_NO_SITE,
                                ),
                            ),
                        )
                    }
                    Err(payload) => {
                        Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "witness",
                                "witness::Executor::call",
                            ),
                        )
                    }
                },
                0 /* terrane-site: src/main.trn:24:16-24:40 */,
            )?,
        );
    }
    pub fn free_dispatch(&self, input: Alpha) -> Result<AlphaReply, TerraneError> {
        return Ok(
            __terrane_raised_err(
                match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| witness::execute(input)),
                ) {
                    Ok(Ok(value)) => Ok(value),
                    Ok(Err(error)) => {
                        Err(
                            crate::TerraneForeignError(
                                crate::TerraneError::custom_raised(
                                    crate::DescriptorId(2),
                                    error.to_string(),
                                    crate::TERRANE_NO_SITE,
                                ),
                            ),
                        )
                    }
                    Err(payload) => {
                        Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "witness",
                                "witness::execute",
                            ),
                        )
                    }
                },
                1 /* terrane-site: src/main.trn:26:16-26:35 */,
            )?,
        );
    }
    pub fn static_dispatch(&self, input: Alpha) -> Result<AlphaReply, TerraneError> {
        return Ok(
            __terrane_raised_err(
                match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| witness::Executor::send(input)),
                ) {
                    Ok(Ok(value)) => Ok(value),
                    Ok(Err(error)) => {
                        Err(
                            crate::TerraneForeignError(
                                crate::TerraneError::custom_raised(
                                    crate::DescriptorId(2),
                                    error.to_string(),
                                    crate::TERRANE_NO_SITE,
                                ),
                            ),
                        )
                    }
                    Err(payload) => {
                        Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "witness",
                                "witness::Executor::send",
                            ),
                        )
                    }
                },
                2 /* terrane-site: src/main.trn:28:16-28:42 */,
            )?,
        );
    }
}
impl CommandProtocol<witness::Alpha, witness::AlphaReply, witness::Executor>
for AlphaCommand {
    fn clone_box(
        &self,
    ) -> Box<
        dyn CommandProtocol<witness::Alpha, witness::AlphaReply, witness::Executor>,
    > {
        Box::new(self.clone())
    }
    fn separate_box(
        &self,
    ) -> Box<
        dyn CommandProtocol<witness::Alpha, witness::AlphaReply, witness::Executor>,
    > {
        Box::new(self.clone())
    }
    fn dispatch(
        &self,
        service: &witness::Executor,
        input: witness::Alpha,
    ) -> Result<witness::AlphaReply, TerraneError> {
        AlphaCommand::dispatch(&*self, service, input)
    }
}
impl From<AlphaCommand>
for Command<witness::Alpha, witness::AlphaReply, witness::Executor> {
    fn from(value: AlphaCommand) -> Self {
        Self(Box::new(value))
    }
}
#[derive(Clone)]
pub struct BetaCommand {}
impl BetaCommand {
    pub fn terrane_construct() -> Self {
        Self {}
    }
    pub fn dispatch(
        &self,
        service: &Executor,
        input: Beta,
    ) -> Result<SharedReply, TerraneError> {
        return Ok(
            __terrane_raised_err(
                match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| (&*service).call(input)),
                ) {
                    Ok(Ok(value)) => Ok(value),
                    Ok(Err(error)) => {
                        Err(
                            crate::TerraneForeignError(
                                crate::TerraneError::custom_raised(
                                    crate::DescriptorId(2),
                                    error.to_string(),
                                    crate::TERRANE_NO_SITE,
                                ),
                            ),
                        )
                    }
                    Err(payload) => {
                        Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "witness",
                                "witness::Executor::call",
                            ),
                        )
                    }
                },
                3 /* terrane-site: src/main.trn:35:16-35:40 */,
            )?,
        );
    }
}
impl CommandProtocol<witness::Beta, protocol::SharedReply, witness::Executor>
for BetaCommand {
    fn clone_box(
        &self,
    ) -> Box<
        dyn CommandProtocol<witness::Beta, protocol::SharedReply, witness::Executor>,
    > {
        Box::new(self.clone())
    }
    fn separate_box(
        &self,
    ) -> Box<
        dyn CommandProtocol<witness::Beta, protocol::SharedReply, witness::Executor>,
    > {
        Box::new(self.clone())
    }
    fn dispatch(
        &self,
        service: &witness::Executor,
        input: witness::Beta,
    ) -> Result<protocol::SharedReply, TerraneError> {
        BetaCommand::dispatch(&*self, service, input)
    }
}
impl From<BetaCommand>
for Command<witness::Beta, protocol::SharedReply, witness::Executor> {
    fn from(value: BetaCommand) -> Self {
        Self(Box::new(value))
    }
}
#[derive(Clone)]
pub struct SharedCommand {}
impl SharedCommand {
    pub fn terrane_construct() -> Self {
        Self {}
    }
    pub fn dispatch(
        &self,
        service: &std::sync::Arc<witness::Executor>,
        input: Alpha,
    ) -> AlphaReply {
        return __terrane_raised(
            match std::panic::catch_unwind(
                std::panic::AssertUnwindSafe(|| (&*service).call(input)),
            ) {
                Ok(Ok(value)) => Ok(value),
                Ok(Err(error)) => {
                    Err(
                        crate::TerraneForeignError(
                            crate::TerraneError::custom_raised(
                                crate::DescriptorId(2),
                                error.to_string(),
                                crate::TERRANE_NO_SITE,
                            ),
                        ),
                    )
                }
                Err(payload) => {
                    Err(
                        crate::__terrane_dependency_panic(
                            payload,
                            "witness",
                            "witness::Executor::call",
                        ),
                    )
                }
            },
            4 /* terrane-site: src/main.trn:42:16-42:40 */,
        );
    }
}
impl CommandProtocol<
    witness::Alpha,
    witness::AlphaReply,
    std::sync::Arc<witness::Executor>,
> for SharedCommand {
    fn clone_box(
        &self,
    ) -> Box<
        dyn CommandProtocol<
            witness::Alpha,
            witness::AlphaReply,
            std::sync::Arc<witness::Executor>,
        >,
    > {
        Box::new(self.clone())
    }
    fn separate_box(
        &self,
    ) -> Box<
        dyn CommandProtocol<
            witness::Alpha,
            witness::AlphaReply,
            std::sync::Arc<witness::Executor>,
        >,
    > {
        Box::new(self.clone())
    }
    fn dispatch(
        &self,
        service: &std::sync::Arc<witness::Executor>,
        input: witness::Alpha,
    ) -> Result<witness::AlphaReply, TerraneError> {
        Ok(SharedCommand::dispatch(&*self, service, input))
    }
}
impl From<SharedCommand>
for Command<witness::Alpha, witness::AlphaReply, std::sync::Arc<witness::Executor>> {
    fn from(value: SharedCommand) -> Self {
        Self(Box::new(value))
    }
}
#[derive(Clone)]
pub struct NestedCommand {}
impl NestedCommand {
    pub fn terrane_construct() -> Self {
        Self {}
    }
    pub fn dispatch(
        &self,
        service: &Executor,
        input: Maybe,
    ) -> Result<Option<terrane_collection_support::List<AlphaReply>>, TerraneError> {
        return Ok(
            __terrane_raised_err(
                match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| (&*service).call(input)),
                ) {
                    Ok(Ok(value)) => {
                        Ok(
                            value
                                .map(|value| terrane_collection_support::List::new(value)),
                        )
                    }
                    Ok(Err(error)) => {
                        Err(
                            crate::TerraneForeignError(
                                crate::TerraneError::custom_raised(
                                    crate::DescriptorId(2),
                                    error.to_string(),
                                    crate::TERRANE_NO_SITE,
                                ),
                            ),
                        )
                    }
                    Err(payload) => {
                        Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "witness",
                                "witness::Executor::call",
                            ),
                        )
                    }
                },
                5 /* terrane-site: src/main.trn:49:16-49:40 */,
            )?,
        );
    }
}
impl CommandProtocol<
    witness::Maybe,
    Option<terrane_collection_support::List<witness::AlphaReply>>,
    witness::Executor,
> for NestedCommand {
    fn clone_box(
        &self,
    ) -> Box<
        dyn CommandProtocol<
            witness::Maybe,
            Option<terrane_collection_support::List<witness::AlphaReply>>,
            witness::Executor,
        >,
    > {
        Box::new(self.clone())
    }
    fn separate_box(
        &self,
    ) -> Box<
        dyn CommandProtocol<
            witness::Maybe,
            Option<terrane_collection_support::List<witness::AlphaReply>>,
            witness::Executor,
        >,
    > {
        Box::new(self.clone())
    }
    fn dispatch(
        &self,
        service: &witness::Executor,
        input: witness::Maybe,
    ) -> Result<
        Option<terrane_collection_support::List<witness::AlphaReply>>,
        TerraneError,
    > {
        NestedCommand::dispatch(&*self, service, input)
    }
}
impl From<NestedCommand>
for Command<
    witness::Maybe,
    Option<terrane_collection_support::List<witness::AlphaReply>>,
    witness::Executor,
> {
    fn from(value: NestedCommand) -> Self {
        Self(Box::new(value))
    }
}
#[derive(Clone)]
pub struct EchoCommand {}
impl EchoCommand {
    pub fn terrane_construct() -> Self {
        Self {}
    }
    pub fn dispatch(
        &self,
        service: &Executor,
        input: witness::Echo<String>,
    ) -> Result<witness::EchoReply<String>, TerraneError> {
        return Ok(
            __terrane_raised_err(
                match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| (&*service).call(input)),
                ) {
                    Ok(Ok(value)) => Ok(value),
                    Ok(Err(error)) => {
                        Err(
                            crate::TerraneForeignError(
                                crate::TerraneError::custom_raised(
                                    crate::DescriptorId(2),
                                    error.to_string(),
                                    crate::TERRANE_NO_SITE,
                                ),
                            ),
                        )
                    }
                    Err(payload) => {
                        Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "witness",
                                "witness::Executor::call",
                            ),
                        )
                    }
                },
                6 /* terrane-site: src/main.trn:56:16-56:40 */,
            )?,
        );
    }
}
impl CommandProtocol<
    witness::Echo<String>,
    witness::EchoReply<String>,
    witness::Executor,
> for EchoCommand {
    fn clone_box(
        &self,
    ) -> Box<
        dyn CommandProtocol<
            witness::Echo<String>,
            witness::EchoReply<String>,
            witness::Executor,
        >,
    > {
        Box::new(self.clone())
    }
    fn separate_box(
        &self,
    ) -> Box<
        dyn CommandProtocol<
            witness::Echo<String>,
            witness::EchoReply<String>,
            witness::Executor,
        >,
    > {
        Box::new(self.clone())
    }
    fn dispatch(
        &self,
        service: &witness::Executor,
        input: witness::Echo<String>,
    ) -> Result<witness::EchoReply<String>, TerraneError> {
        EchoCommand::dispatch(&*self, service, input)
    }
}
impl From<EchoCommand>
for Command<witness::Echo<String>, witness::EchoReply<String>, witness::Executor> {
    fn from(value: EchoCommand) -> Self {
        Self(Box::new(value))
    }
}
#[derive(Clone)]
pub struct AlternateCommand {}
impl AlternateCommand {
    pub fn terrane_construct() -> Self {
        Self {}
    }
    pub fn dispatch(&self, service: &Executor, input: Alpha) -> SharedReply {
        let _ = &service;
        return __terrane_raised(
            match std::panic::catch_unwind(
                std::panic::AssertUnwindSafe(|| witness::alternate(input)),
            ) {
                Ok(value) => Ok(value),
                Err(payload) => {
                    Err(
                        crate::__terrane_dependency_panic(
                            payload,
                            "witness",
                            "witness::alternate",
                        ),
                    )
                }
            },
            7 /* terrane-site: src/main.trn:63:16-63:37 */,
        );
    }
}
impl CommandProtocol<witness::Alpha, protocol::SharedReply, witness::Executor>
for AlternateCommand {
    fn clone_box(
        &self,
    ) -> Box<
        dyn CommandProtocol<witness::Alpha, protocol::SharedReply, witness::Executor>,
    > {
        Box::new(self.clone())
    }
    fn separate_box(
        &self,
    ) -> Box<
        dyn CommandProtocol<witness::Alpha, protocol::SharedReply, witness::Executor>,
    > {
        Box::new(self.clone())
    }
    fn dispatch(
        &self,
        service: &witness::Executor,
        input: witness::Alpha,
    ) -> Result<protocol::SharedReply, TerraneError> {
        Ok(AlternateCommand::dispatch(&*self, service, input))
    }
}
impl From<AlternateCommand>
for Command<witness::Alpha, protocol::SharedReply, witness::Executor> {
    fn from(value: AlternateCommand) -> Self {
        Self(Box::new(value))
    }
}
#[derive(Clone)]
pub struct UniqueCommand {}
impl UniqueCommand {
    pub fn terrane_construct() -> Self {
        Self {}
    }
    pub fn dispatch(
        &self,
        service: &Executor,
        input: Unique,
    ) -> Result<UniqueReply, TerraneError> {
        return Ok(
            __terrane_raised_err(
                match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| (&*service).call(input)),
                ) {
                    Ok(Ok(value)) => Ok(value),
                    Ok(Err(error)) => {
                        Err(
                            crate::TerraneForeignError(
                                crate::TerraneError::custom_raised(
                                    crate::DescriptorId(2),
                                    error.to_string(),
                                    crate::TERRANE_NO_SITE,
                                ),
                            ),
                        )
                    }
                    Err(payload) => {
                        Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "witness",
                                "witness::Executor::call",
                            ),
                        )
                    }
                },
                8 /* terrane-site: src/main.trn:70:16-70:40 */,
            )?,
        );
    }
}
impl CommandProtocol<witness::Unique, witness::UniqueReply, witness::Executor>
for UniqueCommand {
    fn clone_box(
        &self,
    ) -> Box<
        dyn CommandProtocol<witness::Unique, witness::UniqueReply, witness::Executor>,
    > {
        Box::new(self.clone())
    }
    fn separate_box(
        &self,
    ) -> Box<
        dyn CommandProtocol<witness::Unique, witness::UniqueReply, witness::Executor>,
    > {
        Box::new(self.clone())
    }
    fn dispatch(
        &self,
        service: &witness::Executor,
        input: witness::Unique,
    ) -> Result<witness::UniqueReply, TerraneError> {
        UniqueCommand::dispatch(&*self, service, input)
    }
}
impl From<UniqueCommand>
for Command<witness::Unique, witness::UniqueReply, witness::Executor> {
    fn from(value: UniqueCommand) -> Self {
        Self(Box::new(value))
    }
}
#[derive(Clone)]
pub struct TaggedCommand {}
impl TaggedCommand {
    pub fn terrane_construct() -> Self {
        Self {}
    }
    pub fn dispatch(
        &self,
        service: &Executor,
        input: Tagged,
    ) -> Result<TaggedReply, TerraneError> {
        return Ok(
            __terrane_raised_err(
                match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| (&*service).call(input)),
                ) {
                    Ok(Ok(value)) => Ok(value),
                    Ok(Err(error)) => {
                        Err(
                            crate::TerraneForeignError(
                                crate::TerraneError::custom_raised(
                                    crate::DescriptorId(2),
                                    error.to_string(),
                                    crate::TERRANE_NO_SITE,
                                ),
                            ),
                        )
                    }
                    Err(payload) => {
                        Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "witness",
                                "witness::Executor::call",
                            ),
                        )
                    }
                },
                9 /* terrane-site: src/main.trn:77:16-77:40 */,
            )?,
        );
    }
}
impl CommandProtocol<witness::Tagged, witness::TaggedReply, witness::Executor>
for TaggedCommand {
    fn clone_box(
        &self,
    ) -> Box<
        dyn CommandProtocol<witness::Tagged, witness::TaggedReply, witness::Executor>,
    > {
        Box::new(self.clone())
    }
    fn separate_box(
        &self,
    ) -> Box<
        dyn CommandProtocol<witness::Tagged, witness::TaggedReply, witness::Executor>,
    > {
        Box::new(self.clone())
    }
    fn dispatch(
        &self,
        service: &witness::Executor,
        input: witness::Tagged,
    ) -> Result<witness::TaggedReply, TerraneError> {
        TaggedCommand::dispatch(&*self, service, input)
    }
}
impl From<TaggedCommand>
for Command<witness::Tagged, witness::TaggedReply, witness::Executor> {
    fn from(value: TaggedCommand) -> Self {
        Self(Box::new(value))
    }
}
#[derive(Clone)]
pub struct TreeCommand {}
impl TreeCommand {
    pub fn terrane_construct() -> Self {
        Self {}
    }
    pub fn dispatch(
        &self,
        service: &Executor,
        input: TreeRequest,
    ) -> Result<Tree, TerraneError> {
        return Ok(
            __terrane_raised_err(
                match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| (&*service).call(input)),
                ) {
                    Ok(Ok(value)) => Ok(value),
                    Ok(Err(error)) => {
                        Err(
                            crate::TerraneForeignError(
                                crate::TerraneError::custom_raised(
                                    crate::DescriptorId(2),
                                    error.to_string(),
                                    crate::TERRANE_NO_SITE,
                                ),
                            ),
                        )
                    }
                    Err(payload) => {
                        Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "witness",
                                "witness::Executor::call",
                            ),
                        )
                    }
                },
                10 /* terrane-site: src/main.trn:84:16-84:40 */,
            )?,
        );
    }
}
impl CommandProtocol<witness::TreeRequest, witness::Tree, witness::Executor>
for TreeCommand {
    fn clone_box(
        &self,
    ) -> Box<
        dyn CommandProtocol<witness::TreeRequest, witness::Tree, witness::Executor>,
    > {
        Box::new(self.clone())
    }
    fn separate_box(
        &self,
    ) -> Box<
        dyn CommandProtocol<witness::TreeRequest, witness::Tree, witness::Executor>,
    > {
        Box::new(self.clone())
    }
    fn dispatch(
        &self,
        service: &witness::Executor,
        input: witness::TreeRequest,
    ) -> Result<witness::Tree, TerraneError> {
        TreeCommand::dispatch(&*self, service, input)
    }
}
impl From<TreeCommand>
for Command<witness::TreeRequest, witness::Tree, witness::Executor> {
    fn from(value: TreeCommand) -> Self {
        Self(Box::new(value))
    }
}
pub trait RetainedProviderProtocol<TerraneType726573706F6E7365>: Send + Sync {
    fn read(&self) -> TerraneType726573706F6E7365;
}
pub struct RetainedProvider<TerraneType726573706F6E7365>(
    Box<dyn RetainedProviderProtocol<TerraneType726573706F6E7365>>,
);
impl<TerraneType726573706F6E7365> RetainedProvider<TerraneType726573706F6E7365> {
    pub fn read(&self) -> TerraneType726573706F6E7365 {
        self.0.read()
    }
}
pub struct RetainedResource {
    pub payload: Option<Unique>,
}
impl RetainedResource {
    pub fn terrane_construct(value: Unique) -> Self {
        let mut __terrane_constructed_value = Self { payload: None };
        __terrane_constructed_value.construct(value);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, value: Unique) {
        {
            let __terrane_value = value;
            self.payload = Some(__terrane_value);
        }
    }
    pub fn read(&self) -> String {
        let payload: &Unique;
        payload = self.payload.as_ref().expect("required field initialized");
        return {
            let __terrane_snapshot = payload.text.clone();
            __terrane_snapshot
        };
    }
}
impl RetainedProviderProtocol<String> for RetainedResource {
    fn read(&self) -> String {
        RetainedResource::read(&*self)
    }
}
impl From<RetainedResource> for RetainedProvider<String> {
    fn from(value: RetainedResource) -> Self {
        Self(Box::new(value))
    }
}
fn main() {
    let executor: Executor;
    let alpha: AlphaCommand;
    let first: AlphaReply;
    let beta: BetaCommand;
    let second: SharedReply;
    let third: AlphaReply;
    let fourth: AlphaReply;
    let shared: ArcOfTerraneAssociatedCallExecutor;
    let pointer: SharedCommand;
    let through_pointer: AlphaReply;
    let nested: NestedCommand;
    let values: Option<terrane_collection_support::List<AlphaReply>>;
    let missing: Option<terrane_collection_support::List<AlphaReply>>;
    let echo: EchoCommand;
    let generic: witness::EchoReply<String>;
    let other: AlternateCommand;
    let differently_bound: SharedReply;
    let unique: UniqueCommand;
    let owned_result: UniqueReply;
    let reusable: Alpha;
    let inferred: witness::AlphaReply;
    let tagged: TaggedCommand;
    let with_type_field: TaggedReply;
    let held: RetainedProvider<String>;
    let tree_command_value: TreeCommand;
    let tree_result: Tree;
    let leaf: Tree;
    let branch: Tree;
    let children: Option<terrane_collection_support::List<Tree>>;
    let absent: Tree;
    let mut failure: Option<TerraneError> = None;
    let selected_mode: Option<Mode>;
    let missing_mode: Option<Mode>;
    executor = __terrane_raised(
        terrane_static_trn_4578656375746f72_new(),
        11 /* terrane-site: src/main.trn:100:16-100:30 */,
    );
    alpha = AlphaCommand::terrane_construct();
    first = __terrane_traced(
        alpha
            .dispatch(
                &executor,
                witness::Alpha {
                    value: match || -> Result<_, crate::TerraneForeignError> {
                        Ok(
                            terrane_int_support::coerce::<
                                i64,
                            >(&terrane_int_support::Int::from(40_i128))
                                .map_err(|error| crate::TerraneForeignError(
                                    crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
                                ))?,
                        )
                    }() {
                        Ok(value) => value,
                        Err(error) => std::panic::panic_any(error),
                    },
                },
            ),
        12 /* terrane-site: src/main.trn:102:13-102:67 */,
    );
    println!("{}", terrane_scalar_support::scalar_text(&first.value));
    beta = BetaCommand::terrane_construct();
    second = __terrane_traced(
        beta
            .dispatch(
                &executor,
                witness::Beta {
                    text: String::from("selected"),
                },
            ),
        13 /* terrane-site: src/main.trn:105:14-105:73 */,
    );
    println!("{}", terrane_scalar_support::scalar_text(&second.label));
    third = __terrane_traced(
        alpha
            .free_dispatch(witness::Alpha {
                value: match || -> Result<_, crate::TerraneForeignError> {
                    Ok(
                        terrane_int_support::coerce::<
                            i64,
                        >(&terrane_int_support::Int::from(41_i128))
                            .map_err(|error| crate::TerraneForeignError(
                                crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
                            ))?,
                    )
                }() {
                    Ok(value) => value,
                    Err(error) => std::panic::panic_any(error),
                },
            }),
        14 /* terrane-site: src/main.trn:107:13-107:62 */,
    );
    println!("{}", terrane_scalar_support::scalar_text(&third.value));
    fourth = __terrane_traced(
        alpha
            .static_dispatch(witness::Alpha {
                value: match || -> Result<_, crate::TerraneForeignError> {
                    Ok(
                        terrane_int_support::coerce::<
                            i64,
                        >(&terrane_int_support::Int::from(42_i128))
                            .map_err(|error| crate::TerraneForeignError(
                                crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
                            ))?,
                    )
                }() {
                    Ok(value) => value,
                    Err(error) => std::panic::panic_any(error),
                },
            }),
        15 /* terrane-site: src/main.trn:109:14-109:65 */,
    );
    println!("{}", terrane_scalar_support::scalar_text(&fourth.value));
    shared = __terrane_raised(
        shared_executor(),
        16 /* terrane-site: src/main.trn:111:14-111:30 */,
    );
    pointer = SharedCommand::terrane_construct();
    through_pointer = pointer
        .dispatch(
            &shared,
            witness::Alpha {
                value: match || -> Result<_, crate::TerraneForeignError> {
                    Ok(
                        terrane_int_support::coerce::<
                            i64,
                        >(&terrane_int_support::Int::from(43_i128))
                            .map_err(|error| crate::TerraneForeignError(
                                crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
                            ))?,
                    )
                }() {
                    Ok(value) => value,
                    Err(error) => std::panic::panic_any(error),
                },
            },
        );
    println!("{}", terrane_scalar_support::scalar_text(&through_pointer.value));
    nested = NestedCommand::terrane_construct();
    values = __terrane_traced(
        nested
            .dispatch(
                &executor,
                witness::Maybe {
                    value: match || -> Result<_, crate::TerraneForeignError> {
                        Ok(
                            terrane_int_support::coerce::<
                                i64,
                            >(&terrane_int_support::Int::from(44_i128))
                                .map_err(|error| crate::TerraneForeignError(
                                    crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
                                ))?,
                        )
                    }() {
                        Ok(value) => value,
                        Err(error) => std::panic::panic_any(error),
                    },
                },
            ),
        17 /* terrane-site: src/main.trn:116:14-116:69 */,
    );
    if (&values).is_some() {
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_raised(match &values {
            Some(value) => value, _ => unreachable!("flow-proven storage refinement") }
            .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
            18 /* terrane-site: src/main.trn:118:16-118:25 */)),
            18 /* terrane-site: src/main.trn:118:16-118:25 */).value)
        );
    }
    missing = __terrane_traced(
        nested
            .dispatch(
                &executor,
                witness::Maybe {
                    value: match || -> Result<_, crate::TerraneForeignError> {
                        Ok(
                            terrane_int_support::coerce::<
                                i64,
                            >(&terrane_int_support::Int::from(-1_i128))
                                .map_err(|error| crate::TerraneForeignError(
                                    crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
                                ))?,
                        )
                    }() {
                        Ok(value) => value,
                        Err(error) => std::panic::panic_any(error),
                    },
                },
            ),
        19 /* terrane-site: src/main.trn:119:15-119:70 */,
    );
    println!("{}", terrane_scalar_support::scalar_text(&(&missing).is_none()));
    echo = EchoCommand::terrane_construct();
    generic = __terrane_traced(
        echo
            .dispatch(
                &executor,
                __terrane_raised(
                    match std::panic::catch_unwind(
                        std::panic::AssertUnwindSafe(|| witness::Echo::<String> {
                            value: String::from("generic"),
                        }),
                    ) {
                        Ok(value) => Ok(value),
                        Err(payload) => {
                            Err(
                                crate::__terrane_dependency_panic(
                                    payload,
                                    "witness",
                                    "witness::Echo::construct",
                                ),
                            )
                        }
                    },
                    20 /* terrane-site: src/main.trn:122:41-122:73 */,
                ),
            ),
        21 /* terrane-site: src/main.trn:122:15-122:74 */,
    );
    println!("{}", terrane_scalar_support::scalar_text(&generic.value));
    other = AlternateCommand::terrane_construct();
    differently_bound = other
        .dispatch(
            &executor,
            witness::Alpha {
                value: match || -> Result<_, crate::TerraneForeignError> {
                    Ok(
                        terrane_int_support::coerce::<
                            i64,
                        >(&terrane_int_support::Int::from(45_i128))
                            .map_err(|error| crate::TerraneForeignError(
                                crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
                            ))?,
                    )
                }() {
                    Ok(value) => value,
                    Err(error) => std::panic::panic_any(error),
                },
            },
        );
    println!("{}", terrane_scalar_support::scalar_text(&differently_bound.label));
    unique = UniqueCommand::terrane_construct();
    owned_result = __terrane_traced(
        unique
            .dispatch(
                &executor,
                witness::Unique {
                    text: String::from("unique"),
                },
            ),
        22 /* terrane-site: src/main.trn:128:20-128:81 */,
    );
    println!("{}", terrane_scalar_support::scalar_text(&owned_result.text));
    reusable = witness::Alpha {
        value: match || -> Result<_, crate::TerraneForeignError> {
            Ok(
                terrane_int_support::coerce::<
                    i64,
                >(&terrane_int_support::Int::from(46_i128))
                    .map_err(|error| crate::TerraneForeignError(
                        crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
                    ))?,
            )
        }() {
            Ok(value) => value,
            Err(error) => std::panic::panic_any(error),
        },
    };
    inferred = __terrane_raised(
        match std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| witness::execute(reusable.clone())),
        ) {
            Ok(Ok(value)) => Ok(value),
            Ok(Err(error)) => {
                Err(
                    crate::TerraneForeignError(
                        crate::TerraneError::custom_raised(
                            crate::DescriptorId(2),
                            error.to_string(),
                            crate::TERRANE_NO_SITE,
                        ),
                    ),
                )
            }
            Err(payload) => {
                Err(
                    crate::__terrane_dependency_panic(
                        payload,
                        "witness",
                        "witness::execute",
                    ),
                )
            }
        },
        23 /* terrane-site: src/main.trn:131:16-131:33 */,
    );
    println!("{}", terrane_scalar_support::scalar_text(&inferred.value));
    println!("{}", terrane_scalar_support::scalar_text(&reusable.value));
    tagged = TaggedCommand::terrane_construct();
    with_type_field = __terrane_traced(
        tagged
            .dispatch(
                &executor,
                witness::Tagged {
                    value: match || -> Result<_, crate::TerraneForeignError> {
                        Ok(
                            terrane_int_support::coerce::<
                                i64,
                            >(&terrane_int_support::Int::from(48_i128))
                                .map_err(|error| crate::TerraneForeignError(
                                    crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
                                ))?,
                        )
                    }() {
                        Ok(value) => value,
                        Err(error) => std::panic::panic_any(error),
                    },
                },
            ),
        24 /* terrane-site: src/main.trn:135:23-135:79 */,
    );
    println!("{}", terrane_scalar_support::scalar_text(&with_type_field.r#type));
    held = <RetainedProvider<
        String,
    >>::from(
        RetainedResource::terrane_construct(witness::Unique {
            text: String::from("held"),
        }),
    );
    println!("{}", terrane_scalar_support::scalar_text(&held.read()));
    println!("{}", terrane_scalar_support::scalar_text(&held.read()));
    tree_command_value = TreeCommand::terrane_construct();
    tree_result = __terrane_traced(
        tree_command_value
            .dispatch(
                &executor,
                witness::TreeRequest {
                    value: match || -> Result<_, crate::TerraneForeignError> {
                        Ok(
                            terrane_int_support::coerce::<
                                i64,
                            >(&terrane_int_support::Int::from(49_i128))
                                .map_err(|error| crate::TerraneForeignError(
                                    crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
                                ))?,
                        )
                    }() {
                        Ok(value) => value,
                        Err(error) => std::panic::panic_any(error),
                    },
                },
            ),
        25 /* terrane-site: src/main.trn:141:19-141:92 */,
    );
    println!("{}", terrane_scalar_support::scalar_text(&tree_result.value));
    leaf = witness::Tree {
        value: match || -> Result<_, crate::TerraneForeignError> {
            Ok(
                terrane_int_support::coerce::<
                    i64,
                >(&terrane_int_support::Int::from(-2_i128))
                    .map_err(|error| crate::TerraneForeignError(
                        crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
                    ))?,
            )
        }() {
            Ok(value) => value,
            Err(error) => std::panic::panic_any(error),
        },
        children: match || -> Result<_, crate::TerraneForeignError> {
            Ok(
                {
                    let __terrane_field_argument: Option<
                        terrane_collection_support::List<Tree>,
                    > = None;
                    __terrane_field_argument
                }
                    .map(|value| -> Result<_, crate::TerraneForeignError> {
                        Ok(value.into_vec())
                    })
                    .transpose()?,
            )
        }() {
            Ok(value) => value,
            Err(error) => std::panic::panic_any(error),
        }
            .into(),
    };
    println!("{}", terrane_scalar_support::scalar_text(&leaf.value));
    branch = witness::Tree {
        value: match || -> Result<_, crate::TerraneForeignError> {
            Ok(
                terrane_int_support::coerce::<
                    i64,
                >(&terrane_int_support::Int::from(50_i128))
                    .map_err(|error| crate::TerraneForeignError(
                        crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
                    ))?,
            )
        }() {
            Ok(value) => value,
            Err(error) => std::panic::panic_any(error),
        },
        children: match || -> Result<_, crate::TerraneForeignError> {
            Ok(
                {
                    let __terrane_field_argument: Option<
                        terrane_collection_support::List<Tree>,
                    > = Some(terrane_collection_support::List::<Tree>::new(vec![leaf]));
                    __terrane_field_argument
                }
                    .map(|value| -> Result<_, crate::TerraneForeignError> {
                        Ok(value.into_vec())
                    })
                    .transpose()?,
            )
        }() {
            Ok(value) => value,
            Err(error) => std::panic::panic_any(error),
        }
            .into(),
    };
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(child_count(&branch),
        26 /* terrane-site: src/main.trn:146:13-146:32 */))
    );
    children = (&branch.children)
        .as_ref()
        .map(|__terrane_field_item| terrane_collection_support::List::new(
            __terrane_field_item
                .iter()
                .map(|__terrane_field_item| (*__terrane_field_item).clone())
                .collect(),
        ))
        .clone();
    if (&children).is_some() {
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_raised(match &children {
            Some(value) => value, _ => unreachable!("flow-proven storage refinement") }
            .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
            27 /* terrane-site: src/main.trn:149:16-149:27 */)),
            27 /* terrane-site: src/main.trn:149:16-149:27 */).value)
        );
    }
    absent = witness::Tree {
        value: match || -> Result<_, crate::TerraneForeignError> {
            Ok(
                terrane_int_support::coerce::<
                    i64,
                >(&terrane_int_support::Int::from(51_i128))
                    .map_err(|error| crate::TerraneForeignError(
                        crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
                    ))?,
            )
        }() {
            Ok(value) => value,
            Err(error) => std::panic::panic_any(error),
        },
        children: None::<std::vec::Vec<witness::Tree>>,
    };
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(child_count(&absent),
        28 /* terrane-site: src/main.trn:151:13-151:32 */))
    );
    let __terrane_completion_0: TerraneCompletion<()> = (|| {
        let __terrane_try_0: TerraneCompletion<()> = (|| {
            __terrane_traced_completion!(
                alpha.dispatch(&executor, witness::Alpha { value : match | | -> Result <
                _, crate ::TerraneForeignError > { Ok(terrane_int_support::coerce:: < i64
                > (&terrane_int_support::Int::from(- 1_i128)).map_err(| error | crate
                ::TerraneForeignError(crate ::TerraneRaised::raised(error, crate
                ::TERRANE_NO_SITE))) ?) } () { Ok(value) => value, Err(error) =>
                std::panic::panic_any(error) } }), 29 /* terrane-site: src/main.trn:153:9-153:63 */
            );
            TerraneCompletion::Normal
        })();
        match __terrane_try_0 {
            TerraneCompletion::Return(value) => return TerraneCompletion::Return(value),
            TerraneCompletion::Break => return TerraneCompletion::Break,
            TerraneCompletion::Continue => return TerraneCompletion::Continue,
            TerraneCompletion::Normal => {}
            TerraneCompletion::Error(__terrane_error_0) => {
                let mut __terrane_handled_0 = false;
                if !__terrane_handled_0
                    && __terrane_error_0.kind
                        == TerraneErrorKind::Custom(DescriptorId(2))
                {
                    __terrane_handled_0 = true;
                    let _ = failure.insert(__terrane_error_0.clone());
                    println!(
                        "{}", terrane_scalar_support::scalar_text(&failure.as_ref()
                        .expect("flow-proven available binding").clone().message()
                        .to_owned())
                    );
                }
                if !__terrane_handled_0 {
                    return TerraneCompletion::Error(__terrane_error_0);
                }
            }
        }
        TerraneCompletion::Normal
    })();
    match __terrane_completion_0 {
        TerraneCompletion::Normal => {}
        TerraneCompletion::Return(value) => return value,
        TerraneCompletion::Error(error) => __terrane_uncaught(error),
        TerraneCompletion::Break | TerraneCompletion::Continue => {
            __terrane_generated_defect("loop control escaped a non-loop try")
        }
    }
    selected_mode = __terrane_raised(
        mode(true),
        30 /* terrane-site: src/main.trn:156:21-156:31 */,
    );
    if (&selected_mode).is_some() {
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let __terrane_comparison_left =
            match &selected_mode { Some(value) => value, _ =>
            unreachable!("flow-proven storage refinement") }; let
            __terrane_comparison_right = __terrane_raised(match
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| |
            witness::Mode::First)) { Ok(value) => Ok(value), Err(payload) => Err(crate
            ::__terrane_dependency_panic(payload, "witness", "witness::Mode::First")) },
            31 /* terrane-site: src/main.trn:158:34-158:46 */); < Mode as
            std::cmp::PartialEq > ::eq(&__terrane_comparison_left,
            &__terrane_comparison_right) })
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let __terrane_comparison_left =
            __terrane_raised(match
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| |
            witness::Mode::First)) { Ok(value) => Ok(value), Err(payload) => Err(crate
            ::__terrane_dependency_panic(payload, "witness", "witness::Mode::First")) },
            32 /* terrane-site: src/main.trn:159:17-159:29 */); let
            __terrane_comparison_right = match &selected_mode { Some(value) => value, _
            => unreachable!("flow-proven storage refinement") }; < Mode as
            std::cmp::PartialEq > ::eq(&__terrane_comparison_left,
            &__terrane_comparison_right) })
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let __terrane_comparison_left =
            match &selected_mode { Some(value) => value, _ =>
            unreachable!("flow-proven storage refinement") }; let
            __terrane_comparison_right = __terrane_raised(match
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| |
            witness::Mode::Last)) { Ok(value) => Ok(value), Err(payload) => Err(crate
            ::__terrane_dependency_panic(payload, "witness", "witness::Mode::Last")) },
            33 /* terrane-site: src/main.trn:160:34-160:45 */); < Mode as
            std::cmp::PartialEq > ::ne(&__terrane_comparison_left,
            &__terrane_comparison_right) })
        );
    }
    missing_mode = __terrane_raised(
        mode(false),
        34 /* terrane-site: src/main.trn:161:20-161:31 */,
    );
    println!("{}", terrane_scalar_support::scalar_text(&(&missing_mode).is_none()));
    check_numeric_fields();
    __terrane_traced(
        check_recipe_selection(&executor),
        35 /* terrane-site: src/main.trn:164:5-164:37 */,
    );
}
#[derive(Clone)]
pub struct BatchCommand {}
impl BatchCommand {
    pub fn terrane_construct() -> Self {
        Self {}
    }
    pub fn dispatch(
        &self,
        service: &Executor,
        input: witness::Batch<std::vec::Vec<String>>,
    ) -> Result<String, TerraneError> {
        return Ok(
            __terrane_raised_err(
                match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| (&*service).call(input)),
                ) {
                    Ok(Ok(value)) => Ok(value),
                    Ok(Err(error)) => {
                        Err(
                            crate::TerraneForeignError(
                                crate::TerraneError::custom_raised(
                                    crate::DescriptorId(2),
                                    error.to_string(),
                                    crate::TERRANE_NO_SITE,
                                ),
                            ),
                        )
                    }
                    Err(payload) => {
                        Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "witness",
                                "witness::Executor::call",
                            ),
                        )
                    }
                },
                36 /* terrane-site: src/main.trn:171:16-171:40 */,
            )?,
        );
    }
}
impl CommandProtocol<witness::Batch<std::vec::Vec<String>>, String, witness::Executor>
for BatchCommand {
    fn clone_box(
        &self,
    ) -> Box<
        dyn CommandProtocol<
            witness::Batch<std::vec::Vec<String>>,
            String,
            witness::Executor,
        >,
    > {
        Box::new(self.clone())
    }
    fn separate_box(
        &self,
    ) -> Box<
        dyn CommandProtocol<
            witness::Batch<std::vec::Vec<String>>,
            String,
            witness::Executor,
        >,
    > {
        Box::new(self.clone())
    }
    fn dispatch(
        &self,
        service: &witness::Executor,
        input: witness::Batch<std::vec::Vec<String>>,
    ) -> Result<String, TerraneError> {
        BatchCommand::dispatch(&*self, service, input)
    }
}
impl From<BatchCommand>
for Command<witness::Batch<std::vec::Vec<String>>, String, witness::Executor> {
    fn from(value: BatchCommand) -> Self {
        Self(Box::new(value))
    }
}
#[derive(Clone)]
pub struct MarkedCommand {}
impl MarkedCommand {
    pub fn terrane_construct() -> Self {
        Self {}
    }
    pub fn dispatch(
        &self,
        service: &Executor,
        input: Marked,
    ) -> Result<bool, TerraneError> {
        return Ok(
            __terrane_raised_err(
                match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| (&*service).call(input)),
                ) {
                    Ok(Ok(value)) => Ok(value),
                    Ok(Err(error)) => {
                        Err(
                            crate::TerraneForeignError(
                                crate::TerraneError::custom_raised(
                                    crate::DescriptorId(2),
                                    error.to_string(),
                                    crate::TERRANE_NO_SITE,
                                ),
                            ),
                        )
                    }
                    Err(payload) => {
                        Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "witness",
                                "witness::Executor::call",
                            ),
                        )
                    }
                },
                37 /* terrane-site: src/main.trn:178:16-178:40 */,
            )?,
        );
    }
}
impl CommandProtocol<witness::Marked, bool, witness::Executor> for MarkedCommand {
    fn clone_box(
        &self,
    ) -> Box<dyn CommandProtocol<witness::Marked, bool, witness::Executor>> {
        Box::new(self.clone())
    }
    fn separate_box(
        &self,
    ) -> Box<dyn CommandProtocol<witness::Marked, bool, witness::Executor>> {
        Box::new(self.clone())
    }
    fn dispatch(
        &self,
        service: &witness::Executor,
        input: witness::Marked,
    ) -> Result<bool, TerraneError> {
        MarkedCommand::dispatch(&*self, service, input)
    }
}
impl From<MarkedCommand> for Command<witness::Marked, bool, witness::Executor> {
    fn from(value: MarkedCommand) -> Self {
        Self(Box::new(value))
    }
}
#[derive(Clone)]
pub struct BorrowedOutputCommand {}
impl BorrowedOutputCommand {
    pub fn terrane_construct() -> Self {
        Self {}
    }
    pub fn dispatch(
        &self,
        service: &Executor,
        input: BorrowedOutput,
    ) -> Result<AlphaReply, TerraneError> {
        return Ok(
            __terrane_raised_err(
                match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| (&*service).call(input)),
                ) {
                    Ok(Ok(value)) => Ok(value),
                    Ok(Err(error)) => {
                        Err(
                            crate::TerraneForeignError(
                                crate::TerraneError::custom_raised(
                                    crate::DescriptorId(2),
                                    error.to_string(),
                                    crate::TERRANE_NO_SITE,
                                ),
                            ),
                        )
                    }
                    Err(payload) => {
                        Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "witness",
                                "witness::Executor::call",
                            ),
                        )
                    }
                },
                38 /* terrane-site: src/main.trn:185:16-185:40 */,
            )?,
        );
    }
}
impl CommandProtocol<witness::BorrowedOutput, witness::AlphaReply, witness::Executor>
for BorrowedOutputCommand {
    fn clone_box(
        &self,
    ) -> Box<
        dyn CommandProtocol<
            witness::BorrowedOutput,
            witness::AlphaReply,
            witness::Executor,
        >,
    > {
        Box::new(self.clone())
    }
    fn separate_box(
        &self,
    ) -> Box<
        dyn CommandProtocol<
            witness::BorrowedOutput,
            witness::AlphaReply,
            witness::Executor,
        >,
    > {
        Box::new(self.clone())
    }
    fn dispatch(
        &self,
        service: &witness::Executor,
        input: witness::BorrowedOutput,
    ) -> Result<witness::AlphaReply, TerraneError> {
        BorrowedOutputCommand::dispatch(&*self, service, input)
    }
}
impl From<BorrowedOutputCommand>
for Command<witness::BorrowedOutput, witness::AlphaReply, witness::Executor> {
    fn from(value: BorrowedOutputCommand) -> Self {
        Self(Box::new(value))
    }
}
fn optional_number(
    present: bool,
    value: terrane_int_support::Int,
) -> Option<terrane_int_support::Int> {
    if present {
        return Some(value.clone());
    }
    return None;
}
fn check_numeric_fields() {
    let mut fields: NumericFields;
    let selected: Option<terrane_int_support::Int>;
    let absent: Option<terrane_int_support::Int>;
    let invalid: Option<terrane_int_support::Int>;
    fields = witness::NumericFields {
        quality: {
            let __terrane_field_argument: Option<u32> = None;
            __terrane_field_argument
        }
            .into(),
        ratio: {
            let __terrane_field_argument: Option<f64> = None;
            __terrane_field_argument
        }
            .into(),
    };
    selected = optional_number(true, terrane_int_support::Int::from(73_i128));
    {
        let __terrane_value = match selected.clone() {
            Some(__terrane_optional_value) => {
                Some(
                    __terrane_raised(
                        terrane_int_support::coerce::<u32>(&__terrane_optional_value),
                        39 /* terrane-site: src/main.trn:195:22-195:30 */,
                    ),
                )
            }
            None => None,
        };
        fields.quality = __terrane_value;
    }
    {
        let __terrane_value = match selected.clone() {
            Some(__terrane_optional_value) => {
                Some(
                    __terrane_raised(
                        terrane_int_support::exact_f64(&__terrane_optional_value),
                        40 /* terrane-site: src/main.trn:196:20-196:28 */,
                    ),
                )
            }
            None => None,
        };
        fields.ratio = __terrane_value;
    }
    println!(
        "{}", { let mut __terrane_text = String::new(); match &fields.quality {
        Some(__terrane_present) => {
        terrane_scalar_support::ScalarDisplay::write_scalar(&__terrane_present, &mut
        __terrane_text); }, None => __terrane_text.push_str("none"), } __terrane_text }
    );
    println!(
        "{}", { let mut __terrane_text = String::new(); match &fields.ratio {
        Some(__terrane_present) => {
        terrane_scalar_support::ScalarDisplay::write_scalar(&__terrane_present, &mut
        __terrane_text); }, None => __terrane_text.push_str("none"), } __terrane_text }
    );
    absent = optional_number(false, terrane_int_support::Int::from(0_i128));
    {
        let __terrane_value = match absent {
            Some(__terrane_optional_value) => {
                Some(
                    __terrane_raised(
                        terrane_int_support::coerce::<u32>(&__terrane_optional_value),
                        41 /* terrane-site: src/main.trn:200:22-200:28 */,
                    ),
                )
            }
            None => None,
        };
        fields.quality = __terrane_value;
    }
    println!("{}", terrane_scalar_support::scalar_text(&(&fields.quality).is_none()));
    {
        let __terrane_value = match selected {
            Some(__terrane_optional_value) => {
                Some(
                    __terrane_raised(
                        terrane_int_support::coerce::<u32>(&__terrane_optional_value),
                        42 /* terrane-site: src/main.trn:202:22-202:30 */,
                    ),
                )
            }
            None => None,
        };
        fields.quality = __terrane_value;
    }
    invalid = optional_number(true, terrane_int_support::Int::from(-1_i128));
    let __terrane_completion_1: TerraneCompletion<()> = (|| {
        let __terrane_try_1: TerraneCompletion<()> = (|| {
            {
                let __terrane_value = match invalid {
                    Some(__terrane_optional_value) => {
                        Some(
                            __terrane_raised_completion!(
                                terrane_int_support::coerce:: < u32 >
                                (&__terrane_optional_value), 43 /* terrane-site: src/main.trn:205:26-205:33 */
                            ),
                        )
                    }
                    None => None,
                };
                fields.quality = __terrane_value;
            }
            TerraneCompletion::Normal
        })();
        match __terrane_try_1 {
            TerraneCompletion::Return(value) => return TerraneCompletion::Return(value),
            TerraneCompletion::Break => return TerraneCompletion::Break,
            TerraneCompletion::Continue => return TerraneCompletion::Continue,
            TerraneCompletion::Normal => {}
            TerraneCompletion::Error(__terrane_error_1) => {
                let mut __terrane_handled_1 = false;
                if !__terrane_handled_1
                    && __terrane_error_1.kind
                        == TerraneErrorKind::IntegerConversionOverflow
                {
                    __terrane_handled_1 = true;
                    println!(
                        "{}", { let mut __terrane_text = String::new(); match &fields
                        .quality { Some(__terrane_present) => {
                        terrane_scalar_support::ScalarDisplay::write_scalar(&__terrane_present,
                        &mut __terrane_text); }, None => __terrane_text.push_str("none"),
                        } __terrane_text }
                    );
                }
                if !__terrane_handled_1 {
                    return TerraneCompletion::Error(__terrane_error_1);
                }
            }
        }
        TerraneCompletion::Normal
    })();
    match __terrane_completion_1 {
        TerraneCompletion::Normal => {}
        TerraneCompletion::Return(value) => return value,
        TerraneCompletion::Error(error) => __terrane_uncaught(error),
        TerraneCompletion::Break | TerraneCompletion::Continue => {
            __terrane_generated_defect("loop control escaped a non-loop try")
        }
    }
}
fn check_recipe_selection(executor: &Executor) -> Result<(), TerraneError> {
    let batching: BatchCommand;
    let marked_value: MarkedCommand;
    let borrowed_value: BorrowedOutputCommand;
    let result: AlphaReply;
    let borrowed_input: Alpha;
    let borrowed_loan: &Alpha;
    let borrowed_result: protocol::SharedReply;
    let set_request: witness::Batch<std::collections::BTreeSet<String>>;
    let map_request: witness::Batch<
        std::collections::BTreeMap<String, witness::AlphaReply>,
    >;
    let map_result: witness::AlphaReply;
    batching = BatchCommand::terrane_construct();
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_traced_err(batching
        .dispatch(executor, __terrane_raised_err(batch(), 44 /* terrane-site: src/main.trn:211:43-211:49 */) ?), 45 /* terrane-site: src/main.trn:211:13-211:50 */) ?)
    );
    marked_value = MarkedCommand::terrane_construct();
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_traced_err(marked_value
        .dispatch(executor, __terrane_raised_err(marked(), 46 /* terrane-site: src/main.trn:213:47-213:54 */) ?), 47 /* terrane-site: src/main.trn:213:13-213:55 */) ?)
    );
    borrowed_value = BorrowedOutputCommand::terrane_construct();
    result = __terrane_traced_err(
        borrowed_value
            .dispatch(
                executor,
                __terrane_raised_err(
                    borrowed_output(),
                    48 /* terrane-site: src/main.trn:215:50-215:66 */,
                )?,
            ),
        49 /* terrane-site: src/main.trn:215:14-215:67 */,
    )?;
    println!("{}", terrane_scalar_support::scalar_text(&result.value));
    borrowed_input = witness::Alpha {
        value: match || -> Result<_, crate::TerraneForeignError> {
            Ok(
                terrane_int_support::coerce::<
                    i64,
                >(&terrane_int_support::Int::from(93_i128))
                    .map_err(|error| crate::TerraneForeignError(
                        crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
                    ))?,
            )
        }() {
            Ok(value) => value,
            Err(error) => std::panic::panic_any(error),
        },
    };
    borrowed_loan = &borrowed_input;
    borrowed_result = __terrane_raised_err(
        match std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| witness::execute(&*borrowed_loan)),
        ) {
            Ok(Ok(value)) => Ok(value),
            Ok(Err(error)) => {
                Err(
                    crate::TerraneForeignError(
                        crate::TerraneError::custom_raised(
                            crate::DescriptorId(2),
                            error.to_string(),
                            crate::TERRANE_NO_SITE,
                        ),
                    ),
                )
            }
            Err(payload) => {
                Err(
                    crate::__terrane_dependency_panic(
                        payload,
                        "witness",
                        "witness::execute",
                    ),
                )
            }
        },
        50 /* terrane-site: src/main.trn:219:23-219:45 */,
    )?;
    println!("{}", terrane_scalar_support::scalar_text(&borrowed_result.label));
    set_request = __terrane_raised_err(
        set_batch(),
        51 /* terrane-site: src/main.trn:221:19-221:29 */,
    )?;
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised_err(match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| |
        witness::execute(set_request))) { Ok(Ok(value)) =>
        Ok(terrane_int_support::Int::from_u128(value as u128)), Ok(Err(error)) =>
        Err(crate ::TerraneForeignError(crate ::TerraneError::custom_raised(crate
        ::DescriptorId(2), error.to_string(), crate ::TERRANE_NO_SITE))), Err(payload) =>
        Err(crate ::__terrane_dependency_panic(payload, "witness", "witness::execute"))
        }, 52 /* terrane-site: src/main.trn:222:13-222:38 */) ?)
    );
    map_request = __terrane_raised_err(
        mapped_batch(),
        53 /* terrane-site: src/main.trn:223:19-223:32 */,
    )?;
    map_result = __terrane_raised_err(
        match std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| witness::execute(map_request)),
        ) {
            Ok(Ok(value)) => Ok(value),
            Ok(Err(error)) => {
                Err(
                    crate::TerraneForeignError(
                        crate::TerraneError::custom_raised(
                            crate::DescriptorId(2),
                            error.to_string(),
                            crate::TERRANE_NO_SITE,
                        ),
                    ),
                )
            }
            Err(payload) => {
                Err(
                    crate::__terrane_dependency_panic(
                        payload,
                        "witness",
                        "witness::execute",
                    ),
                )
            }
        },
        54 /* terrane-site: src/main.trn:224:18-224:43 */,
    )?;
    println!("{}", terrane_scalar_support::scalar_text(&map_result.value));
    Ok(())
}
