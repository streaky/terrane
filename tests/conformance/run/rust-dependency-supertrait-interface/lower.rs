// Generated deterministically by Terrane <version>.
// Runtime support: 
// Vendored support crates: terrane-int-support, terrane-collection-support, terrane-scalar-support, terrane-string-support
// Source: src/main.trn
// Namespace: app
#[derive(Clone)]
pub struct Values {}
impl Values {
    pub fn terrane_construct() -> Self {
        Self {}
    }
    pub fn middle(&self) -> terrane_int_support::Int {
        return terrane_int_support::Int::from(12_i128);
    }
    pub fn child(&self) -> terrane_int_support::Int {
        return terrane_int_support::Int::from(20_i128);
    }
}
impl ChildProtocol for Values {
    fn clone_box(&self) -> Box<dyn ChildProtocol> {
        Box::new(self.clone())
    }
    fn separate_box(&self) -> Box<dyn ChildProtocol> {
        Box::new(self.clone())
    }
    fn base(&self) -> terrane_int_support::Int {
        || -> Result<terrane_int_support::Int, crate::TerraneForeignError> {
            let __terrane_default = <Values as terrane_supertrait_witness::Base>::base(
                &*self,
            );
            Ok(terrane_int_support::Int::from(i128::from(__terrane_default)))
        }()
            .unwrap_or_else(|error| panic!("{}", error.render()))
    }
    fn middle(&self) -> terrane_int_support::Int {
        Values::middle(&*self)
    }
    fn child(&self) -> terrane_int_support::Int {
        Values::child(&*self)
    }
}
impl From<Values> for Child {
    fn from(value: Values) -> Self {
        Self(Box::new(value))
    }
}
impl terrane_supertrait_witness::Child for Values {
    fn child(&self) -> i64 {
        let __terrane_boundary: Result<i64, crate::TerraneForeignError> = (|| {
            let __terrane_value = <Values>::child(&*self);
            Ok(
                terrane_int_support::coerce::<i64>(&__terrane_value)
                    .map_err(|error| crate::TerraneForeignError(
                        crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
                    ))?,
            )
        })();
        __terrane_boundary.unwrap_or_else(|error| panic!("{}", error.render()))
    }
}
impl MiddleProtocol for Values {
    fn clone_box(&self) -> Box<dyn MiddleProtocol> {
        Box::new(self.clone())
    }
    fn separate_box(&self) -> Box<dyn MiddleProtocol> {
        Box::new(self.clone())
    }
    fn base(&self) -> terrane_int_support::Int {
        || -> Result<terrane_int_support::Int, crate::TerraneForeignError> {
            let __terrane_default = <Values as terrane_supertrait_witness::Base>::base(
                &*self,
            );
            Ok(terrane_int_support::Int::from(i128::from(__terrane_default)))
        }()
            .unwrap_or_else(|error| panic!("{}", error.render()))
    }
    fn middle(&self) -> terrane_int_support::Int {
        Values::middle(&*self)
    }
}
impl From<Values> for Middle {
    fn from(value: Values) -> Self {
        Self(Box::new(value))
    }
}
impl terrane_supertrait_witness::Middle for Values {
    fn middle(&self) -> i64 {
        let __terrane_boundary: Result<i64, crate::TerraneForeignError> = (|| {
            let __terrane_value = <Values>::middle(&*self);
            Ok(
                terrane_int_support::coerce::<i64>(&__terrane_value)
                    .map_err(|error| crate::TerraneForeignError(
                        crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
                    ))?,
            )
        })();
        __terrane_boundary.unwrap_or_else(|error| panic!("{}", error.render()))
    }
}
impl BaseProtocol for Values {
    fn clone_box(&self) -> Box<dyn BaseProtocol> {
        Box::new(self.clone())
    }
    fn separate_box(&self) -> Box<dyn BaseProtocol> {
        Box::new(self.clone())
    }
    fn base(&self) -> terrane_int_support::Int {
        || -> Result<terrane_int_support::Int, crate::TerraneForeignError> {
            let __terrane_default = <Values as terrane_supertrait_witness::Base>::base(
                &*self,
            );
            Ok(terrane_int_support::Int::from(i128::from(__terrane_default)))
        }()
            .unwrap_or_else(|error| panic!("{}", error.render()))
    }
}
impl From<Values> for Base {
    fn from(value: Values) -> Self {
        Self(Box::new(value))
    }
}
impl terrane_supertrait_witness::Base for Values {}
#[derive(Clone)]
pub struct DiamondValues {}
impl DiamondValues {
    pub fn terrane_construct() -> Self {
        Self {}
    }
    pub fn left(&self) -> terrane_int_support::Int {
        return terrane_int_support::Int::from(1_i128);
    }
    pub fn right(&self) -> terrane_int_support::Int {
        return terrane_int_support::Int::from(2_i128);
    }
    pub fn diamond(&self) -> terrane_int_support::Int {
        return terrane_int_support::Int::from(3_i128);
    }
}
impl DiamondProtocol for DiamondValues {
    fn clone_box(&self) -> Box<dyn DiamondProtocol> {
        Box::new(self.clone())
    }
    fn separate_box(&self) -> Box<dyn DiamondProtocol> {
        Box::new(self.clone())
    }
    fn base(&self) -> terrane_int_support::Int {
        || -> Result<terrane_int_support::Int, crate::TerraneForeignError> {
            let __terrane_default = <DiamondValues as terrane_supertrait_witness::Base>::base(
                &*self,
            );
            Ok(terrane_int_support::Int::from(i128::from(__terrane_default)))
        }()
            .unwrap_or_else(|error| panic!("{}", error.render()))
    }
    fn left(&self) -> terrane_int_support::Int {
        DiamondValues::left(&*self)
    }
    fn right(&self) -> terrane_int_support::Int {
        DiamondValues::right(&*self)
    }
    fn diamond(&self) -> terrane_int_support::Int {
        DiamondValues::diamond(&*self)
    }
}
impl From<DiamondValues> for Diamond {
    fn from(value: DiamondValues) -> Self {
        Self(Box::new(value))
    }
}
impl terrane_supertrait_witness::Diamond for DiamondValues {
    fn diamond(&self) -> i64 {
        let __terrane_boundary: Result<i64, crate::TerraneForeignError> = (|| {
            let __terrane_value = <DiamondValues>::diamond(&*self);
            Ok(
                terrane_int_support::coerce::<i64>(&__terrane_value)
                    .map_err(|error| crate::TerraneForeignError(
                        crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
                    ))?,
            )
        })();
        __terrane_boundary.unwrap_or_else(|error| panic!("{}", error.render()))
    }
}
impl LeftProtocol for DiamondValues {
    fn clone_box(&self) -> Box<dyn LeftProtocol> {
        Box::new(self.clone())
    }
    fn separate_box(&self) -> Box<dyn LeftProtocol> {
        Box::new(self.clone())
    }
    fn base(&self) -> terrane_int_support::Int {
        || -> Result<terrane_int_support::Int, crate::TerraneForeignError> {
            let __terrane_default = <DiamondValues as terrane_supertrait_witness::Base>::base(
                &*self,
            );
            Ok(terrane_int_support::Int::from(i128::from(__terrane_default)))
        }()
            .unwrap_or_else(|error| panic!("{}", error.render()))
    }
    fn left(&self) -> terrane_int_support::Int {
        DiamondValues::left(&*self)
    }
}
impl From<DiamondValues> for Left {
    fn from(value: DiamondValues) -> Self {
        Self(Box::new(value))
    }
}
impl terrane_supertrait_witness::Left for DiamondValues {
    fn left(&self) -> i64 {
        let __terrane_boundary: Result<i64, crate::TerraneForeignError> = (|| {
            let __terrane_value = <DiamondValues>::left(&*self);
            Ok(
                terrane_int_support::coerce::<i64>(&__terrane_value)
                    .map_err(|error| crate::TerraneForeignError(
                        crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
                    ))?,
            )
        })();
        __terrane_boundary.unwrap_or_else(|error| panic!("{}", error.render()))
    }
}
impl BaseProtocol for DiamondValues {
    fn clone_box(&self) -> Box<dyn BaseProtocol> {
        Box::new(self.clone())
    }
    fn separate_box(&self) -> Box<dyn BaseProtocol> {
        Box::new(self.clone())
    }
    fn base(&self) -> terrane_int_support::Int {
        || -> Result<terrane_int_support::Int, crate::TerraneForeignError> {
            let __terrane_default = <DiamondValues as terrane_supertrait_witness::Base>::base(
                &*self,
            );
            Ok(terrane_int_support::Int::from(i128::from(__terrane_default)))
        }()
            .unwrap_or_else(|error| panic!("{}", error.render()))
    }
}
impl From<DiamondValues> for Base {
    fn from(value: DiamondValues) -> Self {
        Self(Box::new(value))
    }
}
impl terrane_supertrait_witness::Base for DiamondValues {}
impl RightProtocol for DiamondValues {
    fn clone_box(&self) -> Box<dyn RightProtocol> {
        Box::new(self.clone())
    }
    fn separate_box(&self) -> Box<dyn RightProtocol> {
        Box::new(self.clone())
    }
    fn base(&self) -> terrane_int_support::Int {
        || -> Result<terrane_int_support::Int, crate::TerraneForeignError> {
            let __terrane_default = <DiamondValues as terrane_supertrait_witness::Base>::base(
                &*self,
            );
            Ok(terrane_int_support::Int::from(i128::from(__terrane_default)))
        }()
            .unwrap_or_else(|error| panic!("{}", error.render()))
    }
    fn right(&self) -> terrane_int_support::Int {
        DiamondValues::right(&*self)
    }
}
impl From<DiamondValues> for Right {
    fn from(value: DiamondValues) -> Self {
        Self(Box::new(value))
    }
}
impl terrane_supertrait_witness::Right for DiamondValues {
    fn right(&self) -> i64 {
        let __terrane_boundary: Result<i64, crate::TerraneForeignError> = (|| {
            let __terrane_value = <DiamondValues>::right(&*self);
            Ok(
                terrane_int_support::coerce::<i64>(&__terrane_value)
                    .map_err(|error| crate::TerraneForeignError(
                        crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
                    ))?,
            )
        })();
        __terrane_boundary.unwrap_or_else(|error| panic!("{}", error.render()))
    }
}
fn main() {
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&__terrane_raised(child_total(Values::terrane_construct()),
        0 /* terrane-site: src/main.trn:23:13-23:44 */))
    );
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&__terrane_raised(erased_child_total(Values::terrane_construct()),
        1 /* terrane-site: src/main.trn:24:13-24:51 */))
    );
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&__terrane_raised(diamond_total(DiamondValues::terrane_construct()),
        2 /* terrane-site: src/main.trn:25:13-25:54 */))
    );
}
// Source: <terrane>/projected/deps/terrane-supertrait-witness.trn
// Namespace: deps/terrane-supertrait-witness
pub trait BaseProtocol: Send + Sync {
    fn clone_box(&self) -> Box<dyn BaseProtocol>;
    fn separate_box(&self) -> Box<dyn BaseProtocol>;
    fn base(&self) -> terrane_int_support::Int;
}
impl Clone for Box<dyn BaseProtocol> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}
pub struct Base(Box<dyn BaseProtocol>);
impl Clone for Base {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl Base {
    pub fn base(&self) -> terrane_int_support::Int {
        self.0.base()
    }
}
impl terrane_supertrait_witness::Base for Base {}
pub trait ChildProtocol: Send + Sync {
    fn clone_box(&self) -> Box<dyn ChildProtocol>;
    fn separate_box(&self) -> Box<dyn ChildProtocol>;
    fn base(&self) -> terrane_int_support::Int;
    fn middle(&self) -> terrane_int_support::Int;
    fn child(&self) -> terrane_int_support::Int;
}
impl Clone for Box<dyn ChildProtocol> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}
pub struct Child(Box<dyn ChildProtocol>);
impl Clone for Child {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl Child {
    pub fn base(&self) -> terrane_int_support::Int {
        self.0.base()
    }
    pub fn middle(&self) -> terrane_int_support::Int {
        self.0.middle()
    }
    pub fn child(&self) -> terrane_int_support::Int {
        self.0.child()
    }
}
impl terrane_supertrait_witness::Middle for Child {
    fn middle(&self) -> i64 {
        let __terrane_boundary: Result<i64, crate::TerraneForeignError> = (|| {
            let __terrane_value = <Child>::middle(&*self);
            Ok(
                terrane_int_support::coerce::<i64>(&__terrane_value)
                    .map_err(|error| crate::TerraneForeignError(
                        crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
                    ))?,
            )
        })();
        __terrane_boundary.unwrap_or_else(|error| panic!("{}", error.render()))
    }
}
impl terrane_supertrait_witness::Base for Child {}
impl terrane_supertrait_witness::Child for Child {
    fn child(&self) -> i64 {
        let __terrane_boundary: Result<i64, crate::TerraneForeignError> = (|| {
            let __terrane_value = <Child>::child(&*self);
            Ok(
                terrane_int_support::coerce::<i64>(&__terrane_value)
                    .map_err(|error| crate::TerraneForeignError(
                        crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
                    ))?,
            )
        })();
        __terrane_boundary.unwrap_or_else(|error| panic!("{}", error.render()))
    }
}
pub trait DiamondProtocol: Send + Sync {
    fn clone_box(&self) -> Box<dyn DiamondProtocol>;
    fn separate_box(&self) -> Box<dyn DiamondProtocol>;
    fn base(&self) -> terrane_int_support::Int;
    fn left(&self) -> terrane_int_support::Int;
    fn right(&self) -> terrane_int_support::Int;
    fn diamond(&self) -> terrane_int_support::Int;
}
impl Clone for Box<dyn DiamondProtocol> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}
pub struct Diamond(Box<dyn DiamondProtocol>);
impl Clone for Diamond {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl Diamond {
    pub fn base(&self) -> terrane_int_support::Int {
        self.0.base()
    }
    pub fn left(&self) -> terrane_int_support::Int {
        self.0.left()
    }
    pub fn right(&self) -> terrane_int_support::Int {
        self.0.right()
    }
    pub fn diamond(&self) -> terrane_int_support::Int {
        self.0.diamond()
    }
}
impl terrane_supertrait_witness::Left for Diamond {
    fn left(&self) -> i64 {
        let __terrane_boundary: Result<i64, crate::TerraneForeignError> = (|| {
            let __terrane_value = <Diamond>::left(&*self);
            Ok(
                terrane_int_support::coerce::<i64>(&__terrane_value)
                    .map_err(|error| crate::TerraneForeignError(
                        crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
                    ))?,
            )
        })();
        __terrane_boundary.unwrap_or_else(|error| panic!("{}", error.render()))
    }
}
impl terrane_supertrait_witness::Base for Diamond {}
impl terrane_supertrait_witness::Right for Diamond {
    fn right(&self) -> i64 {
        let __terrane_boundary: Result<i64, crate::TerraneForeignError> = (|| {
            let __terrane_value = <Diamond>::right(&*self);
            Ok(
                terrane_int_support::coerce::<i64>(&__terrane_value)
                    .map_err(|error| crate::TerraneForeignError(
                        crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
                    ))?,
            )
        })();
        __terrane_boundary.unwrap_or_else(|error| panic!("{}", error.render()))
    }
}
impl terrane_supertrait_witness::Diamond for Diamond {
    fn diamond(&self) -> i64 {
        let __terrane_boundary: Result<i64, crate::TerraneForeignError> = (|| {
            let __terrane_value = <Diamond>::diamond(&*self);
            Ok(
                terrane_int_support::coerce::<i64>(&__terrane_value)
                    .map_err(|error| crate::TerraneForeignError(
                        crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
                    ))?,
            )
        })();
        __terrane_boundary.unwrap_or_else(|error| panic!("{}", error.render()))
    }
}
pub trait LeftProtocol: Send + Sync {
    fn clone_box(&self) -> Box<dyn LeftProtocol>;
    fn separate_box(&self) -> Box<dyn LeftProtocol>;
    fn base(&self) -> terrane_int_support::Int;
    fn left(&self) -> terrane_int_support::Int;
}
impl Clone for Box<dyn LeftProtocol> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}
pub struct Left(Box<dyn LeftProtocol>);
impl Clone for Left {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl Left {
    pub fn base(&self) -> terrane_int_support::Int {
        self.0.base()
    }
    pub fn left(&self) -> terrane_int_support::Int {
        self.0.left()
    }
}
impl terrane_supertrait_witness::Base for Left {}
impl terrane_supertrait_witness::Left for Left {
    fn left(&self) -> i64 {
        let __terrane_boundary: Result<i64, crate::TerraneForeignError> = (|| {
            let __terrane_value = <Left>::left(&*self);
            Ok(
                terrane_int_support::coerce::<i64>(&__terrane_value)
                    .map_err(|error| crate::TerraneForeignError(
                        crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
                    ))?,
            )
        })();
        __terrane_boundary.unwrap_or_else(|error| panic!("{}", error.render()))
    }
}
pub trait MiddleProtocol: Send + Sync {
    fn clone_box(&self) -> Box<dyn MiddleProtocol>;
    fn separate_box(&self) -> Box<dyn MiddleProtocol>;
    fn base(&self) -> terrane_int_support::Int;
    fn middle(&self) -> terrane_int_support::Int;
}
impl Clone for Box<dyn MiddleProtocol> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}
pub struct Middle(Box<dyn MiddleProtocol>);
impl Clone for Middle {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl Middle {
    pub fn base(&self) -> terrane_int_support::Int {
        self.0.base()
    }
    pub fn middle(&self) -> terrane_int_support::Int {
        self.0.middle()
    }
}
impl terrane_supertrait_witness::Base for Middle {}
impl terrane_supertrait_witness::Middle for Middle {
    fn middle(&self) -> i64 {
        let __terrane_boundary: Result<i64, crate::TerraneForeignError> = (|| {
            let __terrane_value = <Middle>::middle(&*self);
            Ok(
                terrane_int_support::coerce::<i64>(&__terrane_value)
                    .map_err(|error| crate::TerraneForeignError(
                        crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
                    ))?,
            )
        })();
        __terrane_boundary.unwrap_or_else(|error| panic!("{}", error.render()))
    }
}
pub trait RightProtocol: Send + Sync {
    fn clone_box(&self) -> Box<dyn RightProtocol>;
    fn separate_box(&self) -> Box<dyn RightProtocol>;
    fn base(&self) -> terrane_int_support::Int;
    fn right(&self) -> terrane_int_support::Int;
}
impl Clone for Box<dyn RightProtocol> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}
pub struct Right(Box<dyn RightProtocol>);
impl Clone for Right {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl Right {
    pub fn base(&self) -> terrane_int_support::Int {
        self.0.base()
    }
    pub fn right(&self) -> terrane_int_support::Int {
        self.0.right()
    }
}
impl terrane_supertrait_witness::Base for Right {}
impl terrane_supertrait_witness::Right for Right {
    fn right(&self) -> i64 {
        let __terrane_boundary: Result<i64, crate::TerraneForeignError> = (|| {
            let __terrane_value = <Right>::right(&*self);
            Ok(
                terrane_int_support::coerce::<i64>(&__terrane_value)
                    .map_err(|error| crate::TerraneForeignError(
                        crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
                    ))?,
            )
        })();
        __terrane_boundary.unwrap_or_else(|error| panic!("{}", error.render()))
    }
}
pub fn child_total<T: terrane_supertrait_witness::Child>(
    value: T,
) -> Result<terrane_int_support::Int, crate::TerraneForeignError> {
    let value = value;
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| terrane_supertrait_witness::child_total(value)),
    ) {
        Ok(value) => Ok(terrane_int_support::Int::from(i128::from(value))),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "terrane-supertrait-witness",
                    "terrane_supertrait_witness::child_total",
                ),
            )
        }
    }
}
pub fn diamond_total<T: terrane_supertrait_witness::Diamond>(
    value: T,
) -> Result<terrane_int_support::Int, crate::TerraneForeignError> {
    let value = value;
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| terrane_supertrait_witness::diamond_total(value)),
    ) {
        Ok(value) => Ok(terrane_int_support::Int::from(i128::from(value))),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "terrane-supertrait-witness",
                    "terrane_supertrait_witness::diamond_total",
                ),
            )
        }
    }
}
pub fn erased_child_total<
    TerraneBoxed0: terrane_supertrait_witness::Child + core::marker::Send
        + core::marker::Sync + 'static,
>(value: TerraneBoxed0) -> Result<terrane_int_support::Int, crate::TerraneForeignError> {
    let value = Box::new(value);
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| terrane_supertrait_witness::erased_child_total(
            value,
        )),
    ) {
        Ok(value) => Ok(terrane_int_support::Int::from(i128::from(value))),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "terrane-supertrait-witness",
                    "terrane_supertrait_witness::erased_child_total",
                ),
            )
        }
    }
}
