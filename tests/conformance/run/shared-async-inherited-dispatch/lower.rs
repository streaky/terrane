// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: shared-async-inherited-dispatch
#[derive(Clone)]
pub struct BaseStorage {
    pub total: terrane_int_support::Int,
}
impl BaseStorage {
    pub fn terrane_construct() -> Self {
        Self {
            total: terrane_int_support::Int::from(0_i128),
        }
    }
    pub async fn accumulate(
        &mut self,
        delta: terrane_int_support::Int,
    ) -> terrane_int_support::Int {
        {
            let __terrane_value = self.total.clone() + delta.clone();
            self.total = __terrane_value;
        }
        return self.total.clone();
    }
    pub async fn __terrane_shared_async_accumulate(
        __terrane_shared_handle: impl TerraneSharedHandle<Self> + Clone,
        delta: terrane_int_support::Int,
    ) -> terrane_int_support::Int {
        {
            let __terrane_value = {
                let __terrane_snapshot = __terrane_shared_handle
                    .lock()
                    .expect("shared async receiver lock poisoned")
                    .total
                    .clone();
                __terrane_snapshot
            } + delta.clone();
            __terrane_shared_handle
                .lock()
                .expect("shared async receiver lock poisoned")
                .total = __terrane_value;
        }
        return {
            let __terrane_snapshot = __terrane_shared_handle
                .lock()
                .expect("shared async receiver lock poisoned")
                .total
                .clone();
            __terrane_snapshot
        };
    }
}
#[derive(Clone)]
pub enum Base {
    Own(BaseStorage),
    Derived(Derived),
    Leaf(Leaf),
}
impl Base {
    pub fn terrane_construct() -> Self {
        Self::Own(BaseStorage::terrane_construct())
    }
    pub async fn accumulate(
        &mut self,
        delta: terrane_int_support::Int,
    ) -> terrane_int_support::Int {
        match self {
            Self::Own(value) => value.accumulate(delta).await,
            Self::Derived(value) => value.accumulate(delta).await,
            Self::Leaf(value) => value.accumulate(delta).await,
        }
    }
    pub async fn __terrane_shared_async_accumulate(
        __terrane_shared_handle: impl TerraneSharedHandle<Self> + Clone,
        delta: terrane_int_support::Int,
    ) -> terrane_int_support::Int {
        let __terrane_variant = {
            let value = __terrane_shared_handle
                .lock()
                .expect("shared async receiver lock poisoned");
            match &*value {
                Self::Own(_) => 0,
                Self::Derived(_) => 1,
                Self::Leaf(_) => 2,
            }
        };
        match __terrane_variant {
            0 => {
                BaseStorage::__terrane_shared_async_accumulate(
                        TerraneSharedProjection::new(
                            __terrane_shared_handle,
                            |value: &Self| match value {
                                Self::Own(value) => value,
                                _ => unreachable!("shared receiver variant changed"),
                            },
                            |value: &mut Self| match value {
                                Self::Own(value) => value,
                                _ => unreachable!("shared receiver variant changed"),
                            },
                        ),
                        delta,
                    )
                    .await
            }
            1 => {
                Derived::__terrane_shared_async_accumulate(
                        TerraneSharedProjection::new(
                            __terrane_shared_handle,
                            |value: &Self| match value {
                                Self::Derived(value) => value,
                                _ => unreachable!("shared receiver variant changed"),
                            },
                            |value: &mut Self| match value {
                                Self::Derived(value) => value,
                                _ => unreachable!("shared receiver variant changed"),
                            },
                        ),
                        delta,
                    )
                    .await
            }
            2 => {
                Leaf::__terrane_shared_async_accumulate(
                        TerraneSharedProjection::new(
                            __terrane_shared_handle,
                            |value: &Self| match value {
                                Self::Leaf(value) => value,
                                _ => unreachable!("shared receiver variant changed"),
                            },
                            |value: &mut Self| match value {
                                Self::Leaf(value) => value,
                                _ => unreachable!("shared receiver variant changed"),
                            },
                        ),
                        delta,
                    )
                    .await
            }
            _ => unreachable!("invalid shared receiver variant"),
        }
    }
    pub fn terrane_field_total(&self) -> &terrane_int_support::Int {
        match self {
            Self::Own(value) => &value.total,
            Self::Derived(value) => value.terrane_field_total(),
            Self::Leaf(value) => &value.total,
        }
    }
    pub fn terrane_field_total_mut(&mut self) -> &mut terrane_int_support::Int {
        match self {
            Self::Own(value) => &mut value.total,
            Self::Derived(value) => value.terrane_field_total_mut(),
            Self::Leaf(value) => &mut value.total,
        }
    }
}
#[derive(Clone)]
pub struct DerivedStorage {
    pub total: terrane_int_support::Int,
    pub bonus: terrane_int_support::Int,
}
impl DerivedStorage {
    pub fn terrane_construct() -> Self {
        Self {
            total: terrane_int_support::Int::from(0_i128),
            bonus: terrane_int_support::Int::from(10_i128),
        }
    }
    pub async fn accumulate(
        &mut self,
        delta: terrane_int_support::Int,
    ) -> terrane_int_support::Int {
        {
            let __terrane_value = self.total.clone() + delta.clone()
                + self.bonus.clone();
            self.total = __terrane_value;
        }
        return self.total.clone();
    }
    pub async fn __terrane_shared_async_accumulate(
        __terrane_shared_handle: impl TerraneSharedHandle<Self> + Clone,
        delta: terrane_int_support::Int,
    ) -> terrane_int_support::Int {
        {
            let __terrane_value = {
                let __terrane_snapshot = __terrane_shared_handle
                    .lock()
                    .expect("shared async receiver lock poisoned")
                    .total
                    .clone();
                __terrane_snapshot
            } + delta.clone()
                + {
                    let __terrane_snapshot = __terrane_shared_handle
                        .lock()
                        .expect("shared async receiver lock poisoned")
                        .bonus
                        .clone();
                    __terrane_snapshot
                };
            __terrane_shared_handle
                .lock()
                .expect("shared async receiver lock poisoned")
                .total = __terrane_value;
        }
        return {
            let __terrane_snapshot = __terrane_shared_handle
                .lock()
                .expect("shared async receiver lock poisoned")
                .total
                .clone();
            __terrane_snapshot
        };
    }
}
#[derive(Clone)]
pub enum Derived {
    Own(DerivedStorage),
    Leaf(Leaf),
}
impl Derived {
    pub fn terrane_construct() -> Self {
        Self::Own(DerivedStorage::terrane_construct())
    }
    pub async fn accumulate(
        &mut self,
        delta: terrane_int_support::Int,
    ) -> terrane_int_support::Int {
        match self {
            Self::Own(value) => value.accumulate(delta).await,
            Self::Leaf(value) => value.accumulate(delta).await,
        }
    }
    pub async fn __terrane_shared_async_accumulate(
        __terrane_shared_handle: impl TerraneSharedHandle<Self> + Clone,
        delta: terrane_int_support::Int,
    ) -> terrane_int_support::Int {
        let __terrane_variant = {
            let value = __terrane_shared_handle
                .lock()
                .expect("shared async receiver lock poisoned");
            match &*value {
                Self::Own(_) => 0,
                Self::Leaf(_) => 1,
            }
        };
        match __terrane_variant {
            0 => {
                DerivedStorage::__terrane_shared_async_accumulate(
                        TerraneSharedProjection::new(
                            __terrane_shared_handle,
                            |value: &Self| match value {
                                Self::Own(value) => value,
                                _ => unreachable!("shared receiver variant changed"),
                            },
                            |value: &mut Self| match value {
                                Self::Own(value) => value,
                                _ => unreachable!("shared receiver variant changed"),
                            },
                        ),
                        delta,
                    )
                    .await
            }
            1 => {
                Leaf::__terrane_shared_async_accumulate(
                        TerraneSharedProjection::new(
                            __terrane_shared_handle,
                            |value: &Self| match value {
                                Self::Leaf(value) => value,
                                _ => unreachable!("shared receiver variant changed"),
                            },
                            |value: &mut Self| match value {
                                Self::Leaf(value) => value,
                                _ => unreachable!("shared receiver variant changed"),
                            },
                        ),
                        delta,
                    )
                    .await
            }
            _ => unreachable!("invalid shared receiver variant"),
        }
    }
    pub fn terrane_field_total(&self) -> &terrane_int_support::Int {
        match self {
            Self::Own(value) => &value.total,
            Self::Leaf(value) => &value.total,
        }
    }
    pub fn terrane_field_total_mut(&mut self) -> &mut terrane_int_support::Int {
        match self {
            Self::Own(value) => &mut value.total,
            Self::Leaf(value) => &mut value.total,
        }
    }
    pub fn terrane_field_bonus(&self) -> &terrane_int_support::Int {
        match self {
            Self::Own(value) => &value.bonus,
            Self::Leaf(value) => &value.bonus,
        }
    }
    pub fn terrane_field_bonus_mut(&mut self) -> &mut terrane_int_support::Int {
        match self {
            Self::Own(value) => &mut value.bonus,
            Self::Leaf(value) => &mut value.bonus,
        }
    }
}
#[derive(Clone)]
pub struct Leaf {
    pub total: terrane_int_support::Int,
    pub bonus: terrane_int_support::Int,
    pub tag: terrane_int_support::Int,
}
impl Leaf {
    pub fn terrane_construct() -> Self {
        Self {
            total: terrane_int_support::Int::from(0_i128),
            bonus: terrane_int_support::Int::from(10_i128),
            tag: terrane_int_support::Int::from(1_i128),
        }
    }
    pub async fn accumulate(
        &mut self,
        delta: terrane_int_support::Int,
    ) -> terrane_int_support::Int {
        {
            let __terrane_value = self.total.clone() + delta.clone()
                + self.bonus.clone();
            self.total = __terrane_value;
        }
        return self.total.clone();
    }
    pub async fn __terrane_shared_async_accumulate(
        __terrane_shared_handle: impl TerraneSharedHandle<Self> + Clone,
        delta: terrane_int_support::Int,
    ) -> terrane_int_support::Int {
        {
            let __terrane_value = {
                let __terrane_snapshot = __terrane_shared_handle
                    .lock()
                    .expect("shared async receiver lock poisoned")
                    .total
                    .clone();
                __terrane_snapshot
            } + delta.clone()
                + {
                    let __terrane_snapshot = __terrane_shared_handle
                        .lock()
                        .expect("shared async receiver lock poisoned")
                        .bonus
                        .clone();
                    __terrane_snapshot
                };
            __terrane_shared_handle
                .lock()
                .expect("shared async receiver lock poisoned")
                .total = __terrane_value;
        }
        return {
            let __terrane_snapshot = __terrane_shared_handle
                .lock()
                .expect("shared async receiver lock poisoned")
                .total
                .clone();
            __terrane_snapshot
        };
    }
}
fn main() {
    __terrane_run(async move {
        let plain: std::sync::Arc<TerraneSharedCell<Base>>;
        let plain_owner: std::sync::Arc<TerraneSharedCell<Base>>;
        let value: std::sync::Arc<TerraneSharedCell<Base>>;
        let owner: std::sync::Arc<TerraneSharedCell<Base>>;
        let alias: std::sync::Arc<TerraneSharedCell<Base>>;
        plain = std::sync::Arc::new(TerraneSharedCell::new(Base::terrane_construct()));
        plain_owner = plain.clone();
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_await({ let
            __terrane_shared_call_handle = plain_owner.clone(); let
            __terrane_shared_argument_0 = terrane_int_support::Int::from(2_i128); let
            __terrane_shared_argument_1 = __terrane_shared_argument_0;
            Base::__terrane_shared_async_accumulate(__terrane_shared_call_handle,
            __terrane_shared_argument_1) }). await)
        );
        value = std::sync::Arc::new(
            TerraneSharedCell::new(Base::Leaf(Leaf::terrane_construct())),
        );
        owner = value.clone();
        alias = owner.clone();
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_await({ let
            __terrane_shared_call_handle = owner.clone(); let __terrane_shared_argument_0
            = terrane_int_support::Int::from(5_i128); let __terrane_shared_argument_1 =
            __terrane_shared_argument_0;
            Base::__terrane_shared_async_accumulate(__terrane_shared_call_handle,
            __terrane_shared_argument_1) }). await)
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let __terrane_snapshot = alias
            .lock().expect("shared reference lock poisoned").terrane_field_total()
            .clone(); __terrane_snapshot })
        );
    });
}
