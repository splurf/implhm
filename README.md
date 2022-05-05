# implhm

##### Simplified library of *collision-handling* **HashMaps**

## Getting Started
Place *implhm* in your `Cargo.toml`:
```toml
[dependencies]
implhm = "1.0.6"
```

## Features
There are several different methods for handling collision. *implhm* provides the most basic implementations. The following features are available:

+ **separate-chaining** (*default*)
+ **open-addressing**
    + *double-hashing*
    + *linear-probing*
    + *quadratic-probing*
+ **separate-chaining-test**
+ **open-addressing-test**
    + *double-hashing-test*
    + *linear-probing-test*
    + *quadratic-probing-test*

Here is an example of using a single feature:
```toml
[dependencies]
implhm = { version = "1.0.6", default-features = false, features = ["quadratic-probing"] }
```

## Usage
A basic example of hash collision using two strings:
```rust
use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

fn hash<T: Hash>(key: T) -> u64 {
    let mut state = DefaultHasher::new();
    key.hash(&mut state);
    state.finish() % 17
}

fn main() {
    /*
        When passed through the `hash` function,
        `orange` and `blueberry` both equal `8`
    */
    let a = hash("orange");
    let b = hash("blueberry");
    /*
        If *collision* isn't handled, then the *value*
        ("orange") at the location of the *key* (`8`)
        would be replaced with the *value* ("blueberry")
    */
    assert_eq!(a, b)
}
```
Here, collision is completely handled by *separate chaining*:
```rust
use implhm::{Map, MapMut, SCHashMap};

fn main() {
    let mut map = SCHashMap::default();

    map.insert("orange", "ORANGE");
    map.insert("blueberry", "BLUEBERRY");
    /*
        In the case of *separate chaining*, collision is
        handled by placing any key-pairs that calculate to
        the same hash into an ordered list at that index.
    */
    assert_eq!(map.get("orange"), Some(&"ORANGE"));
    assert_eq!(map.get("blueberry"), Some(&"BLUEBERRY"));
}
```
