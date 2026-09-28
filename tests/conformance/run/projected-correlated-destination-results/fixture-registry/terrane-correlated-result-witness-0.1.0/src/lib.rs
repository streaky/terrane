use std::collections::BTreeMap;

pub fn correlated_values<Key, Index, Value>() -> BTreeMap<Key, Vec<BTreeMap<Index, Value>>>
where
    Key: Ord + Default,
    Index: Ord + Default,
    Value: Default,
{
    BTreeMap::default()
}
