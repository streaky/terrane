// Generated deterministically by Terrane <version>.
// Runtime support: 
// Vendored support crates: terrane-int-support, terrane-collection-support, terrane-scalar-support, terrane-string-support
// Source: case.trn
// Namespace: callable-class-parameters
fn increment(value: terrane_int_support::Int) -> terrane_int_support::Int {
    return value.clone() + terrane_int_support::Int::from(1_i128);
}
#[derive(Clone)]
pub struct DirectHolder {
    pub stored: terrane_int_support::Int,
}
impl DirectHolder {
    pub fn terrane_construct(
        operation: std::sync::Arc<
            dyn Fn(terrane_int_support::Int) -> terrane_int_support::Int + Send + Sync,
        >,
    ) -> Self {
        let mut value = Self {
            stored: terrane_int_support::Int::from(0_i128),
        };
        value.construct(operation);
        value
    }
    pub fn construct(
        &mut self,
        operation: std::sync::Arc<
            dyn Fn(terrane_int_support::Int) -> terrane_int_support::Int + Send + Sync,
        >,
    ) {
        self.stored = operation(terrane_int_support::Int::from(1_i128));
    }
    pub fn apply(
        &self,
        operation: std::sync::Arc<
            dyn Fn(terrane_int_support::Int) -> terrane_int_support::Int + Send + Sync,
        >,
        value: terrane_int_support::Int,
    ) -> terrane_int_support::Int {
        return operation(self.stored.clone() + value.clone());
    }
    pub fn ignore(
        &self,
        operation: std::sync::Arc<
            dyn Fn(terrane_int_support::Int) -> terrane_int_support::Int + Send + Sync,
        >,
        value: terrane_int_support::Int,
    ) -> terrane_int_support::Int {
        let _ = &operation;
        return value.clone();
    }
}
#[derive(Clone)]
pub struct BaseHolderStorage {
    pub stored: terrane_int_support::Int,
}
impl BaseHolderStorage {
    pub fn terrane_construct(
        operation: std::sync::Arc<
            dyn Fn(terrane_int_support::Int) -> terrane_int_support::Int + Send + Sync,
        >,
    ) -> Self {
        let mut value = Self {
            stored: terrane_int_support::Int::from(0_i128),
        };
        value.construct(operation);
        value
    }
    pub fn construct(
        &mut self,
        operation: std::sync::Arc<
            dyn Fn(terrane_int_support::Int) -> terrane_int_support::Int + Send + Sync,
        >,
    ) {
        self.stored = operation(terrane_int_support::Int::from(1_i128));
    }
    pub fn apply(
        &self,
        operation: std::sync::Arc<
            dyn Fn(terrane_int_support::Int) -> terrane_int_support::Int + Send + Sync,
        >,
        value: terrane_int_support::Int,
    ) -> terrane_int_support::Int {
        return operation(self.stored.clone() + value.clone());
    }
}
#[derive(Clone)]
pub enum BaseHolder {
    Own(BaseHolderStorage),
    ChildHolder(ChildHolder),
}
impl BaseHolder {
    pub fn terrane_construct(
        operation: std::sync::Arc<
            dyn Fn(terrane_int_support::Int) -> terrane_int_support::Int + Send + Sync,
        >,
    ) -> Self {
        Self::Own(BaseHolderStorage::terrane_construct(operation))
    }
    pub fn apply(
        &self,
        operation: std::sync::Arc<
            dyn Fn(terrane_int_support::Int) -> terrane_int_support::Int + Send + Sync,
        >,
        value: terrane_int_support::Int,
    ) -> terrane_int_support::Int {
        match self {
            Self::Own(value_) => value_.apply(operation, value),
            Self::ChildHolder(value_) => value_.apply(operation, value),
        }
    }
    pub fn terrane_field_stored(&self) -> &terrane_int_support::Int {
        match self {
            Self::Own(value) => &value.stored,
            Self::ChildHolder(value) => &value.stored,
        }
    }
    pub fn terrane_field_stored_mut(&mut self) -> &mut terrane_int_support::Int {
        match self {
            Self::Own(value) => &mut value.stored,
            Self::ChildHolder(value) => &mut value.stored,
        }
    }
}
#[derive(Clone)]
pub struct ChildHolder {
    pub stored: terrane_int_support::Int,
}
impl ChildHolder {
    pub fn terrane_construct(
        operation: std::sync::Arc<
            dyn Fn(terrane_int_support::Int) -> terrane_int_support::Int + Send + Sync,
        >,
    ) -> Self {
        let mut value = Self {
            stored: terrane_int_support::Int::from(0_i128),
        };
        value.construct(operation);
        value
    }
    pub fn construct(
        &mut self,
        operation: std::sync::Arc<
            dyn Fn(terrane_int_support::Int) -> terrane_int_support::Int + Send + Sync,
        >,
    ) {
        self.stored = operation(terrane_int_support::Int::from(1_i128));
    }
    pub fn apply(
        &self,
        operation: std::sync::Arc<
            dyn Fn(terrane_int_support::Int) -> terrane_int_support::Int + Send + Sync,
        >,
        value: terrane_int_support::Int,
    ) -> terrane_int_support::Int {
        return operation(self.stored.clone() + value.clone());
    }
}
fn main() {
    let operation: std::sync::Arc<
        dyn Fn(terrane_int_support::Int) -> terrane_int_support::Int + Send + Sync,
    > = std::sync::Arc::new(increment);
    let direct: DirectHolder = DirectHolder::terrane_construct(operation.clone());
    println!(
        "{}", terrane_scalar_support::scalar_text(&direct.apply(operation.clone(),
        terrane_int_support::Int::from(4_i128)))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&direct.ignore(operation.clone(),
        terrane_int_support::Int::from(9_i128)))
    );
    let base: BaseHolder = BaseHolder::terrane_construct(operation.clone());
    println!(
        "{}", terrane_scalar_support::scalar_text(&base.apply(operation.clone(),
        terrane_int_support::Int::from(5_i128)))
    );
    let child: ChildHolder = ChildHolder::terrane_construct(operation.clone());
    println!(
        "{}", terrane_scalar_support::scalar_text(&child.apply(operation.clone(),
        terrane_int_support::Int::from(6_i128)))
    );
}
