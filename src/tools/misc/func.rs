/** Create a new table by filling a vector with the specified `capacity` */
pub fn create_table<F, T>(capacity: usize, f: F) -> Vec<T>
where
    F: Fn() -> T,
{
    (0..capacity).map(|_| f()).collect()
}

/** Calculate the next prime integer */
pub fn next_prime(mut n: usize) -> usize {
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
