mod base;
mod ordered_map;

use {
    super::{create_table, entry::*, next_prime, DEFAULT_CAPACITY},
    ordered_map::OrderedMap,
};

pub use base::SCHashMap;
