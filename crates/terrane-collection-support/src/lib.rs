use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::hash::{BuildHasherDefault, Hash, Hasher};
use std::sync::Arc;

use terrane_int_support::Int;
use unicode_segmentation::UnicodeSegmentation;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum IterationStep<T> {
    Item(T),
    End,
}

/// Unicode Character Database version used for collection grapheme iteration.
pub const UNICODE_DATA_VERSION: (u64, u64, u64) = (16, 0, 0);

const _: () = {
    assert!(unicode_segmentation::UNICODE_VERSION.0 == UNICODE_DATA_VERSION.0);
    assert!(unicode_segmentation::UNICODE_VERSION.1 == UNICODE_DATA_VERSION.1);
    assert!(unicode_segmentation::UNICODE_VERSION.2 == UNICODE_DATA_VERSION.2);
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AsyncIterationStep<T> {
    pub item: bool,
    pub end: bool,
    pub value: Option<T>,
}

impl<T> AsyncIterationStep<T> {
    pub fn item(value: T) -> Self {
        Self {
            item: true,
            end: false,
            value: Some(value),
        }
    }

    #[must_use]
    pub fn end() -> Self {
        Self {
            item: false,
            end: true,
            value: None,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AsyncSinkOutcome {
    pub accepted: bool,
    pub closed: bool,
}

impl AsyncSinkOutcome {
    #[must_use]
    pub fn from_accepted(accepted: bool) -> Self {
        Self {
            accepted,
            closed: !accepted,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Iterator<T> {
    items: Arc<Vec<T>>,
    index: usize,
    ended: bool,
}

impl<T> Iterator<T> {
    #[must_use]
    pub fn new(items: Vec<T>) -> Self {
        Self {
            items: Arc::new(items),
            index: 0,
            ended: false,
        }
    }
}

impl<T: Clone> Iterator<T> {
    #[must_use]
    #[expect(
        clippy::should_implement_trait,
        reason = "Terrane iteration returns an explicit typed step rather than Rust Option"
    )]
    pub fn next(&mut self) -> IterationStep<T> {
        next_indexed(&self.items, &mut self.index, &mut self.ended)
    }
}

#[derive(Clone, Debug)]
pub struct CollectionIterator<C> {
    collection: C,
    index: usize,
    ended: bool,
}

impl<C> CollectionIterator<C> {
    fn new(collection: C) -> Self {
        Self {
            collection,
            index: 0,
            ended: false,
        }
    }
}

impl<C: IndexedIteration> CollectionIterator<C> {
    #[must_use]
    #[expect(
        clippy::should_implement_trait,
        reason = "Terrane iteration returns an explicit typed step rather than Rust Option"
    )]
    pub fn next(&mut self) -> IterationStep<C::Item> {
        next_indexed(&self.collection, &mut self.index, &mut self.ended)
    }
}

#[derive(Clone, Debug)]
pub struct BorrowingIterator<'a, T> {
    items: &'a [T],
    index: usize,
    ended: bool,
}

impl<'a, T> BorrowingIterator<'a, T> {
    fn new(items: &'a [T]) -> Self {
        Self {
            items,
            index: 0,
            ended: false,
        }
    }

    #[must_use]
    #[expect(
        clippy::should_implement_trait,
        reason = "Terrane iteration returns an explicit typed step rather than Rust Option"
    )]
    pub fn next(&mut self) -> IterationStep<&'a T> {
        if self.ended {
            return IterationStep::End;
        }
        let Some(item) = self.items.get(self.index) else {
            self.ended = true;
            return IterationStep::End;
        };
        self.index += 1;
        IterationStep::Item(item)
    }
}

fn next_indexed<S, T>(source: &S, index: &mut usize, ended: &mut bool) -> IterationStep<T>
where
    S: IndexedIteration<Item = T>,
{
    if *ended {
        return IterationStep::End;
    }
    if let Some(item) = source.item_at(*index) {
        *index += 1;
        IterationStep::Item(item)
    } else {
        *ended = true;
        IterationStep::End
    }
}

#[doc(hidden)]
pub trait IndexedIteration {
    type Item;
    fn item_at(&self, index: usize) -> Option<Self::Item>;
}

impl<T: Clone> IndexedIteration for Arc<Vec<T>> {
    type Item = T;
    fn item_at(&self, index: usize) -> Option<T> {
        self.get(index).cloned()
    }
}

#[must_use]
pub fn slice_iterator<T: Clone>(items: &[T]) -> CollectionIterator<&[T]> {
    CollectionIterator::new(items)
}

impl<T: Clone> IndexedIteration for &[T] {
    type Item = T;
    fn item_at(&self, index: usize) -> Option<T> {
        self.get(index).cloned()
    }
}

pub trait Iterable {
    type Item: Clone + 'static;
    type Iter;
    fn terrane_iterator(&self) -> Self::Iter;
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct List<T>(Arc<Vec<T>>);

impl<T> List<T> {
    #[must_use]
    pub fn new(items: Vec<T>) -> Self {
        Self(Arc::new(items))
    }
    #[must_use]
    pub fn length(&self) -> i128 {
        self.0.len() as i128
    }
    /// Consumes the list and reuses its backing vector when it is uniquely owned.
    #[must_use]
    pub fn into_vec(self) -> Vec<T>
    where
        T: Clone,
    {
        Arc::unwrap_or_clone(self.0)
    }
    #[must_use]
    pub fn get(&self, index: usize) -> Option<&T> {
        self.0.get(index)
    }
    #[must_use]
    pub fn terrane_borrowing_iterator(&self) -> BorrowingIterator<'_, T> {
        BorrowingIterator::new(&self.0)
    }
    /// Returns the indexed item or an error when the index is outside the list.
    ///
    /// # Errors
    /// Returns [`IndexError`] when `index` is out of range.
    pub fn get_or_error(&self, index: usize) -> Result<T, IndexError>
    where
        T: Clone,
    {
        self.0
            .get(index)
            .cloned()
            .ok_or_else(|| IndexError::from_usize(index))
    }
    /// Returns exclusive access to the backing vector after separating shared storage.
    ///
    /// Compiler-generated bulk mutation regions use this once before a loop so repeated
    /// mutations do not perform an atomic uniqueness check for every element.
    #[doc(hidden)]
    #[inline]
    pub fn make_unique(&mut self) -> &mut Vec<T>
    where
        T: Clone,
    {
        Arc::make_mut(&mut self.0)
    }
}

impl<T: Clone> IntoIterator for List<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        Arc::unwrap_or_clone(self.0).into_iter()
    }
}

impl<T: Clone> List<T> {
    /// Replaces an indexed item.
    ///
    /// # Errors
    ///
    /// Returns [`IndexError`] when `index` is outside the list.
    #[inline]
    pub fn set(&mut self, index: usize, value: T) -> Result<(), IndexError> {
        let Some(slot) = self.make_unique().get_mut(index) else {
            return Err(IndexError::from_usize(index));
        };
        *slot = value;
        Ok(())
    }
    /// Removes and returns one indexed item.
    ///
    /// # Errors
    ///
    /// Returns [`IndexError`] when `index` is outside the list.
    #[inline]
    pub fn remove(&mut self, index: usize) -> Result<T, IndexError> {
        if index >= self.0.len() {
            return Err(IndexError::from_usize(index));
        }
        Ok(self.make_unique().remove(index))
    }
    #[inline]
    pub fn append(&mut self, value: T) {
        self.make_unique().push(value);
    }
    /// Removes every item in iteration order.
    #[inline]
    pub fn clear(&mut self) {
        self.make_unique().clear();
    }
    /// Stably orders the list in place after separating shared storage once.
    pub fn sort_by<F>(&mut self, compare: F)
    where
        F: FnMut(&T, &T) -> Ordering,
    {
        self.make_unique().sort_by(compare);
    }
}

#[must_use]
pub fn compare_float32_ascending(left: &f32, right: &f32) -> Ordering {
    compare_float_ascending(*left, *right)
}

#[must_use]
pub fn compare_float32_descending(left: &f32, right: &f32) -> Ordering {
    compare_float_descending(*left, *right)
}

#[must_use]
pub fn compare_float64_ascending(left: &f64, right: &f64) -> Ordering {
    compare_float_ascending(*left, *right)
}

#[must_use]
pub fn compare_float64_descending(left: &f64, right: &f64) -> Ordering {
    compare_float_descending(*left, *right)
}

fn compare_float_ascending<T: PartialOrd>(left: T, right: T) -> Ordering {
    match left.partial_cmp(&right) {
        Some(ordering) => ordering,
        None if left.partial_cmp(&left).is_none() => {
            if right.partial_cmp(&right).is_none() {
                Ordering::Equal
            } else {
                Ordering::Greater
            }
        }
        None => Ordering::Less,
    }
}

fn compare_float_descending<T: PartialOrd>(left: T, right: T) -> Ordering {
    match (left.partial_cmp(&left), right.partial_cmp(&right)) {
        (None, None) => Ordering::Equal,
        (None, Some(_)) => Ordering::Greater,
        (Some(_), None) => Ordering::Less,
        (Some(_), Some(_)) => right
            .partial_cmp(&left)
            .expect("non-NaN floating values are totally ordered"),
    }
}

impl<T: Clone + 'static> IndexedIteration for List<T> {
    type Item = T;
    fn item_at(&self, index: usize) -> Option<T> {
        self.0.get(index).cloned()
    }
}

impl<T: Clone + 'static> Iterable for List<T> {
    type Item = T;
    type Iter = CollectionIterator<Self>;
    fn terrane_iterator(&self) -> Self::Iter {
        CollectionIterator::new(self.clone())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Tuple<T>(Arc<Vec<T>>);
impl<T> Tuple<T> {
    #[must_use]
    pub fn new(items: Vec<T>) -> Self {
        Self(Arc::new(items))
    }
    #[must_use]
    pub fn length(&self) -> i128 {
        self.0.len() as i128
    }
    #[must_use]
    pub fn get(&self, index: usize) -> Option<&T> {
        self.0.get(index)
    }
    #[must_use]
    pub fn terrane_borrowing_iterator(&self) -> BorrowingIterator<'_, T> {
        BorrowingIterator::new(&self.0)
    }
    /// Consumes a uniquely owned tuple without cloning its elements.
    ///
    /// Returns the tuple unchanged when another persistent handle still shares its storage.
    ///
    /// # Errors
    /// Returns the original tuple when its backing storage is shared.
    pub fn try_into_iter(self) -> Result<std::vec::IntoIter<T>, Self> {
        Arc::try_unwrap(self.0).map(Vec::into_iter).map_err(Self)
    }
    /// Returns the indexed item or an error when the index is outside the tuple.
    ///
    /// # Errors
    /// Returns [`IndexError`] when `index` is out of range.
    pub fn get_or_error(&self, index: usize) -> Result<T, IndexError>
    where
        T: Clone,
    {
        self.0
            .get(index)
            .cloned()
            .ok_or_else(|| IndexError::from_usize(index))
    }
}
impl<T: Clone + 'static> IndexedIteration for Tuple<T> {
    type Item = T;
    fn item_at(&self, index: usize) -> Option<T> {
        self.0.get(index).cloned()
    }
}
impl<T: Clone + 'static> Iterable for Tuple<T> {
    type Item = T;
    type Iter = CollectionIterator<Self>;
    fn terrane_iterator(&self) -> Self::Iter {
        CollectionIterator::new(self.clone())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Entry<K, V> {
    pub key: K,
    pub value: V,
}
impl<K, V> Entry<K, V> {
    #[must_use]
    pub fn new(key: K, value: V) -> Self {
        Self { key, value }
    }
}

#[derive(Clone, Debug)]
struct StableHasher(u64);

impl Default for StableHasher {
    fn default() -> Self {
        Self(0xcbf2_9ce4_8422_2325)
    }
}

impl Hasher for StableHasher {
    fn finish(&self) -> u64 {
        self.0
    }

    fn write(&mut self, bytes: &[u8]) {
        for byte in bytes {
            self.0 ^= u64::from(*byte);
            self.0 = self.0.wrapping_mul(0x0000_0100_0000_01b3);
        }
    }
}

type FixedState = BuildHasherDefault<StableHasher>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Map<K: Eq + Hash, V>(Arc<indexmap::IndexMap<K, V, FixedState>>);
impl<K: Eq + Hash + Clone, V: Clone> Map<K, V> {
    #[must_use]
    pub fn new(entries: Vec<Entry<K, V>>) -> Self {
        let mut map = indexmap::IndexMap::with_hasher(FixedState::default());
        for entry in entries {
            map.insert(entry.key, entry.value);
        }
        Self(Arc::new(map))
    }
    #[must_use]
    pub fn length(&self) -> i128 {
        self.0.len() as i128
    }
    #[must_use]
    pub fn get(&self, key: &K) -> Option<&V> {
        self.0.get(key)
    }
    /// Returns the mapped value or an error when the key is absent.
    ///
    /// # Errors
    /// Returns [`MissingKey`] when `key` is absent.
    pub fn get_or_error(&self, key: &K) -> Result<V, MissingKey> {
        self.get(key).cloned().ok_or(MissingKey)
    }
    pub fn set(&mut self, key: K, value: V) {
        Arc::make_mut(&mut self.0).insert(key, value);
    }
    /// Removes and returns the mapped value without separating shared storage on a miss.
    ///
    /// # Errors
    /// Returns [`MissingKey`] when `key` is absent.
    pub fn remove(&mut self, key: &K) -> Result<V, MissingKey> {
        self.remove_checked(key).ok_or(MissingKey)
    }
    pub fn remove_checked(&mut self, key: &K) -> Option<V> {
        if let Some(map) = Arc::get_mut(&mut self.0) {
            return map.shift_remove(key);
        }
        self.0.contains_key(key).then(|| {
            Arc::make_mut(&mut self.0)
                .shift_remove(key)
                .expect("key presence was checked before copy-on-write separation")
        })
    }
    #[must_use]
    pub fn keys(&self) -> List<K> {
        List::new(self.0.keys().cloned().collect())
    }
    #[must_use]
    pub fn values(&self) -> List<V> {
        List::new(self.0.values().cloned().collect())
    }
    #[must_use]
    pub fn entries(&self) -> List<Entry<K, V>> {
        List::new(
            self.0
                .iter()
                .map(|(key, value)| Entry::new(key.clone(), value.clone()))
                .collect(),
        )
    }
}
impl<K: Eq + Hash + Clone + 'static, V: Clone + 'static> IndexedIteration for Map<K, V> {
    type Item = Entry<K, V>;
    fn item_at(&self, index: usize) -> Option<Self::Item> {
        self.0
            .get_index(index)
            .map(|(key, value)| Entry::new(key.clone(), value.clone()))
    }
}
impl<K: Eq + Hash + Clone + 'static, V: Clone + 'static> Iterable for Map<K, V> {
    type Item = Entry<K, V>;
    type Iter = CollectionIterator<Self>;
    fn terrane_iterator(&self) -> Self::Iter {
        CollectionIterator::new(self.clone())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Set<T: Eq + Hash>(Arc<indexmap::IndexSet<T, FixedState>>);
impl<T: Eq + Hash + Clone> Set<T> {
    #[must_use]
    pub fn new(items: Vec<T>) -> Self {
        let mut set = indexmap::IndexSet::with_hasher(FixedState::default());
        set.extend(items);
        Self(Arc::new(set))
    }
    #[must_use]
    pub fn length(&self) -> i128 {
        self.0.len() as i128
    }
    #[must_use]
    pub fn contains(&self, item: &T) -> bool {
        self.0.contains(item)
    }
    pub fn add(&mut self, item: T) {
        Arc::make_mut(&mut self.0).insert(item);
    }
    pub fn remove(&mut self, item: &T) -> bool {
        Arc::make_mut(&mut self.0).shift_remove(item)
    }
}
impl<T: Eq + Hash + Clone + 'static> IndexedIteration for Set<T> {
    type Item = T;
    fn item_at(&self, index: usize) -> Option<T> {
        self.0.get_index(index).cloned()
    }
}
impl<T: Eq + Hash + Clone + 'static> Iterable for Set<T> {
    type Item = T;
    type Iter = CollectionIterator<Self>;
    fn terrane_iterator(&self) -> Self::Iter {
        CollectionIterator::new(self.clone())
    }
}

fn stable_hash<T: Hash>(value: &T) -> u64 {
    let mut hasher = StableHasher::default();
    value.hash(&mut hasher);
    hasher.finish()
}

fn insert_by_stable_hash<T: Hash>(items: &mut Vec<T>, item: T) -> usize {
    let hash = stable_hash(&item);
    let index = items.partition_point(|candidate| stable_hash(candidate) <= hash);
    items.insert(index, item);
    index
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct UnorderedMapData<K: Eq + Hash, V> {
    values: HashMap<K, V, FixedState>,
    iteration_keys: Vec<K>,
    iteration_positions: HashMap<K, usize, FixedState>,
}

impl<K: Eq + Hash, V> UnorderedMapData<K, V> {
    fn indexed_value(&self, key: &K) -> &V {
        self.values.get(key).expect("indexed key must exist")
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UnorderedMap<K: Eq + Hash, V>(Arc<UnorderedMapData<K, V>>);

impl<K: Eq + Hash + Clone, V: Clone> UnorderedMap<K, V> {
    #[must_use]
    pub fn new(entries: Vec<Entry<K, V>>) -> Self {
        let mut values = HashMap::with_hasher(FixedState::default());
        let mut iteration_keys = Vec::new();
        for entry in entries {
            if !values.contains_key(&entry.key) {
                iteration_keys.push(entry.key.clone());
            }
            values.insert(entry.key, entry.value);
        }
        iteration_keys.sort_by_key(stable_hash);
        let iteration_positions = iteration_keys
            .iter()
            .cloned()
            .enumerate()
            .map(|(index, key)| (key, index))
            .collect();
        Self(Arc::new(UnorderedMapData {
            values,
            iteration_keys,
            iteration_positions,
        }))
    }
    #[must_use]
    pub fn length(&self) -> i128 {
        self.0.values.len() as i128
    }
    #[must_use]
    pub fn get(&self, key: &K) -> Option<&V> {
        self.0.values.get(key)
    }
    /// Returns the mapped value or an error when the key is absent.
    ///
    /// # Errors
    /// Returns [`MissingKey`] when `key` is absent.
    pub fn get_or_error(&self, key: &K) -> Result<V, MissingKey> {
        self.get(key).cloned().ok_or(MissingKey)
    }
    pub fn set(&mut self, key: K, value: V) {
        let data = Arc::make_mut(&mut self.0);
        if !data.values.contains_key(&key) {
            let index = insert_by_stable_hash(&mut data.iteration_keys, key.clone());
            for (position, existing) in data.iteration_keys[index..].iter().enumerate() {
                data.iteration_positions
                    .insert(existing.clone(), index + position);
            }
        }
        data.values.insert(key, value);
    }
    /// Removes and returns the mapped value without separating shared storage on a miss.
    ///
    /// # Errors
    /// Returns [`MissingKey`] when `key` is absent.
    pub fn remove(&mut self, key: &K) -> Result<V, MissingKey> {
        self.remove_checked(key).ok_or(MissingKey)
    }
    pub fn remove_checked(&mut self, key: &K) -> Option<V> {
        if Arc::get_mut(&mut self.0).is_none() && !self.0.values.contains_key(key) {
            return None;
        }
        let data = Arc::make_mut(&mut self.0);
        let value = data.values.remove(key)?;
        let index = data
            .iteration_positions
            .remove(key)
            .expect("indexed key must retain its iteration position");
        data.iteration_keys.swap_remove(index);
        if let Some(moved) = data.iteration_keys.get(index) {
            data.iteration_positions.insert(moved.clone(), index);
        }
        Some(value)
    }
    #[must_use]
    pub fn keys(&self) -> List<K> {
        List::new(self.0.iteration_keys.clone())
    }
    #[must_use]
    pub fn values(&self) -> List<V> {
        List::new(
            self.0
                .iteration_keys
                .iter()
                .map(|key| self.0.indexed_value(key).clone())
                .collect(),
        )
    }
    #[must_use]
    pub fn entries(&self) -> List<Entry<K, V>> {
        List::new(
            self.0
                .iteration_keys
                .iter()
                .map(|key| Entry::new(key.clone(), self.0.indexed_value(key).clone()))
                .collect(),
        )
    }
}

impl<K: Eq + Hash + Clone + 'static, V: Clone + 'static> IndexedIteration for UnorderedMap<K, V> {
    type Item = Entry<K, V>;
    fn item_at(&self, index: usize) -> Option<Self::Item> {
        let key = self.0.iteration_keys.get(index)?;
        Some(Entry::new(key.clone(), self.0.indexed_value(key).clone()))
    }
}

impl<K: Eq + Hash + Clone + 'static, V: Clone + 'static> Iterable for UnorderedMap<K, V> {
    type Item = Entry<K, V>;
    type Iter = CollectionIterator<Self>;
    fn terrane_iterator(&self) -> Self::Iter {
        CollectionIterator::new(self.clone())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct UnorderedSetData<T: Eq + Hash> {
    values: HashSet<T, FixedState>,
    iteration_items: Vec<T>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UnorderedSet<T: Eq + Hash>(Arc<UnorderedSetData<T>>);

impl<T: Eq + Hash + Clone> UnorderedSet<T> {
    #[must_use]
    pub fn new(items: Vec<T>) -> Self {
        let mut values = HashSet::with_hasher(FixedState::default());
        let mut iteration_items = Vec::new();
        for item in items {
            if values.insert(item.clone()) {
                iteration_items.push(item);
            }
        }
        iteration_items.sort_by_key(stable_hash);
        Self(Arc::new(UnorderedSetData {
            values,
            iteration_items,
        }))
    }
    #[must_use]
    pub fn length(&self) -> i128 {
        self.0.values.len() as i128
    }
    #[must_use]
    pub fn contains(&self, item: &T) -> bool {
        self.0.values.contains(item)
    }
    pub fn add(&mut self, item: T) {
        let data = Arc::make_mut(&mut self.0);
        if data.values.insert(item.clone()) {
            insert_by_stable_hash(&mut data.iteration_items, item);
        }
    }
    pub fn remove(&mut self, item: &T) -> bool {
        let data = Arc::make_mut(&mut self.0);
        if !data.values.remove(item) {
            return false;
        }
        data.iteration_items.retain(|candidate| candidate != item);
        true
    }
}

impl<T: Eq + Hash + Clone + 'static> IndexedIteration for UnorderedSet<T> {
    type Item = T;
    fn item_at(&self, index: usize) -> Option<T> {
        self.0.iteration_items.get(index).cloned()
    }
}

impl<T: Eq + Hash + Clone + 'static> Iterable for UnorderedSet<T> {
    type Item = T;
    type Iter = CollectionIterator<Self>;
    fn terrane_iterator(&self) -> Self::Iter {
        CollectionIterator::new(self.clone())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Range {
    start: Int,
    end: Int,
    step: Int,
    inclusive: bool,
}
impl Range {
    /// Constructs a half-open range.
    ///
    /// # Errors
    /// Returns [`RangeStepError`] when `step` is zero.
    pub fn new(start: Int, end: Int, step: Int) -> Result<Self, RangeStepError> {
        if step == Int::from(0_i64) {
            return Err(RangeStepError);
        }
        Ok(Self {
            start,
            end,
            step,
            inclusive: false,
        })
    }
    /// Constructs an inclusive range.
    ///
    /// # Errors
    /// Returns [`RangeStepError`] when `step` is zero.
    pub fn through(start: Int, end: Int, step: Int) -> Result<Self, RangeStepError> {
        Self::new(start, end, step).map(|range| Self {
            inclusive: true,
            ..range
        })
    }
}
#[derive(Clone, Debug)]
pub struct RangeIterator {
    current: Int,
    end: Int,
    step: Int,
    inclusive: bool,
    ascending: bool,
    ended: bool,
}

impl RangeIterator {
    #[must_use]
    #[expect(
        clippy::should_implement_trait,
        reason = "Terrane iteration returns an explicit typed step rather than Rust Option"
    )]
    pub fn next(&mut self) -> IterationStep<Int> {
        if self.ended {
            return IterationStep::End;
        }
        let in_bounds = if self.ascending {
            self.current < self.end || (self.inclusive && self.current == self.end)
        } else {
            self.current > self.end || (self.inclusive && self.current == self.end)
        };
        if !in_bounds {
            self.ended = true;
            return IterationStep::End;
        }
        let item = self.current.clone();
        self.current = self.current.clone() + self.step.clone();
        IterationStep::Item(item)
    }
}

impl Iterable for Range {
    type Item = Int;
    type Iter = RangeIterator;
    fn terrane_iterator(&self) -> Self::Iter {
        RangeIterator {
            current: self.start.clone(),
            end: self.end.clone(),
            step: self.step.clone(),
            inclusive: self.inclusive,
            ascending: self.step > Int::from(0_i64),
            ended: false,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IndexError {
    pub index: Int,
}
impl IndexError {
    #[must_use]
    pub fn from_usize(index: usize) -> Self {
        Self {
            index: Int::from_u128(index as u128),
        }
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MissingKey;
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RangeStepError;
impl std::fmt::Display for IndexError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "index {} is out of range", self.index)
    }
}

impl std::error::Error for IndexError {}

impl std::fmt::Display for MissingKey {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("key is absent")
    }
}

impl std::error::Error for MissingKey {}
impl std::fmt::Display for RangeStepError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("range step is zero")
    }
}

impl std::error::Error for RangeStepError {}
/// Converts an adaptive integer to a collection index.
///
/// # Errors
/// Returns [`IndexError`] when `index` is negative or does not fit in `usize`.
pub fn index_from_int(index: &Int) -> Result<usize, IndexError> {
    index.as_usize().ok_or_else(|| IndexError {
        index: index.clone(),
    })
}
/// Returns one byte from a byte sequence.
///
/// # Errors
/// Returns [`IndexError`] when `index` is outside the sequence.
pub fn byte_at(value: &[u8], index: usize) -> Result<u8, IndexError> {
    value
        .get(index)
        .copied()
        .ok_or_else(|| IndexError::from_usize(index))
}

/// Returns the bytes selected by a Terrane range.
///
/// # Errors
/// Returns [`IndexError`] when any selected index is negative, cannot fit in `usize`, or is outside
/// the sequence.
pub fn byte_slice(value: &[u8], range: &Range) -> Result<Vec<u8>, IndexError> {
    let mut selected = Vec::new();
    let mut indices = range.terrane_iterator();
    loop {
        match indices.next() {
            IterationStep::Item(index) => {
                let index = index_from_int(&index)?;
                selected.push(byte_at(value, index)?);
            }
            IterationStep::End => return Ok(selected),
        }
    }
}

#[must_use]
pub fn string_iterator(value: &str) -> Iterator<String> {
    Iterator::new(value.graphemes(true).map(str::to_owned).collect())
}
#[must_use]
pub fn bytes_iterator(value: &[u8]) -> Iterator<u8> {
    Iterator::new(value.to_vec())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sticky_end_never_revisits_source() {
        let mut iterator = Iterator::new(vec![None::<u8>]);
        assert_eq!(iterator.next(), IterationStep::Item(None));
        assert_eq!(iterator.next(), IterationStep::End);
        assert_eq!(iterator.next(), IterationStep::End);
    }

    #[test]
    fn stable_hasher_owns_its_algorithm() {
        let mut hasher = StableHasher::default();
        hasher.write(b"terrane");
        assert_eq!(hasher.finish(), 0x3f87_dd9c_872a_eb2c);
    }

    #[test]
    fn collection_iteration_does_not_clone_items_before_advancing() {
        use std::sync::atomic::{AtomicUsize, Ordering};

        #[derive(Debug)]
        struct CountedClone(Arc<AtomicUsize>);

        impl Clone for CountedClone {
            fn clone(&self) -> Self {
                self.0.fetch_add(1, Ordering::Relaxed);
                Self(Arc::clone(&self.0))
            }
        }

        let clones = Arc::new(AtomicUsize::new(0));
        let list = List::new(vec![
            CountedClone(Arc::clone(&clones)),
            CountedClone(Arc::clone(&clones)),
        ]);
        let mut iterator = list.terrane_iterator();
        assert_eq!(clones.load(Ordering::Relaxed), 0);
        assert!(matches!(iterator.next(), IterationStep::Item(_)));
        assert_eq!(clones.load(Ordering::Relaxed), 1);
    }

    #[test]
    fn list_assignment_separates_on_first_mutation() {
        let original = List::new(vec![1]);
        let mut copy = original.clone();
        copy.append(2);
        assert_eq!(original.length(), 1);
        assert_eq!(copy.length(), 2);
    }

    #[test]
    fn list_bulk_mutation_separates_shared_storage_once() {
        let original = List::new(vec![1]);
        let mut copy = original.clone();
        let values = copy.make_unique();
        values.reserve(2);
        values.push(2);
        values.push(3);
        assert_eq!(original.0.as_slice(), &[1]);
        assert_eq!(copy.0.as_slice(), &[1, 2, 3]);
    }

    #[test]
    fn tuple_consumption_moves_unique_items_and_rejects_shared_storage() {
        let unique = Tuple::new(vec![String::from("moved")]);
        assert_eq!(
            unique.try_into_iter().unwrap().collect::<Vec<_>>(),
            ["moved"]
        );

        let shared = Tuple::new(vec![String::from("shared")]);
        let alias = shared.clone();
        assert!(shared.try_into_iter().is_err());
        assert_eq!(alias.get(0).map(String::as_str), Some("shared"));
    }

    #[test]
    fn unordered_insertion_matches_bulk_construction_order() {
        let entries = vec![
            Entry::new("third", 3),
            Entry::new("first", 1),
            Entry::new("second", 2),
        ];
        let bulk_map = UnorderedMap::new(entries.clone());
        let mut inserted_map = UnorderedMap::new(Vec::new());
        for entry in entries {
            inserted_map.set(entry.key, entry.value);
        }
        assert_eq!(inserted_map.keys(), bulk_map.keys());

        let items = vec!["third", "first", "second"];
        let bulk_set = UnorderedSet::new(items.clone());
        let mut inserted_set = UnorderedSet::new(Vec::new());
        for item in items {
            inserted_set.add(item);
        }
        assert_eq!(inserted_set, bulk_set);
    }

    #[test]
    fn map_removal_preserves_order_and_avoids_separating_misses() {
        let mut map = Map::new(vec![
            Entry::new("first", 1),
            Entry::new("second", 2),
            Entry::new("third", 3),
        ]);
        let alias = map.clone();
        assert_eq!(map.remove_checked(&"absent"), None);
        assert!(Arc::ptr_eq(&map.0, &alias.0));
        assert_eq!(map.remove(&"second"), Ok(2));
        assert!(!Arc::ptr_eq(&map.0, &alias.0));
        map.set("second", 4);
        assert_eq!(map.keys().into_vec(), ["first", "third", "second"]);
        assert_eq!(alias.keys().into_vec(), ["first", "second", "third"]);
    }

    #[test]
    fn unordered_map_removal_updates_constant_time_iteration_index() {
        let mut map = UnorderedMap::new(vec![
            Entry::new("first", 1),
            Entry::new("second", 2),
            Entry::new("third", 3),
        ]);
        let alias = map.clone();
        assert_eq!(map.remove_checked(&"absent"), None);
        assert!(Arc::ptr_eq(&map.0, &alias.0));
        assert_eq!(map.remove(&"second"), Ok(2));
        assert!(!Arc::ptr_eq(&map.0, &alias.0));
        assert_eq!(map.length(), 2);
        assert_eq!(map.entries().length(), 2);
        assert_eq!(alias.length(), 3);
    }

    #[test]
    fn stable_sort_separates_shared_storage_once_and_retains_equal_order() {
        let original = List::new(vec![(2, 'a'), (1, 'b'), (2, 'c'), (1, 'd')]);
        let mut ascending = original.clone();
        ascending.sort_by(|left, right| left.0.cmp(&right.0));
        assert_eq!(
            ascending.into_vec(),
            [(1, 'b'), (1, 'd'), (2, 'a'), (2, 'c')]
        );
        assert_eq!(
            original.into_vec(),
            [(2, 'a'), (1, 'b'), (2, 'c'), (1, 'd')]
        );
    }

    #[test]
    fn floating_sort_keeps_nan_last_and_signed_zero_stable() {
        let first_nan = f64::from_bits(0x7ff8_0000_0000_0001);
        let second_nan = f64::from_bits(0xfff8_0000_0000_0002);
        let values = vec![first_nan, -0.0, 2.0, 0.0, -1.0, second_nan];

        let mut ascending = List::new(values.clone());
        ascending.sort_by(compare_float64_ascending);
        let ascending = ascending.into_vec();
        assert_eq!(&ascending[..4], &[-1.0, -0.0, 0.0, 2.0]);
        assert_eq!(ascending[1].to_bits(), (-0.0_f64).to_bits());
        assert_eq!(ascending[4].to_bits(), first_nan.to_bits());
        assert_eq!(ascending[5].to_bits(), second_nan.to_bits());

        let mut descending = List::new(values);
        descending.sort_by(compare_float64_descending);
        let descending = descending.into_vec();
        assert_eq!(&descending[..4], &[2.0, -0.0, 0.0, -1.0]);
        assert_eq!(descending[1].to_bits(), (-0.0_f64).to_bits());
        assert_eq!(descending[4].to_bits(), first_nan.to_bits());
        assert_eq!(descending[5].to_bits(), second_nan.to_bits());
    }
}
