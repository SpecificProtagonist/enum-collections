use std::{
    alloc::{alloc, Layout},
    marker::PhantomData,
    slice::from_raw_parts_mut,
};

use crate::Enumerated;

/// A key-value table optimized for Enums used as keys. Initialized with `V`'s [Default] value.
///
/// ```
/// use enum_collections::{enum_collections, EnumTable, Enumerated};
/// #[enum_collections]
/// enum Letter {
///     A,
///     B,
/// }
///
/// let mut map: EnumTable<Letter, u8> = EnumTable::new();
/// map.insert(Letter::A, 42);
/// assert_eq!(&42u8, map.get(Letter::A));
/// assert_eq!(&u8::default(), map.get(Letter::B));
///```
pub struct EnumTable<'a, K, V>
where
    K: Enumerated,
    V: Default,
{
    values: &'a mut [V],
    _key_phantom_data: PhantomData<K>,
}

impl<'a, K, V> EnumTable<'a, K, V>
where
    K: Enumerated,
    V: Default,
{
    /// Creates a new [EnumTable], with pre-allocated space for all keys of the enum `K`. With the underlying array righsized,
    /// no resizing is further required. All values are initialized with `V`'s [Default] value.
    pub fn new() -> Self {
        Self {
            values: unsafe {
                let raw_memory = alloc(Layout::array::<V>(K::len()).unwrap());
                let values_array: &'a mut [V] = from_raw_parts_mut(raw_memory as *mut V, K::len());
                for value in values_array.iter_mut() {
                    *value = V::default();
                }
                values_array
            },
            _key_phantom_data: PhantomData {},
        }
    }

    /// Obtain a value for given `key`, always returning a value `V`,
    /// as the EnumTable is pre-initialized with defaults.
    ///
    /// ### Args
    /// - `key` - Instance of `K`, used to look up the corresponding value.
    #[inline]
    pub fn get(&self, key: K) -> &V {
        &self.values[key.position()]
    }

    /// Stores given `value` under the provided `key`. Overrides any existing corresponding value.
    ///
    /// ### Args
    /// - `key` - The instance of `K` the value inserted can be looked up for.
    /// - `values` - Value to bind to `K`.
    #[inline]
    pub fn insert(&mut self, key: K, value: V) {
        self.values[key.position()] = value
    }
}

impl<'a, K, V> Default for EnumTable<'a, K, V>
where
    K: Enumerated,
    V: Default,
{
    /// Constructs a new instance, capable of holding all values of key `K` without further resizing.
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use crate::Enumerated;
    use enum_collections_macros::enum_collections;

    use super::EnumTable;

    #[enum_collections]
    enum Letter {
        A,
        B,
    }

    #[derive(Eq, PartialEq, Debug, Clone)]
    struct Value {
        name: String,
    }

    impl Value {
        fn new(name: String) -> Self {
            Self { name }
        }
    }

    impl Default for Value {
        fn default() -> Self {
            Self {
                name: "Non-empty default".to_owned(),
            }
        }
    }

    #[test]
    fn new_all_default() {
        let enum_table = EnumTable::<Letter, Value>::new();
        for index in 0..Letter::len() {
            assert_eq!(Value::default(), enum_table.values[index]);
        }
    }

    #[test]
    fn inserts() {
        let mut enum_table = EnumTable::<Letter, Value>::new();
        let inserted_value = Value::new("Hello".to_string());
        enum_table.insert(Letter::A, inserted_value.clone());
        assert_eq!(&inserted_value, enum_table.get(Letter::A));
        assert_eq!(&Value::default(), enum_table.get(Letter::B));
    }
}