#![no_std]

//! Rust map that can be defined in a const context.
//!
//! There are two ways to create it:
//!
//! ```rust
//! use const_lookup_map::{ConstLookup, lookup};
//!
//! const LOOKUP_MACRO: ConstLookup<3, &str, &str> = lookup! {
//!     "best" => "better",
//!     "test" => "testing",
//!     "guessed" => "guessing",
//! };
//! ```
//!
//! ```rust
//! use const_lookup_map::ConstLookup;
//!
//! pub const LOOKUP: ConstLookup<4, &str, &str> = ConstLookup::new(
//!     ["bye", "hallo", "hey", "test"],
//!     [
//!         "bye.example.com",
//!         "hallo.example.com",
//!         "hey.example.com",
//!         "test.example.com",
//!     ],
//! );
//! ```
//!
//! One note; The keys should be in order/sorted because the get method will use this to effienctly get the value. See [`ConstLookup::check_sorted`]
//!
//! # Usage
//!
//! ```rust
//! use const_lookup_map::{ConstLookup, lookup};
//!
//! const LOOKUP: ConstLookup<3, &str, &str> = lookup! {
//!     "best" => "better",
//!     "test" => "testing",
//!     "guessed" => "guessing",
//! };
//!
//! fn my_function() {
//!   assert_eq!(LOOKUP.get(&"best"), Some(&"better"));
//!   assert_eq!(LOOKUP[&"best"], "better");
//! }
//! # my_function()
//! ```

fn is_sorted<I>(data: I) -> bool
where
    I: IntoIterator,
    I::Item: Ord,
{
    let mut it = data.into_iter();
    match it.next() {
        None => true,
        Some(first) => it
            .scan(first, |state, next| {
                let cmp = *state <= next;
                *state = next;
                Some(cmp)
            })
            .all(|b| b),
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ConstLookup<const N: usize, K: Ord, V> {
    pub keys: [K; N],
    pub values: [V; N],
}

impl<const N: usize, K: Ord, V> ConstLookup<N, K, V> {
    /// Returns the number of elements in the map.
    pub const fn len(&self) -> usize {
        N
    }

    pub const fn new(keys: [K; N], values: [V; N]) -> ConstLookup<N, K, V> {
        ConstLookup { keys, values }
    }

    /// because keys cannot be checked at compiletime if it is sorted, add this to your tests:
    ///
    /// ```rust
    /// #[test]
    /// fn verify_my_lookup_is_sorted() {
    ///     assert!(MY_LOOKUP.check_sorted(), "MY_LOOKUP is not sorted")
    /// }
    /// ```
    pub fn check_sorted(&self) -> bool {
        is_sorted(&self.keys)
    }

    /// Returns a reference to the value corresponding to the key.
    pub fn get(&self, key: &K) -> Option<&V> {
        let index = self.keys.binary_search(key).ok()?;
        self.values.get(index)
    }

    /// Returns true if the map contains a value for the specified key.
    pub fn contains_key(&self, key: &K) -> bool {
        self.keys.binary_search(key).is_ok()
    }
}

// impl<const N: usize, K: Ord, V> ConstLookup<N, K, V> {
//     pub const fn const_contains<Q: ~const PartialEq>(&self, key: &K) -> bool {
//         let mut i = 0;
//         while i < self.keys.len() {
//             if key == &self.keys[i] {
//                 return true;
//             }
//             i = i + 1;
//         }

//         false
//     }
// }

impl<const N: usize, K: Ord, V> core::ops::Index<&K> for ConstLookup<N, K, V> {
    type Output = V;

    fn index(&self, index: &K) -> &V {
        self.get(index)
            .expect("key not found in ConstLookup, use `get` for a safe option")
    }
}

#[cfg(test)]
const LOOKUP: ConstLookup<4, &str, &str> = ConstLookup::new(
    ["bye", "hallo", "hey", "test"],
    [
        "bye.example.com",
        "hallo.example.com",
        "hey.example.com",
        "test.example.com",
    ],
);

#[macro_export(local_inner_macros)]
macro_rules! lookup {
    (@single $($x:tt)*) => (());
    (@count $($rest:expr),*) => (<[()]>::len(&[$(lookup!(@single $rest)),*]));

    ($($key:expr => $value:expr,)+) => { lookup!($($key => $value),+) };
    ($($key:expr => $value:expr),*) => {
        {
            const _CAP: usize = lookup!(@count $($key),*);
            let mut keys: [_; _CAP] = unsafe {
                let arr = core::mem::MaybeUninit::uninit();
                arr.assume_init()
            };

            let mut values: [_; _CAP] = unsafe {
                let arr = core::mem::MaybeUninit::uninit();
                arr.assume_init()
            };

            let mut i = 0;
            $(
                keys[i] = $key;
                values[i] = $value;
                i+=1;
            )*

            _ = i;

            ConstLookup::new(keys, values)
        }
    };
}

#[cfg(test)]
const fn large() -> bool {
    LOOKUP.len() > 100
}

#[test]
fn verify_my_lookup_is_sorted() {
    assert!(LOOKUP.check_sorted(), "LOOKUP is not sorted")
}

#[test]
fn get_test() {
    assert_eq!(Some(&"hey.example.com"), LOOKUP.get(&"hey"));
}

#[test]
fn index_test() {
    assert_eq!("hey.example.com", LOOKUP[&"hey"]);
}

#[test]
fn const_func() {
    assert!(!large())
}

#[cfg(test)]
const LOOKUP_MACRO: ConstLookup<3, &str, &str> = lookup! {
    "best" => "better",
    "test" => "testing",
    "guessed" => "guessing",
};

#[test]
fn lookup_macro_works_for_const() {
    assert_eq!(
        ConstLookup {
            keys: ["best", "test", "guessed"],
            values: ["better", "testing", "guessing"]
        },
        LOOKUP_MACRO
    );
}

#[test]
fn lookup_macro_works_for_normal_env() {
    let lookup = lookup! {
        "best" => "better",
        "test" => "testing",
        "guessed" => "guessing",
    };

    assert_eq!(
        ConstLookup {
            keys: ["best", "test", "guessed"],
            values: ["better", "testing", "guessing"]
        },
        lookup
    );
}
