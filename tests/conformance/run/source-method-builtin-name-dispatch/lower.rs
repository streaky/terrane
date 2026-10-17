// Generated deterministically by Terrane <version>.
// Source: case.trn
// Namespace: source-method-builtin-name-dispatch
#[derive(Clone)]
pub struct Plain {
    pub count: terrane_int_support::Int,
}
impl Plain {
    pub fn terrane_construct() -> Self {
        Self {
            count: terrane_int_support::Int::from(1_i128),
        }
    }
    pub fn add(&self, amount: terrane_int_support::Int) -> terrane_int_support::Int {
        return self.count.clone() + amount.clone();
    }
    pub fn multiply(
        &mut self,
        amount: terrane_int_support::Int,
    ) -> terrane_int_support::Int {
        {
            let __terrane_value = self.count.clone() * amount.clone();
            self.count = __terrane_value;
        }
        return self.count.clone();
    }
    pub fn encode(&self, amount: terrane_int_support::Int) -> terrane_int_support::Int {
        return self.count.clone() + amount.clone();
    }
}
#[derive(Clone)]
pub struct BaseStorage {
    pub count: terrane_int_support::Int,
}
impl BaseStorage {
    pub fn terrane_construct() -> Self {
        Self {
            count: terrane_int_support::Int::from(1_i128),
        }
    }
    pub fn add(&self, amount: terrane_int_support::Int) -> terrane_int_support::Int {
        return self.count.clone() + amount.clone();
    }
    pub fn multiply(
        &mut self,
        amount: terrane_int_support::Int,
    ) -> terrane_int_support::Int {
        {
            let __terrane_value = self.count.clone() * amount.clone();
            self.count = __terrane_value;
        }
        return self.count.clone();
    }
    pub fn encode(&self, amount: terrane_int_support::Int) -> terrane_int_support::Int {
        return self.count.clone() + amount.clone();
    }
}
#[derive(Clone)]
pub enum Base {
    Own(BaseStorage),
    Derived(Derived),
}
impl Base {
    pub fn terrane_construct() -> Self {
        Self::Own(BaseStorage::terrane_construct())
    }
    pub fn add(&self, amount: terrane_int_support::Int) -> terrane_int_support::Int {
        match self {
            Self::Own(value) => value.add(amount),
            Self::Derived(value) => value.add(amount),
        }
    }
    pub fn multiply(
        &mut self,
        amount: terrane_int_support::Int,
    ) -> terrane_int_support::Int {
        match self {
            Self::Own(value) => value.multiply(amount),
            Self::Derived(value) => value.multiply(amount),
        }
    }
    pub fn encode(&self, amount: terrane_int_support::Int) -> terrane_int_support::Int {
        match self {
            Self::Own(value) => value.encode(amount),
            Self::Derived(value) => value.encode(amount),
        }
    }
    pub fn terrane_field_count(&self) -> &terrane_int_support::Int {
        match self {
            Self::Own(value) => &value.count,
            Self::Derived(value) => &value.count,
        }
    }
    pub fn terrane_field_count_mut(&mut self) -> &mut terrane_int_support::Int {
        match self {
            Self::Own(value) => &mut value.count,
            Self::Derived(value) => &mut value.count,
        }
    }
}
#[derive(Clone)]
pub struct Derived {
    pub count: terrane_int_support::Int,
    pub marker: bool,
}
impl Derived {
    pub fn terrane_construct() -> Self {
        Self {
            count: terrane_int_support::Int::from(1_i128),
            marker: true,
        }
    }
    pub fn add(&self, amount: terrane_int_support::Int) -> terrane_int_support::Int {
        return self.count.clone() + amount.clone();
    }
    pub fn multiply(
        &mut self,
        amount: terrane_int_support::Int,
    ) -> terrane_int_support::Int {
        {
            let __terrane_value = self.count.clone() * amount.clone();
            self.count = __terrane_value;
        }
        return self.count.clone();
    }
    pub fn encode(&self, amount: terrane_int_support::Int) -> terrane_int_support::Int {
        return self.count.clone() + amount.clone();
    }
}
fn ordinary_plain(value: &Plain) {
    println!(
        "{}", terrane_scalar_support::scalar_text(&value.clone()
        .add(terrane_int_support::Int::from(2_i128)))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&value.clone()
        .encode(terrane_int_support::Int::from(3_i128)))
    );
}
fn ordinary_base(value: &Base) {
    println!(
        "{}", terrane_scalar_support::scalar_text(&value.clone()
        .add(terrane_int_support::Int::from(2_i128)))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&value.clone()
        .encode(terrane_int_support::Int::from(3_i128)))
    );
}
fn shared_plain(value: std::sync::Arc<std::sync::Mutex<Plain>>) {
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_owner = value.clone();
        let __terrane_shared_argument_0 = terrane_int_support::Int::from(2_i128); let
        __terrane_shared_argument_1 = __terrane_shared_argument_0; let
        __terrane_call_result = __terrane_owner.lock()
        .expect("shared reference lock poisoned").add(__terrane_shared_argument_1);
        __terrane_call_result })
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_owner = value.clone();
        let __terrane_shared_argument_0 = terrane_int_support::Int::from(2_i128); let
        __terrane_shared_argument_1 = __terrane_shared_argument_0; let
        __terrane_call_result = __terrane_owner.lock()
        .expect("shared reference lock poisoned").multiply(__terrane_shared_argument_1);
        __terrane_call_result })
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_owner = value.clone();
        let __terrane_shared_argument_0 = terrane_int_support::Int::from(3_i128); let
        __terrane_shared_argument_1 = __terrane_shared_argument_0; let
        __terrane_call_result = __terrane_owner.lock()
        .expect("shared reference lock poisoned").encode(__terrane_shared_argument_1);
        __terrane_call_result })
    );
}
fn shared_base(value: std::sync::Arc<std::sync::Mutex<Base>>) {
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_owner = value.clone();
        let __terrane_shared_argument_0 = terrane_int_support::Int::from(2_i128); let
        __terrane_shared_argument_1 = __terrane_shared_argument_0; let
        __terrane_call_result = __terrane_owner.lock()
        .expect("shared reference lock poisoned").add(__terrane_shared_argument_1);
        __terrane_call_result })
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_owner = value.clone();
        let __terrane_shared_argument_0 = terrane_int_support::Int::from(2_i128); let
        __terrane_shared_argument_1 = __terrane_shared_argument_0; let
        __terrane_call_result = __terrane_owner.lock()
        .expect("shared reference lock poisoned").multiply(__terrane_shared_argument_1);
        __terrane_call_result })
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_owner = value.clone();
        let __terrane_shared_argument_0 = terrane_int_support::Int::from(3_i128); let
        __terrane_shared_argument_1 = __terrane_shared_argument_0; let
        __terrane_call_result = __terrane_owner.lock()
        .expect("shared reference lock poisoned").encode(__terrane_shared_argument_1);
        __terrane_call_result })
    );
}
fn main() {
    let first: Plain;
    let second: Base;
    let third: std::sync::Arc<std::sync::Mutex<Plain>>;
    let fourth: std::sync::Arc<std::sync::Mutex<Base>>;
    let fifth: std::sync::Arc<std::sync::Mutex<Plain>>;
    let fifth_held: std::sync::Arc<std::sync::Mutex<Plain>>;
    let fifth_observer: std::sync::Weak<std::sync::Mutex<Plain>>;
    let sixth: std::sync::Arc<std::sync::Mutex<Base>>;
    let sixth_held: std::sync::Arc<std::sync::Mutex<Base>>;
    let sixth_observer: std::sync::Weak<std::sync::Mutex<Base>>;
    first = Plain::terrane_construct();
    ordinary_plain(&first);
    second = Base::Derived(Derived::terrane_construct());
    ordinary_base(&second);
    third = std::sync::Arc::new(std::sync::Mutex::new(Plain::terrane_construct()));
    shared_plain(third.clone());
    fourth = std::sync::Arc::new(
        std::sync::Mutex::new(Base::Derived(Derived::terrane_construct())),
    );
    shared_base(fourth.clone());
    fifth = std::sync::Arc::new(std::sync::Mutex::new(Plain::terrane_construct()));
    fifth_held = fifth.clone();
    fifth_observer = std::sync::Arc::downgrade(&fifth.clone());
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_owner = fifth_observer
        .upgrade().expect("reference expired"); let __terrane_shared_argument_0 =
        terrane_int_support::Int::from(2_i128); let __terrane_shared_argument_1 =
        __terrane_shared_argument_0; let __terrane_call_result = __terrane_owner.lock()
        .expect("reference lock poisoned").multiply(__terrane_shared_argument_1);
        __terrane_call_result })
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_snapshot = fifth_held
        .lock().expect("shared reference lock poisoned").count.clone();
        __terrane_snapshot })
    );
    sixth = std::sync::Arc::new(
        std::sync::Mutex::new(Base::Derived(Derived::terrane_construct())),
    );
    sixth_held = sixth.clone();
    sixth_observer = std::sync::Arc::downgrade(&sixth.clone());
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_owner = sixth_observer
        .upgrade().expect("reference expired"); let __terrane_shared_argument_0 =
        terrane_int_support::Int::from(2_i128); let __terrane_shared_argument_1 =
        __terrane_shared_argument_0; let __terrane_call_result = __terrane_owner.lock()
        .expect("reference lock poisoned").multiply(__terrane_shared_argument_1);
        __terrane_call_result })
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_snapshot = sixth_held
        .lock().expect("shared reference lock poisoned").terrane_field_count().clone();
        __terrane_snapshot })
    );
}
