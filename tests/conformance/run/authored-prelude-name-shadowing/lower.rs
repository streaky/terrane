// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/main.trn
// Namespace: app
#[derive(Clone)]
pub struct TerraneNsAppOptionIdentity2F617070006F7074696F6E {
    pub value: terrane_int_support::Int,
}
impl TerraneNsAppOptionIdentity2F617070006F7074696F6E {
    pub fn terrane_construct() -> Self {
        Self {
            value: terrane_int_support::Int::from(1_i128),
        }
    }
}
#[derive(Clone)]
pub struct TerraneNsAppVecIdentity2F61707000766563 {
    pub value: terrane_int_support::Int,
}
impl TerraneNsAppVecIdentity2F61707000766563 {
    pub fn terrane_construct() -> Self {
        Self {
            value: terrane_int_support::Int::from(2_i128),
        }
    }
}
#[derive(Clone)]
pub struct TerraneNsAppResultIdentity2F61707000726573756C74 {
    pub value: terrane_int_support::Int,
}
impl TerraneNsAppResultIdentity2F61707000726573756C74 {
    pub fn terrane_construct() -> Self {
        Self {
            value: terrane_int_support::Int::from(3_i128),
        }
    }
}
#[derive(Clone)]
pub struct TerraneNsAppStringIdentity2F61707000537472696E67 {
    pub value: terrane_int_support::Int,
}
impl TerraneNsAppStringIdentity2F61707000537472696E67 {
    pub fn terrane_construct() -> Self {
        Self {
            value: terrane_int_support::Int::from(4_i128),
        }
    }
}
#[derive(Clone)]
pub struct TerraneNsAppFutureIdentity2F61707000667574757265 {
    pub value: terrane_int_support::Int,
}
impl TerraneNsAppFutureIdentity2F61707000667574757265 {
    pub fn terrane_construct() -> Self {
        Self {
            value: terrane_int_support::Int::from(1_i128),
        }
    }
}
#[derive(Clone)]
pub struct TerraneNsAppIntoFutureIdentity2F61707000696E746F2D667574757265 {
    pub value: terrane_int_support::Int,
}
impl TerraneNsAppIntoFutureIdentity2F61707000696E746F2D667574757265 {
    pub fn terrane_construct() -> Self {
        Self {
            value: terrane_int_support::Int::from(2_i128),
        }
    }
}
#[derive(Clone)]
pub struct TerraneNsAppDoubleEndedIteratorIdentity2F61707000646F75626C652D656E6465642D6974657261746F72 {
    pub value: terrane_int_support::Int,
}
impl TerraneNsAppDoubleEndedIteratorIdentity2F61707000646F75626C652D656E6465642D6974657261746F72 {
    pub fn terrane_construct() -> Self {
        Self {
            value: terrane_int_support::Int::from(3_i128),
        }
    }
}
async fn later() -> terrane_int_support::Int {
    return terrane_int_support::Int::from(41_i128);
}
fn main() {
    __terrane_run(async move {
        let absent: Option<TerraneNsAppOptionIdentity2F617070006F7074696F6E>;
        let numbers: terrane_collection_support::List<
            TerraneNsAppOptionIdentity2F617070006F7074696F6E,
        >;
        let maybe_numbers: terrane_collection_support::List<
            Option<TerraneNsAppOptionIdentity2F617070006F7074696F6E>,
        >;
        let first: TerraneNsAppResultIdentity2F61707000726573756C74;
        let second: TerraneNsAppStringIdentity2F61707000537472696E67;
        let f: TerraneNsAppFutureIdentity2F61707000667574757265;
        let conversion: TerraneNsAppIntoFutureIdentity2F61707000696E746F2D667574757265;
        let iterator: TerraneNsAppDoubleEndedIteratorIdentity2F61707000646F75626C652D656E6465642D6974657261746F72;
        absent = None;
        numbers = terrane_collection_support::List::<
            TerraneNsAppOptionIdentity2F617070006F7074696F6E,
        >::new(vec![]);
        maybe_numbers = terrane_collection_support::List::<
            Option<TerraneNsAppOptionIdentity2F617070006F7074696F6E>,
        >::new(vec![absent]);
        first = TerraneNsAppResultIdentity2F61707000726573756C74::terrane_construct();
        second = TerraneNsAppStringIdentity2F61707000537472696E67::terrane_construct();
        println!("{}", terrane_scalar_support::scalar_text(&first.value));
        println!("{}", terrane_scalar_support::scalar_text(&second.value));
        println!(
            "{}",
            terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(numbers
            .length()))
        );
        println!(
            "{}",
            terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(maybe_numbers
            .length()))
        );
        f = TerraneNsAppFutureIdentity2F61707000667574757265::terrane_construct();
        conversion = TerraneNsAppIntoFutureIdentity2F61707000696E746F2D667574757265::terrane_construct();
        iterator = TerraneNsAppDoubleEndedIteratorIdentity2F61707000646F75626C652D656E6465642D6974657261746F72::terrane_construct();
        println!(
            "{}", terrane_scalar_support::scalar_text(&(f.value.clone() +
            __terrane_await(later()). await))
        );
        println!("{}", terrane_scalar_support::scalar_text(&conversion.value));
        println!("{}", terrane_scalar_support::scalar_text(&iterator.value));
    });
}
