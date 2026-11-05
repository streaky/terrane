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
        return Ok({
            let __terrane_conversion_input_0 = input;
            __terrane_raised_err(
                || -> Result<_, crate::TerraneForeignError> {
                    match match std::panic::catch_unwind(
                        std::panic::AssertUnwindSafe(|| -> Result<
                            _,
                            crate::TerraneForeignError,
                        > { Ok((&*service).call(__terrane_conversion_input_0)) }),
                    ) {
                        Ok(result) => result?,
                        Err(payload) => {
                            return Err(
                                crate::__terrane_dependency_panic(
                                    payload,
                                    "witness",
                                    "witness::Executor::call",
                                ),
                            );
                        }
                    } {
                        Ok(value) => Ok(value),
                        Err(error) => {
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
                    }
                }(),
                0 /* terrane-site: src/main.trn:24:16-24:40 */,
            )?
        });
    }
    pub fn free_dispatch(&self, input: Alpha) -> Result<AlphaReply, TerraneError> {
        return Ok({
            let __terrane_conversion_input_0 = input;
            __terrane_raised_err(
                || -> Result<_, crate::TerraneForeignError> {
                    match match std::panic::catch_unwind(
                        std::panic::AssertUnwindSafe(|| -> Result<
                            _,
                            crate::TerraneForeignError,
                        > { Ok(witness::execute(__terrane_conversion_input_0)) }),
                    ) {
                        Ok(result) => result?,
                        Err(payload) => {
                            return Err(
                                crate::__terrane_dependency_panic(
                                    payload,
                                    "witness",
                                    "witness::execute",
                                ),
                            );
                        }
                    } {
                        Ok(value) => Ok(value),
                        Err(error) => {
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
                    }
                }(),
                1 /* terrane-site: src/main.trn:26:16-26:35 */,
            )?
        });
    }
    pub fn static_dispatch(&self, input: Alpha) -> Result<AlphaReply, TerraneError> {
        return Ok({
            let __terrane_conversion_input_0 = input;
            __terrane_raised_err(
                || -> Result<_, crate::TerraneForeignError> {
                    match match std::panic::catch_unwind(
                        std::panic::AssertUnwindSafe(|| -> Result<
                            _,
                            crate::TerraneForeignError,
                        > { Ok(witness::Executor::send(__terrane_conversion_input_0)) }),
                    ) {
                        Ok(result) => result?,
                        Err(payload) => {
                            return Err(
                                crate::__terrane_dependency_panic(
                                    payload,
                                    "witness",
                                    "witness::Executor::send",
                                ),
                            );
                        }
                    } {
                        Ok(value) => Ok(value),
                        Err(error) => {
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
                    }
                }(),
                2 /* terrane-site: src/main.trn:28:16-28:42 */,
            )?
        });
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
        return Ok({
            let __terrane_conversion_input_0 = input;
            __terrane_raised_err(
                || -> Result<_, crate::TerraneForeignError> {
                    match match std::panic::catch_unwind(
                        std::panic::AssertUnwindSafe(|| -> Result<
                            _,
                            crate::TerraneForeignError,
                        > { Ok((&*service).call(__terrane_conversion_input_0)) }),
                    ) {
                        Ok(result) => result?,
                        Err(payload) => {
                            return Err(
                                crate::__terrane_dependency_panic(
                                    payload,
                                    "witness",
                                    "witness::Executor::call",
                                ),
                            );
                        }
                    } {
                        Ok(value) => Ok(value),
                        Err(error) => {
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
                    }
                }(),
                3 /* terrane-site: src/main.trn:35:16-35:40 */,
            )?
        });
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
            || -> Result<_, crate::TerraneForeignError> {
                match match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| -> Result<
                        _,
                        crate::TerraneForeignError,
                    > { Ok((&*service).call(input)) }),
                ) {
                    Ok(result) => result?,
                    Err(payload) => {
                        return Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "witness",
                                "witness::Executor::call",
                            ),
                        );
                    }
                } {
                    Ok(value) => Ok(value),
                    Err(error) => {
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
                }
            }(),
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
        return Ok({
            let __terrane_conversion_input_0 = input;
            __terrane_raised_err(
                || -> Result<_, crate::TerraneForeignError> {
                    match match std::panic::catch_unwind(
                        std::panic::AssertUnwindSafe(|| -> Result<
                            _,
                            crate::TerraneForeignError,
                        > { Ok((&*service).call(__terrane_conversion_input_0)) }),
                    ) {
                        Ok(result) => result?,
                        Err(payload) => {
                            return Err(
                                crate::__terrane_dependency_panic(
                                    payload,
                                    "witness",
                                    "witness::Executor::call",
                                ),
                            );
                        }
                    } {
                        Ok(value) => {
                            Ok(
                                value
                                    .map(|value| terrane_collection_support::List::new(value)),
                            )
                        }
                        Err(error) => {
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
                    }
                }(),
                5 /* terrane-site: src/main.trn:49:16-49:40 */,
            )?
        });
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
        return Ok({
            let __terrane_conversion_input_0 = input;
            __terrane_raised_err(
                || -> Result<_, crate::TerraneForeignError> {
                    match match std::panic::catch_unwind(
                        std::panic::AssertUnwindSafe(|| -> Result<
                            _,
                            crate::TerraneForeignError,
                        > { Ok((&*service).call(__terrane_conversion_input_0)) }),
                    ) {
                        Ok(result) => result?,
                        Err(payload) => {
                            return Err(
                                crate::__terrane_dependency_panic(
                                    payload,
                                    "witness",
                                    "witness::Executor::call",
                                ),
                            );
                        }
                    } {
                        Ok(value) => Ok(value),
                        Err(error) => {
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
                    }
                }(),
                6 /* terrane-site: src/main.trn:56:16-56:40 */,
            )?
        });
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
        return {
            let __terrane_conversion_input_0 = input;
            __terrane_raised(
                || -> Result<_, crate::TerraneForeignError> {
                    {
                        let value = match std::panic::catch_unwind(
                            std::panic::AssertUnwindSafe(|| -> Result<
                                _,
                                crate::TerraneForeignError,
                            > { Ok(witness::alternate(__terrane_conversion_input_0)) }),
                        ) {
                            Ok(result) => result?,
                            Err(payload) => {
                                return Err(
                                    crate::__terrane_dependency_panic(
                                        payload,
                                        "witness",
                                        "witness::alternate",
                                    ),
                                );
                            }
                        };
                        Ok::<_, crate::TerraneForeignError>(value)
                    }
                }(),
                7 /* terrane-site: src/main.trn:63:16-63:37 */,
            )
        };
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
        return Ok({
            let __terrane_conversion_input_0 = input;
            __terrane_raised_err(
                || -> Result<_, crate::TerraneForeignError> {
                    match match std::panic::catch_unwind(
                        std::panic::AssertUnwindSafe(|| -> Result<
                            _,
                            crate::TerraneForeignError,
                        > { Ok((&*service).call(__terrane_conversion_input_0)) }),
                    ) {
                        Ok(result) => result?,
                        Err(payload) => {
                            return Err(
                                crate::__terrane_dependency_panic(
                                    payload,
                                    "witness",
                                    "witness::Executor::call",
                                ),
                            );
                        }
                    } {
                        Ok(value) => Ok(value),
                        Err(error) => {
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
                    }
                }(),
                8 /* terrane-site: src/main.trn:70:16-70:40 */,
            )?
        });
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
        return Ok({
            let __terrane_conversion_input_0 = input;
            __terrane_raised_err(
                || -> Result<_, crate::TerraneForeignError> {
                    match match std::panic::catch_unwind(
                        std::panic::AssertUnwindSafe(|| -> Result<
                            _,
                            crate::TerraneForeignError,
                        > { Ok((&*service).call(__terrane_conversion_input_0)) }),
                    ) {
                        Ok(result) => result?,
                        Err(payload) => {
                            return Err(
                                crate::__terrane_dependency_panic(
                                    payload,
                                    "witness",
                                    "witness::Executor::call",
                                ),
                            );
                        }
                    } {
                        Ok(value) => Ok(value),
                        Err(error) => {
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
                    }
                }(),
                9 /* terrane-site: src/main.trn:77:16-77:40 */,
            )?
        });
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
        return Ok({
            let __terrane_conversion_input_0 = input;
            __terrane_raised_err(
                || -> Result<_, crate::TerraneForeignError> {
                    match match std::panic::catch_unwind(
                        std::panic::AssertUnwindSafe(|| -> Result<
                            _,
                            crate::TerraneForeignError,
                        > { Ok((&*service).call(__terrane_conversion_input_0)) }),
                    ) {
                        Ok(result) => result?,
                        Err(payload) => {
                            return Err(
                                crate::__terrane_dependency_panic(
                                    payload,
                                    "witness",
                                    "witness::Executor::call",
                                ),
                            );
                        }
                    } {
                        Ok(value) => Ok(value),
                        Err(error) => {
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
                    }
                }(),
                10 /* terrane-site: src/main.trn:84:16-84:40 */,
            )?
        });
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
    let shared: std::sync::Arc<witness::Executor>;
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
                {
                    let __terrane_conversion_input_0 = terrane_int_support::Int::from(
                        40_i128,
                    );
                    __terrane_raised(
                        || -> Result<_, crate::TerraneForeignError> {
                            Ok(witness::Alpha {
                                value: terrane_int_support::coerce::<
                                    i64,
                                >(&__terrane_conversion_input_0)
                                    .map_err(|error| crate::TerraneForeignError(
                                        crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
                                    ))?,
                            })
                        }(),
                        12 /* terrane-site: src/main.trn:102:40-102:66 */,
                    )
                },
            ),
        13 /* terrane-site: src/main.trn:102:13-102:67 */,
    );
    println!("{}", terrane_scalar_support::scalar_text(&first.value));
    beta = BetaCommand::terrane_construct();
    second = __terrane_traced(
        beta
            .dispatch(
                &executor,
                {
                    let __terrane_conversion_input_0 = String::from("selected");
                    __terrane_raised(
                        || -> Result<_, crate::TerraneForeignError> {
                            Ok(witness::Beta {
                                text: __terrane_conversion_input_0,
                            })
                        }(),
                        14 /* terrane-site: src/main.trn:105:40-105:72 */,
                    )
                },
            ),
        15 /* terrane-site: src/main.trn:105:14-105:73 */,
    );
    println!("{}", terrane_scalar_support::scalar_text(&second.label));
    third = __terrane_traced(
        alpha
            .free_dispatch({
                let __terrane_conversion_input_0 = terrane_int_support::Int::from(
                    41_i128,
                );
                __terrane_raised(
                    || -> Result<_, crate::TerraneForeignError> {
                        Ok(witness::Alpha {
                            value: terrane_int_support::coerce::<
                                i64,
                            >(&__terrane_conversion_input_0)
                                .map_err(|error| crate::TerraneForeignError(
                                    crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
                                ))?,
                        })
                    }(),
                    16 /* terrane-site: src/main.trn:107:35-107:61 */,
                )
            }),
        17 /* terrane-site: src/main.trn:107:13-107:62 */,
    );
    println!("{}", terrane_scalar_support::scalar_text(&third.value));
    fourth = __terrane_traced(
        alpha
            .static_dispatch({
                let __terrane_conversion_input_0 = terrane_int_support::Int::from(
                    42_i128,
                );
                __terrane_raised(
                    || -> Result<_, crate::TerraneForeignError> {
                        Ok(witness::Alpha {
                            value: terrane_int_support::coerce::<
                                i64,
                            >(&__terrane_conversion_input_0)
                                .map_err(|error| crate::TerraneForeignError(
                                    crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
                                ))?,
                        })
                    }(),
                    18 /* terrane-site: src/main.trn:109:38-109:64 */,
                )
            }),
        19 /* terrane-site: src/main.trn:109:14-109:65 */,
    );
    println!("{}", terrane_scalar_support::scalar_text(&fourth.value));
    shared = __terrane_raised(
        shared_executor(),
        20 /* terrane-site: src/main.trn:111:14-111:30 */,
    );
    pointer = SharedCommand::terrane_construct();
    through_pointer = pointer
        .dispatch(
            &shared,
            {
                let __terrane_conversion_input_0 = terrane_int_support::Int::from(
                    43_i128,
                );
                __terrane_raised(
                    || -> Result<_, crate::TerraneForeignError> {
                        Ok(witness::Alpha {
                            value: terrane_int_support::coerce::<
                                i64,
                            >(&__terrane_conversion_input_0)
                                .map_err(|error| crate::TerraneForeignError(
                                    crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
                                ))?,
                        })
                    }(),
                    21 /* terrane-site: src/main.trn:113:50-113:76 */,
                )
            },
        );
    println!("{}", terrane_scalar_support::scalar_text(&through_pointer.value));
    nested = NestedCommand::terrane_construct();
    values = __terrane_traced(
        nested
            .dispatch(
                &executor,
                {
                    let __terrane_conversion_input_0 = terrane_int_support::Int::from(
                        44_i128,
                    );
                    __terrane_raised(
                        || -> Result<_, crate::TerraneForeignError> {
                            Ok(witness::Maybe {
                                value: terrane_int_support::coerce::<
                                    i64,
                                >(&__terrane_conversion_input_0)
                                    .map_err(|error| crate::TerraneForeignError(
                                        crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
                                    ))?,
                            })
                        }(),
                        22 /* terrane-site: src/main.trn:116:42-116:68 */,
                    )
                },
            ),
        23 /* terrane-site: src/main.trn:116:14-116:69 */,
    );
    if (&values).is_some() {
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_raised(match &values {
            Some(value) => value, _ => unreachable!("flow-proven storage refinement") }
            .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
            24 /* terrane-site: src/main.trn:118:16-118:25 */)),
            24 /* terrane-site: src/main.trn:118:16-118:25 */).value)
        );
    }
    missing = __terrane_traced(
        nested
            .dispatch(
                &executor,
                {
                    let __terrane_conversion_input_0 = terrane_int_support::Int::from(
                        -1_i128,
                    );
                    __terrane_raised(
                        || -> Result<_, crate::TerraneForeignError> {
                            Ok(witness::Maybe {
                                value: terrane_int_support::coerce::<
                                    i64,
                                >(&__terrane_conversion_input_0)
                                    .map_err(|error| crate::TerraneForeignError(
                                        crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
                                    ))?,
                            })
                        }(),
                        25 /* terrane-site: src/main.trn:119:43-119:69 */,
                    )
                },
            ),
        26 /* terrane-site: src/main.trn:119:15-119:70 */,
    );
    println!("{}", terrane_scalar_support::scalar_text(&(&missing).is_none()));
    echo = EchoCommand::terrane_construct();
    generic = __terrane_traced(
        echo
            .dispatch(
                &executor,
                {
                    let __terrane_conversion_input_0 = String::from("generic");
                    __terrane_raised(
                        || -> Result<_, crate::TerraneForeignError> {
                            Ok(witness::Echo::<String> {
                                value: __terrane_conversion_input_0,
                            })
                        }(),
                        27 /* terrane-site: src/main.trn:122:41-122:73 */,
                    )
                },
            ),
        28 /* terrane-site: src/main.trn:122:15-122:74 */,
    );
    println!("{}", terrane_scalar_support::scalar_text(&generic.value));
    other = AlternateCommand::terrane_construct();
    differently_bound = other
        .dispatch(
            &executor,
            {
                let __terrane_conversion_input_0 = terrane_int_support::Int::from(
                    45_i128,
                );
                __terrane_raised(
                    || -> Result<_, crate::TerraneForeignError> {
                        Ok(witness::Alpha {
                            value: terrane_int_support::coerce::<
                                i64,
                            >(&__terrane_conversion_input_0)
                                .map_err(|error| crate::TerraneForeignError(
                                    crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
                                ))?,
                        })
                    }(),
                    29 /* terrane-site: src/main.trn:125:52-125:78 */,
                )
            },
        );
    println!("{}", terrane_scalar_support::scalar_text(&differently_bound.label));
    unique = UniqueCommand::terrane_construct();
    owned_result = __terrane_traced(
        unique
            .dispatch(
                &executor,
                {
                    let __terrane_conversion_input_0 = String::from("unique");
                    __terrane_raised(
                        || -> Result<_, crate::TerraneForeignError> {
                            Ok(witness::Unique {
                                text: __terrane_conversion_input_0,
                            })
                        }(),
                        30 /* terrane-site: src/main.trn:128:48-128:80 */,
                    )
                },
            ),
        31 /* terrane-site: src/main.trn:128:20-128:81 */,
    );
    println!("{}", terrane_scalar_support::scalar_text(&owned_result.text));
    reusable = {
        let __terrane_conversion_input_0 = terrane_int_support::Int::from(46_i128);
        __terrane_raised(
            || -> Result<_, crate::TerraneForeignError> {
                Ok(witness::Alpha {
                    value: terrane_int_support::coerce::<
                        i64,
                    >(&__terrane_conversion_input_0)
                        .map_err(|error| crate::TerraneForeignError(
                            crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
                        ))?,
                })
            }(),
            32 /* terrane-site: src/main.trn:130:16-130:42 */,
        )
    };
    inferred = {
        let __terrane_conversion_input_0 = reusable.clone();
        __terrane_raised(
            || -> Result<_, crate::TerraneForeignError> {
                match match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| -> Result<
                        _,
                        crate::TerraneForeignError,
                    > { Ok(witness::execute(__terrane_conversion_input_0)) }),
                ) {
                    Ok(result) => result?,
                    Err(payload) => {
                        return Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "witness",
                                "witness::execute",
                            ),
                        );
                    }
                } {
                    Ok(value) => Ok(value),
                    Err(error) => {
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
                }
            }(),
            33 /* terrane-site: src/main.trn:131:16-131:33 */,
        )
    };
    println!("{}", terrane_scalar_support::scalar_text(&inferred.value));
    println!("{}", terrane_scalar_support::scalar_text(&reusable.value));
    tagged = TaggedCommand::terrane_construct();
    with_type_field = __terrane_traced(
        tagged
            .dispatch(
                &executor,
                {
                    let __terrane_conversion_input_0 = terrane_int_support::Int::from(
                        48_i128,
                    );
                    __terrane_raised(
                        || -> Result<_, crate::TerraneForeignError> {
                            Ok(witness::Tagged {
                                value: terrane_int_support::coerce::<
                                    i64,
                                >(&__terrane_conversion_input_0)
                                    .map_err(|error| crate::TerraneForeignError(
                                        crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
                                    ))?,
                            })
                        }(),
                        34 /* terrane-site: src/main.trn:135:51-135:78 */,
                    )
                },
            ),
        35 /* terrane-site: src/main.trn:135:23-135:79 */,
    );
    println!("{}", terrane_scalar_support::scalar_text(&with_type_field.r#type));
    held = <RetainedProvider<
        String,
    >>::from(
        RetainedResource::terrane_construct({
            let __terrane_conversion_input_0 = String::from("held");
            __terrane_raised(
                || -> Result<_, crate::TerraneForeignError> {
                    Ok(witness::Unique {
                        text: __terrane_conversion_input_0,
                    })
                }(),
                36 /* terrane-site: src/main.trn:137:77-137:107 */,
            )
        }),
    );
    println!("{}", terrane_scalar_support::scalar_text(&held.read()));
    println!("{}", terrane_scalar_support::scalar_text(&held.read()));
    tree_command_value = TreeCommand::terrane_construct();
    tree_result = __terrane_traced(
        tree_command_value
            .dispatch(
                &executor,
                {
                    let __terrane_conversion_input_0 = terrane_int_support::Int::from(
                        49_i128,
                    );
                    __terrane_raised(
                        || -> Result<_, crate::TerraneForeignError> {
                            Ok(witness::TreeRequest {
                                value: terrane_int_support::coerce::<
                                    i64,
                                >(&__terrane_conversion_input_0)
                                    .map_err(|error| crate::TerraneForeignError(
                                        crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
                                    ))?,
                            })
                        }(),
                        37 /* terrane-site: src/main.trn:141:59-141:91 */,
                    )
                },
            ),
        38 /* terrane-site: src/main.trn:141:19-141:92 */,
    );
    println!("{}", terrane_scalar_support::scalar_text(&tree_result.value));
    leaf = {
        let __terrane_conversion_input_0 = terrane_int_support::Int::from(-2_i128);
        let __terrane_conversion_input_1 = None;
        __terrane_raised(
            || -> Result<_, crate::TerraneForeignError> {
                Ok(witness::Tree {
                    value: terrane_int_support::coerce::<
                        i64,
                    >(&__terrane_conversion_input_0)
                        .map_err(|error| crate::TerraneForeignError(
                            crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
                        ))?,
                    children: {
                        let __terrane_field_argument: Option<
                            terrane_collection_support::List<Tree>,
                        > = __terrane_conversion_input_1;
                        __terrane_field_argument
                    }
                        .map(|
                            value,
                        | -> Result<Vec<witness::Tree>, crate::TerraneForeignError> {
                            Ok(
                                value
                                    .into_iter()
                                    .map(|item| -> Result<_, crate::TerraneForeignError> {
                                        Ok(item)
                                    })
                                    .collect::<Result<Vec<witness::Tree>, _>>()?,
                            )
                        })
                        .transpose()?
                        .into(),
                })
            }(),
            39 /* terrane-site: src/main.trn:143:12-143:54 */,
        )
    };
    println!("{}", terrane_scalar_support::scalar_text(&leaf.value));
    branch = {
        let __terrane_conversion_input_0 = terrane_int_support::Int::from(50_i128);
        let __terrane_conversion_input_1 = Some(
            terrane_collection_support::List::<Tree>::new(vec![leaf]),
        );
        __terrane_raised(
            || -> Result<_, crate::TerraneForeignError> {
                Ok(witness::Tree {
                    value: terrane_int_support::coerce::<
                        i64,
                    >(&__terrane_conversion_input_0)
                        .map_err(|error| crate::TerraneForeignError(
                            crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
                        ))?,
                    children: {
                        let __terrane_field_argument: Option<
                            terrane_collection_support::List<Tree>,
                        > = __terrane_conversion_input_1;
                        __terrane_field_argument
                    }
                        .map(|
                            value,
                        | -> Result<Vec<witness::Tree>, crate::TerraneForeignError> {
                            Ok(
                                value
                                    .into_iter()
                                    .map(|item| -> Result<_, crate::TerraneForeignError> {
                                        Ok(item)
                                    })
                                    .collect::<Result<Vec<witness::Tree>, _>>()?,
                            )
                        })
                        .transpose()?
                        .into(),
                })
            }(),
            40 /* terrane-site: src/main.trn:145:14-145:64 */,
        )
    };
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(child_count(&branch),
        41 /* terrane-site: src/main.trn:146:13-146:32 */))
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
            42 /* terrane-site: src/main.trn:149:16-149:27 */)),
            42 /* terrane-site: src/main.trn:149:16-149:27 */).value)
        );
    }
    absent = {
        let __terrane_conversion_input_0 = terrane_int_support::Int::from(51_i128);
        __terrane_raised(
            || -> Result<_, crate::TerraneForeignError> {
                Ok(witness::Tree {
                    value: terrane_int_support::coerce::<
                        i64,
                    >(&__terrane_conversion_input_0)
                        .map_err(|error| crate::TerraneForeignError(
                            crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
                        ))?,
                    children: None::<Vec<witness::Tree>>,
                })
            }(),
            43 /* terrane-site: src/main.trn:150:14-150:39 */,
        )
    };
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(child_count(&absent),
        44 /* terrane-site: src/main.trn:151:13-151:32 */))
    );
    let __terrane_completion_0: TerraneCompletion<()> = (|| {
        let __terrane_try_0: TerraneCompletion<()> = (|| {
            __terrane_traced_completion!(
                alpha.dispatch(&executor, { let __terrane_conversion_input_0 =
                terrane_int_support::Int::from(- 1_i128); __terrane_raised_completion!(|
                | -> Result < _, crate ::TerraneForeignError > { Ok(witness::Alpha {
                value : terrane_int_support::coerce:: < i64 >
                (&__terrane_conversion_input_0).map_err(| error | crate
                ::TerraneForeignError(crate ::TerraneRaised::raised(error, crate
                ::TERRANE_NO_SITE))) ? }) } (), 45 /* terrane-site: src/main.trn:153:36-153:62 */) }), 46 /* terrane-site: src/main.trn:153:9-153:63 */
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
                    {
                        let __terrane_error_text_0 = failure
                            .as_ref()
                            .expect("flow-proven available binding")
                            .message();
                        println!("{}", __terrane_error_text_0);
                    };
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
    selected_mode = {
        let __terrane_conversion_input_0 = true;
        __terrane_raised(
            mode(__terrane_conversion_input_0),
            47 /* terrane-site: src/main.trn:156:21-156:31 */,
        )
    };
    if (&selected_mode).is_some() {
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let __terrane_comparison_left =
            &match &selected_mode { Some(value) => value, _ =>
            unreachable!("flow-proven storage refinement") }; let
            __terrane_receiver_value_1 = &__terrane_raised(| | -> Result < _, crate
            ::TerraneForeignError > { { let value = match
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
            crate ::TerraneForeignError > { Ok(witness::Mode::First) })) { Ok(result) =>
            result ?, Err(payload) => return Err(crate
            ::__terrane_dependency_panic(payload, "witness", "witness::Mode::First")) };
            Ok:: < _, crate ::TerraneForeignError > (value) } } (),
            48 /* terrane-site: src/main.trn:158:34-158:46 */); let
            __terrane_comparison_right = &__terrane_receiver_value_1; < Mode as
            std::cmp::PartialEq > ::eq(__terrane_comparison_left,
            __terrane_comparison_right) })
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let __terrane_receiver_value_2 =
            &__terrane_raised(| | -> Result < _, crate ::TerraneForeignError > { { let
            value = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | ->
            Result < _, crate ::TerraneForeignError > { Ok(witness::Mode::First) })) {
            Ok(result) => result ?, Err(payload) => return Err(crate
            ::__terrane_dependency_panic(payload, "witness", "witness::Mode::First")) };
            Ok:: < _, crate ::TerraneForeignError > (value) } } (),
            49 /* terrane-site: src/main.trn:159:17-159:29 */); let
            __terrane_comparison_left = &__terrane_receiver_value_2; let
            __terrane_comparison_right = &match &selected_mode { Some(value) => value, _
            => unreachable!("flow-proven storage refinement") }; < Mode as
            std::cmp::PartialEq > ::eq(__terrane_comparison_left,
            __terrane_comparison_right) })
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let __terrane_comparison_left =
            &match &selected_mode { Some(value) => value, _ =>
            unreachable!("flow-proven storage refinement") }; let
            __terrane_receiver_value_3 = &__terrane_raised(| | -> Result < _, crate
            ::TerraneForeignError > { { let value = match
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
            crate ::TerraneForeignError > { Ok(witness::Mode::Last) })) { Ok(result) =>
            result ?, Err(payload) => return Err(crate
            ::__terrane_dependency_panic(payload, "witness", "witness::Mode::Last")) };
            Ok:: < _, crate ::TerraneForeignError > (value) } } (),
            50 /* terrane-site: src/main.trn:160:34-160:45 */); let
            __terrane_comparison_right = &__terrane_receiver_value_3; < Mode as
            std::cmp::PartialEq > ::ne(__terrane_comparison_left,
            __terrane_comparison_right) })
        );
    }
    missing_mode = {
        let __terrane_conversion_input_0 = false;
        __terrane_raised(
            mode(__terrane_conversion_input_0),
            51 /* terrane-site: src/main.trn:161:20-161:31 */,
        )
    };
    println!("{}", terrane_scalar_support::scalar_text(&(&missing_mode).is_none()));
    check_numeric_fields();
    __terrane_traced(
        check_recipe_selection(&executor),
        52 /* terrane-site: src/main.trn:164:5-164:37 */,
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
        return Ok({
            let __terrane_conversion_input_0 = input;
            __terrane_raised_err(
                || -> Result<_, crate::TerraneForeignError> {
                    match match std::panic::catch_unwind(
                        std::panic::AssertUnwindSafe(|| -> Result<
                            _,
                            crate::TerraneForeignError,
                        > { Ok((&*service).call(__terrane_conversion_input_0)) }),
                    ) {
                        Ok(result) => result?,
                        Err(payload) => {
                            return Err(
                                crate::__terrane_dependency_panic(
                                    payload,
                                    "witness",
                                    "witness::Executor::call",
                                ),
                            );
                        }
                    } {
                        Ok(value) => Ok(value),
                        Err(error) => {
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
                    }
                }(),
                53 /* terrane-site: src/main.trn:171:16-171:40 */,
            )?
        });
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
        return Ok({
            let __terrane_conversion_input_0 = input;
            __terrane_raised_err(
                || -> Result<_, crate::TerraneForeignError> {
                    match match std::panic::catch_unwind(
                        std::panic::AssertUnwindSafe(|| -> Result<
                            _,
                            crate::TerraneForeignError,
                        > { Ok((&*service).call(__terrane_conversion_input_0)) }),
                    ) {
                        Ok(result) => result?,
                        Err(payload) => {
                            return Err(
                                crate::__terrane_dependency_panic(
                                    payload,
                                    "witness",
                                    "witness::Executor::call",
                                ),
                            );
                        }
                    } {
                        Ok(value) => Ok(value),
                        Err(error) => {
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
                    }
                }(),
                54 /* terrane-site: src/main.trn:178:16-178:40 */,
            )?
        });
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
        return Ok({
            let __terrane_conversion_input_0 = input;
            __terrane_raised_err(
                || -> Result<_, crate::TerraneForeignError> {
                    match match std::panic::catch_unwind(
                        std::panic::AssertUnwindSafe(|| -> Result<
                            _,
                            crate::TerraneForeignError,
                        > { Ok((&*service).call(__terrane_conversion_input_0)) }),
                    ) {
                        Ok(result) => result?,
                        Err(payload) => {
                            return Err(
                                crate::__terrane_dependency_panic(
                                    payload,
                                    "witness",
                                    "witness::Executor::call",
                                ),
                            );
                        }
                    } {
                        Ok(value) => Ok(value),
                        Err(error) => {
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
                    }
                }(),
                55 /* terrane-site: src/main.trn:185:16-185:40 */,
            )?
        });
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
    fields = {
        let __terrane_conversion_input_0 = None;
        let __terrane_conversion_input_1 = None;
        __terrane_raised(
            || -> Result<_, crate::TerraneForeignError> {
                Ok(witness::NumericFields {
                    quality: {
                        let __terrane_field_argument: Option<u32> = __terrane_conversion_input_0;
                        __terrane_field_argument
                    }
                        .into(),
                    ratio: {
                        let __terrane_field_argument: Option<f64> = __terrane_conversion_input_1;
                        __terrane_field_argument
                    }
                        .into(),
                })
            }(),
            56 /* terrane-site: src/main.trn:193:14-193:66 */,
        )
    };
    selected = optional_number(true, terrane_int_support::Int::from(73_i128));
    {
        let __terrane_value = match selected.clone() {
            Some(__terrane_optional_value) => {
                Some(
                    __terrane_raised(
                        terrane_int_support::coerce::<u32>(&__terrane_optional_value),
                        57 /* terrane-site: src/main.trn:195:22-195:30 */,
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
                        58 /* terrane-site: src/main.trn:196:20-196:28 */,
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
                        59 /* terrane-site: src/main.trn:200:22-200:28 */,
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
                        60 /* terrane-site: src/main.trn:202:22-202:30 */,
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
                                (&__terrane_optional_value), 61 /* terrane-site: src/main.trn:205:26-205:33 */
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
        .dispatch(executor, __terrane_raised_err(batch(), 62 /* terrane-site: src/main.trn:211:43-211:49 */) ?), 63 /* terrane-site: src/main.trn:211:13-211:50 */) ?)
    );
    marked_value = MarkedCommand::terrane_construct();
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_traced_err(marked_value
        .dispatch(executor, __terrane_raised_err(marked(), 64 /* terrane-site: src/main.trn:213:47-213:54 */) ?), 65 /* terrane-site: src/main.trn:213:13-213:55 */) ?)
    );
    borrowed_value = BorrowedOutputCommand::terrane_construct();
    result = __terrane_traced_err(
        borrowed_value
            .dispatch(
                executor,
                __terrane_raised_err(
                    borrowed_output(),
                    66 /* terrane-site: src/main.trn:215:50-215:66 */,
                )?,
            ),
        67 /* terrane-site: src/main.trn:215:14-215:67 */,
    )?;
    println!("{}", terrane_scalar_support::scalar_text(&result.value));
    borrowed_input = {
        let __terrane_conversion_input_0 = terrane_int_support::Int::from(93_i128);
        __terrane_raised_err(
            || -> Result<_, crate::TerraneForeignError> {
                Ok(witness::Alpha {
                    value: terrane_int_support::coerce::<
                        i64,
                    >(&__terrane_conversion_input_0)
                        .map_err(|error| crate::TerraneForeignError(
                            crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
                        ))?,
                })
            }(),
            68 /* terrane-site: src/main.trn:217:22-217:48 */,
        )?
    };
    borrowed_loan = &borrowed_input;
    borrowed_result = __terrane_raised_err(
        || -> Result<_, crate::TerraneForeignError> {
            match match std::panic::catch_unwind(
                std::panic::AssertUnwindSafe(|| -> Result<_, crate::TerraneForeignError> {
                    Ok(witness::execute(&*borrowed_loan))
                }),
            ) {
                Ok(result) => result?,
                Err(payload) => {
                    return Err(
                        crate::__terrane_dependency_panic(
                            payload,
                            "witness",
                            "witness::execute",
                        ),
                    );
                }
            } {
                Ok(value) => Ok(value),
                Err(error) => {
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
            }
        }(),
        69 /* terrane-site: src/main.trn:219:23-219:45 */,
    )?;
    println!("{}", terrane_scalar_support::scalar_text(&borrowed_result.label));
    set_request = __terrane_raised_err(
        set_batch(),
        70 /* terrane-site: src/main.trn:221:19-221:29 */,
    )?;
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_conversion_input_0 =
        set_request; __terrane_raised_err(| | -> Result < _, crate ::TerraneForeignError
        > { match match std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | ->
        Result < _, crate ::TerraneForeignError > {
        Ok(witness::execute(__terrane_conversion_input_0)) })) { Ok(result) => result ?,
        Err(payload) => return Err(crate ::__terrane_dependency_panic(payload, "witness",
        "witness::execute")) } { Ok(value) =>
        Ok(terrane_int_support::Int::from_u128(value as u128)), Err(error) => Err(crate
        ::TerraneForeignError(crate ::TerraneError::custom_raised(crate
        ::DescriptorId(2), error.to_string(), crate ::TERRANE_NO_SITE))) } } (),
        71 /* terrane-site: src/main.trn:222:13-222:38 */) ? })
    );
    map_request = __terrane_raised_err(
        mapped_batch(),
        72 /* terrane-site: src/main.trn:223:19-223:32 */,
    )?;
    map_result = {
        let __terrane_conversion_input_0 = map_request;
        __terrane_raised_err(
            || -> Result<_, crate::TerraneForeignError> {
                match match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| -> Result<
                        _,
                        crate::TerraneForeignError,
                    > { Ok(witness::execute(__terrane_conversion_input_0)) }),
                ) {
                    Ok(result) => result?,
                    Err(payload) => {
                        return Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "witness",
                                "witness::execute",
                            ),
                        );
                    }
                } {
                    Ok(value) => Ok(value),
                    Err(error) => {
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
                }
            }(),
            73 /* terrane-site: src/main.trn:224:18-224:43 */,
        )?
    };
    println!("{}", terrane_scalar_support::scalar_text(&map_result.value));
    Ok(())
}
