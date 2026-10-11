// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
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
