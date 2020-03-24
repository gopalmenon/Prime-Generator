const FIRST_PRIME: u64 = 2;

struct Prime {
    next_prime: u64,
}

impl Prime {
    fn new() -> Self {
        Prime {
            next_prime: FIRST_PRIME,
        }
    }
}

impl Iterator for Prime {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        // loop {
        //     self.next_prime += 1;
        //     break;
        // }
        Some(self.next_prime)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn first_prime() {
        assert_eq!(super::FIRST_PRIME, super::Prime::new().next().unwrap());
    }
}
