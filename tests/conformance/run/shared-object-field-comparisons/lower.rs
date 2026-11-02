// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: shared-object-field-comparisons
#[derive(Clone)]
pub struct HolderStorage {
    pub a: Duration,
    pub b: Duration,
}
impl HolderStorage {
    pub fn terrane_construct() -> Self {
        Self {
            a: __terrane_traced(
                Duration::terrane_static_seconds(terrane_int_support::Int::from(1_i128)),
                0 /* terrane-site: case.trn:5:18-5:38 */,
            ),
            b: __terrane_traced(
                Duration::terrane_static_seconds(terrane_int_support::Int::from(2_i128)),
                1 /* terrane-site: case.trn:6:18-6:38 */,
            ),
        }
    }
}
#[derive(Clone)]
pub enum Holder {
    Own(HolderStorage),
    InheritedHolder(InheritedHolder),
}
impl Holder {
    pub fn terrane_construct() -> Self {
        Self::Own(HolderStorage::terrane_construct())
    }
    pub fn terrane_field_a(&self) -> &Duration {
        match self {
            Self::Own(value) => &value.a,
            Self::InheritedHolder(value) => &value.a,
        }
    }
    pub fn terrane_field_a_mut(&mut self) -> &mut Duration {
        match self {
            Self::Own(value) => &mut value.a,
            Self::InheritedHolder(value) => &mut value.a,
        }
    }
    pub fn terrane_field_b(&self) -> &Duration {
        match self {
            Self::Own(value) => &value.b,
            Self::InheritedHolder(value) => &value.b,
        }
    }
    pub fn terrane_field_b_mut(&mut self) -> &mut Duration {
        match self {
            Self::Own(value) => &mut value.b,
            Self::InheritedHolder(value) => &mut value.b,
        }
    }
}
#[derive(Clone)]
pub struct InheritedHolder {
    pub a: Duration,
    pub b: Duration,
}
impl InheritedHolder {
    pub fn terrane_construct() -> Self {
        Self {
            a: __terrane_traced(
                Duration::terrane_static_seconds(terrane_int_support::Int::from(1_i128)),
                0 /* terrane-site: case.trn:5:18-5:38 */,
            ),
            b: __terrane_traced(
                Duration::terrane_static_seconds(terrane_int_support::Int::from(2_i128)),
                1 /* terrane-site: case.trn:6:18-6:38 */,
            ),
        }
    }
}
#[derive(Clone)]
pub struct RequiredHolder {
    pub a: Option<Duration>,
    pub b: Option<Duration>,
}
impl RequiredHolder {
    pub fn terrane_construct(a: Duration, b: Duration) -> Self {
        let mut __terrane_constructed_value = Self { a: None, b: None };
        __terrane_constructed_value.construct(a, b);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, a: Duration, b: Duration) {
        {
            let __terrane_value = a;
            self.a = Some(__terrane_value);
        }
        {
            let __terrane_value = b;
            self.b = Some(__terrane_value);
        }
    }
}
fn replace(
    owner: std::sync::Arc<TerraneSharedCell<Holder>>,
) -> Result<Duration, TerraneError> {
    {
        let __terrane_owner_0 = owner.clone();
        let __terrane_value = __terrane_traced_err(
            Duration::terrane_static_seconds(terrane_int_support::Int::from(2_i128)),
            2 /* terrane-site: case.trn:18:15-18:35 */,
        )?;
        *__terrane_owner_0
            .lock()
            .expect("shared reference lock poisoned")
            .terrane_field_a_mut() = __terrane_value;
    }
    return Ok({
        let __terrane_snapshot = owner
            .lock()
            .expect("shared reference lock poisoned")
            .terrane_field_a()
            .clone();
        __terrane_snapshot
    });
}
fn main() {
    let h: std::sync::Arc<TerraneSharedCell<Holder>>;
    let other: Duration;
    let alias: std::sync::Arc<TerraneSharedCell<Holder>>;
    let observer: std::sync::Weak<TerraneSharedCell<Holder>>;
    let ordinary: Holder;
    let inherited: std::sync::Arc<TerraneSharedCell<InheritedHolder>>;
    let initialized: std::sync::Arc<TerraneSharedCell<RequiredHolder>>;
    h = std::sync::Arc::new(TerraneSharedCell::new(Holder::terrane_construct()));
    other = __terrane_traced(
        Duration::terrane_static_seconds(terrane_int_support::Int::from(1_i128)),
        3 /* terrane-site: case.trn:23:13-23:33 */,
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_comparison_left = &{
        let __terrane_snapshot = h.lock().expect("shared reference lock poisoned")
        .terrane_field_a().clone(); __terrane_snapshot }; let __terrane_comparison_right
        = &other; < Duration as std::cmp::PartialEq > ::eq(__terrane_comparison_left,
        __terrane_comparison_right) })
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_comparison_left =
        &other; let __terrane_comparison_right = &{ let __terrane_snapshot = h.lock()
        .expect("shared reference lock poisoned").terrane_field_a().clone();
        __terrane_snapshot }; < Duration as std::cmp::PartialEq >
        ::eq(__terrane_comparison_left, __terrane_comparison_right) })
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_comparison_left = &{
        let __terrane_snapshot = h.lock().expect("shared reference lock poisoned")
        .terrane_field_a().clone(); __terrane_snapshot }; let __terrane_comparison_right
        = &{ let __terrane_snapshot = h.lock().expect("shared reference lock poisoned")
        .terrane_field_b().clone(); __terrane_snapshot }; < Duration as
        std::cmp::PartialOrd > ::lt(__terrane_comparison_left,
        __terrane_comparison_right) })
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_comparison_left = &{
        let __terrane_snapshot = h.lock().expect("shared reference lock poisoned")
        .terrane_field_b().clone(); __terrane_snapshot }; let __terrane_comparison_right
        = &{ let __terrane_snapshot = h.lock().expect("shared reference lock poisoned")
        .terrane_field_a().clone(); __terrane_snapshot }; < Duration as
        std::cmp::PartialOrd > ::gt(__terrane_comparison_left,
        __terrane_comparison_right) })
    );
    alias = h.clone();
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_comparison_left = &{
        let __terrane_snapshot = h.lock().expect("shared reference lock poisoned")
        .terrane_field_a().clone(); __terrane_snapshot }; let __terrane_comparison_right
        = &{ let __terrane_snapshot = alias.lock()
        .expect("shared reference lock poisoned").terrane_field_b().clone();
        __terrane_snapshot }; < Duration as std::cmp::PartialOrd >
        ::le(__terrane_comparison_left, __terrane_comparison_right) })
    );
    observer = std::sync::Arc::downgrade(&h);
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_comparison_left = &{
        let __terrane_snapshot = { let __terrane_owner = observer.upgrade()
        .expect("reference expired"); __terrane_owner.lock()
        .expect("reference lock poisoned").terrane_field_a().clone() };
        __terrane_snapshot }; let __terrane_comparison_right = &{ let __terrane_snapshot
        = h.lock().expect("shared reference lock poisoned").terrane_field_b().clone();
        __terrane_snapshot }; < Duration as std::cmp::PartialOrd >
        ::lt(__terrane_comparison_left, __terrane_comparison_right) })
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_comparison_left = &{
        let __terrane_snapshot = h.lock().expect("shared reference lock poisoned")
        .terrane_field_a().clone(); __terrane_snapshot }; let __terrane_comparison_right
        = &__terrane_traced(replace(alias.clone()), 4 /* terrane-site: case.trn:32:19-32:33 */); < Duration as std::cmp::PartialOrd >
        ::lt(__terrane_comparison_left, __terrane_comparison_right) })
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_comparison_left = &{
        let __terrane_snapshot = h.lock().expect("shared reference lock poisoned")
        .terrane_field_a().clone(); __terrane_snapshot }; let __terrane_comparison_right
        = &{ let __terrane_snapshot = h.lock().expect("shared reference lock poisoned")
        .terrane_field_b().clone(); __terrane_snapshot }; < Duration as
        std::cmp::PartialEq > ::eq(__terrane_comparison_left, __terrane_comparison_right)
        })
    );
    ordinary = Holder::terrane_construct();
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_comparison_left =
        &ordinary.terrane_field_a(); let __terrane_comparison_right = &ordinary
        .terrane_field_b(); < Duration as std::cmp::PartialOrd >
        ::lt(__terrane_comparison_left, __terrane_comparison_right) })
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&ordinary.terrane_field_a().clone()
        .seconds)
    );
    inherited = std::sync::Arc::new(
        TerraneSharedCell::new(InheritedHolder::terrane_construct()),
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_comparison_left = &{
        let __terrane_snapshot = inherited.lock()
        .expect("shared reference lock poisoned").a.clone(); __terrane_snapshot }; let
        __terrane_comparison_right = &{ let __terrane_snapshot = inherited.lock()
        .expect("shared reference lock poisoned").b.clone(); __terrane_snapshot }; <
        Duration as std::cmp::PartialOrd > ::lt(__terrane_comparison_left,
        __terrane_comparison_right) })
    );
    initialized = std::sync::Arc::new(
        TerraneSharedCell::new(
            RequiredHolder::terrane_construct(
                __terrane_traced(
                    Duration::terrane_static_seconds(
                        terrane_int_support::Int::from(1_i128),
                    ),
                    5 /* terrane-site: case.trn:39:58-39:78 */,
                ),
                __terrane_traced(
                    Duration::terrane_static_seconds(
                        terrane_int_support::Int::from(2_i128),
                    ),
                    6 /* terrane-site: case.trn:39:82-39:102 */,
                ),
            ),
        ),
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_comparison_left = &{
        let __terrane_snapshot = initialized.lock()
        .expect("shared reference lock poisoned").a.as_ref()
        .expect("required field initialized").clone(); __terrane_snapshot }; let
        __terrane_comparison_right = &{ let __terrane_snapshot = initialized.lock()
        .expect("shared reference lock poisoned").b.as_ref()
        .expect("required field initialized").clone(); __terrane_snapshot }; < Duration
        as std::cmp::PartialOrd > ::lt(__terrane_comparison_left,
        __terrane_comparison_right) })
    );
}
