// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: model/state.trn
// Namespace: model
#[derive(Clone)]
pub enum State<TerraneType54> {
    Pending,
    Completed(TerraneType54),
}
// Source: src/main.trn
// Namespace: app
fn finish(amount: terrane_int_support::Int) -> State<terrane_int_support::Int> {
    return {
        let __terrane_enum_payload_0 = amount.clone();
        State::<terrane_int_support::Int>::Completed(__terrane_enum_payload_0)
    };
}
fn result(value: &State<terrane_int_support::Int>) -> terrane_int_support::Int {
    let amount: &terrane_int_support::Int;
    match &value {
        State::Pending => {
            return terrane_int_support::Int::from(0_i128);
        }
        State::Completed(__terrane_pattern_373_amount) => {
            amount = __terrane_pattern_373_amount;
            return amount.clone();
        }
    }
}
fn main() {
    let mut j: Job;
    j = Job::terrane_construct();
    {
        let __terrane_value = finish(terrane_int_support::Int::from(1_i128));
        j.status = __terrane_value;
    }
    println!("{}", terrane_scalar_support::scalar_text(&result(&j.status)));
    {
        let __terrane_value = finish(terrane_int_support::Int::from(42_i128));
        j.status = __terrane_value;
    }
    println!("{}", terrane_scalar_support::scalar_text(&result(&j.status)));
    {
        let __terrane_value = {
            let __terrane_enum_payload_0 = terrane_int_support::Int::from(5_i128);
            State::<terrane_int_support::Int>::Completed(__terrane_enum_payload_0)
        };
        j.status = __terrane_value;
    }
    println!("{}", terrane_scalar_support::scalar_text(&result(&j.status)));
    {
        let __terrane_value = terrane_collection_support::List::<
            State<terrane_int_support::Int>,
        >::new(vec![finish(terrane_int_support::Int::from(9_i128))]);
        j.statuses = __terrane_value;
    }
    println!(
        "{}", terrane_scalar_support::scalar_text(&result({ let __terrane_index =
        __terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        0 /* terrane-site: src/main.trn:26:25-26:38 */); __terrane_raised(j.statuses
        .get(__terrane_index).ok_or_else(| |
        terrane_collection_support::IndexError::from_usize(__terrane_index)),
        0 /* terrane-site: src/main.trn:26:25-26:38 */) }))
    );
}
// Source: wrapper/job.trn
// Namespace: wrapper
#[derive(Clone)]
pub struct Job {
    pub status: State<terrane_int_support::Int>,
    pub statuses: terrane_collection_support::List<State<terrane_int_support::Int>>,
}
impl Job {
    pub fn terrane_construct() -> Self {
        Self {
            status: State::<terrane_int_support::Int>::Pending,
            statuses: terrane_collection_support::List::<
                State<terrane_int_support::Int>,
            >::new(Vec::new()),
        }
    }
}
