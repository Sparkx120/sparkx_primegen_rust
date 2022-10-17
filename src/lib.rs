pub mod common {
    pub struct Config {
        pub prime_range_start: u64,
        pub prime_range_end: u64,
    }

    impl Config {
        pub fn build() -> Config {
            Config {
                prime_range_start: 2,
                prime_range_end:10000000000,
            }
        }
    }
}

pub mod driver {
    use std::{io, io::Write};
    use super::common::Config;
    use super::generators;

    pub fn run() {
        let config = Config::build();
        println!("Finding Prime Numbers between {} and {}", config.prime_range_start, config.prime_range_end);

        let primes = generators::sieve_of_eratosthenes(&config);
        println!("{} primes found under {}", primes.len(), config.prime_range_end);
        //for prime in primes {
        //    print!("{}, ", prime);
        //}
        io::stdout().flush().ok().expect("Could not flush stdout");
    }
}

pub mod generators {
    use super::common::Config;
    use integer_sqrt::IntegerSquareRoot;

    pub fn sieve_of_eratosthenes(config: &Config) -> Vec<u64> {
        let mut sieve = Vec::with_capacity(config.prime_range_end as usize);
        let mut primes = Vec::new();

        for _ in 0..config.prime_range_end {
            sieve.push(true);
        }

        for i in 2..config.prime_range_end.integer_sqrt() {
            if sieve[i as usize] {
                let square = i * i;
                let mut multiple = 0;
                let mut option = Some(square + (i*multiple));
                while let Some(j) = option {
                    if j >= config.prime_range_end {
                        option = None;
                    }
                    else {
                        sieve[j as usize] = false;
                        multiple += 1;
                        option = Some(square + (i*multiple));
                    }
                }
            }
        }

        for i in 2..config.prime_range_end {
            if sieve[i as usize] {
                primes.push(i);        
            }
        }
        
        primes
    }
}
