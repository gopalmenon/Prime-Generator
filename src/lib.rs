const FIRST_PRIME_LESS_ONE: u64 = 1;

/// Prime number struct that lets you call an iterator to generate primes
///
/// # Examples
///
/// ```
/// use primes::Prime;
/// let mut prime = Prime::new();
/// for _ in 1..10 {
///     prime.next();
/// }
/// assert_eq!(prime.next().unwrap(), 29);
/// ```
pub struct Prime {
    next_prime: u64,
}

impl Prime {
    pub fn new() -> Self {
        Prime {
            next_prime: FIRST_PRIME_LESS_ONE,
        }
    }
    fn is_prime(&self, prime_candidate: u64) -> bool {
        let upper_limit = (prime_candidate as f64).sqrt() as u64;
        for divisor in 2..=upper_limit {
            if prime_candidate % divisor == 0 {
                return false;
            }
        }
        true
    }
}

impl Iterator for Prime {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.next_prime == 1 || self.next_prime == 2 {
                self.next_prime += 1;
            } else {
                self.next_prime += 2;
            }

            if self.is_prime(self.next_prime) {
                break;
            }
        }
        Some(self.next_prime)
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn first_prime() {
        assert_eq!(super::Prime::new().next().unwrap(), 2);
    }

    #[test]
    fn second_prime() {
        let mut prime = super::Prime::new();
        prime.next();
        assert_eq!(prime.next().unwrap(), 3);
    }

    #[test]
    fn tenth_prime() {
        let mut prime = super::Prime::new();
        for _ in 1..10 {
            prime.next();
        }
        assert_eq!(prime.next().unwrap(), 29);
    }

    #[test]
    fn twentieth_prime() {
        let mut prime = super::Prime::new();
        for _ in 1..20 {
            prime.next();
        }
        assert_eq!(prime.next().unwrap(), 71);
    }
}
