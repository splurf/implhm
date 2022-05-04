use super::entry;

#[cfg(feature = "separate-chaining")]
mod separate_chaining;

#[cfg(feature = "separate-chaining")]
pub use separate_chaining::*;

#[cfg(feature = "open-addressing")]
mod open_addressing;

#[cfg(feature = "open-addressing")]
pub use open_addressing::*;

/** Default capacity for every container */
const DEFAULT_CAPACITY: usize = 17;

/** Create a new table by filling a vector with the specified `capacity` */
fn create_table<F, T>(capacity: usize, f: F) -> Vec<T>
where
    F: Fn(usize) -> T,
{
    (0..capacity).map(|_| f(capacity)).collect()
}

/** Calculate the next prime integer */
fn next_prime(mut n: usize) -> usize {
    loop {
        n += 1;

        let mut i = 0;

        for j in 1..=n {
            if n % j == 0 {
                i += 1;
            }
        }
        if i == 2 {
            break n;
        }
    }
}
