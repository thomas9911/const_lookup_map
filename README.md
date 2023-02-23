# const_lookup_map

Rust map that can be defined in a const context.

There are two ways to create it:

```rust
use const_lookup_map::{ConstLookup, lookup};

const LOOKUP_MACRO: ConstLookup<3, &str, &str> = lookup! {
    "best" => "better",
    "test" => "testing",
    "guessed" => "guessing",
};
```

```rust
use const_lookup_map::ConstLookup;

pub const LOOKUP: ConstLookup<4, &str, &str> = ConstLookup::new(
    ["bye", "hallo", "hey", "test"],
    [
        "bye.example.com",
        "hallo.example.com",
        "hey.example.com",
        "test.example.com",
    ],
);
```

One note; The keys should be in order/sorted because the get method will use this to effienctly get the value. See [`ConstLookup::check_sorted`]

## Usage

```rust
use const_lookup_map::{ConstLookup, lookup};

const LOOKUP: ConstLookup<3, &str, &str> = lookup! {
    "best" => "better",
    "test" => "testing",
    "guessed" => "guessing",
};

fn my_function() {
  assert_eq!(LOOKUP.get(&"best"), Some(&"better"));
  assert_eq!(LOOKUP[&"best"], "better");
}
```
