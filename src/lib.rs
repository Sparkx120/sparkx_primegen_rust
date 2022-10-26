#![doc = include_str!("../README.md")]

/// Common structs and constants for sparkx_primegen
pub mod common {
    use std::fmt;

    /// Config for sparkx_primegen
    ///
    /// # Examples
    /// To construct a new config you can run
    /// ```
    /// let config = sparkx_primegen::Config::build(1000);
    /// 
    /// assert_eq!(1000, config.prime_range_end)
    /// ```
    pub struct Config {
        pub prime_range_start: u64,
        pub prime_range_end: u64,
        pub sieve_segment_size: u64,
        pub progress: bool,
    }

    impl Config {
        pub fn build(prime_range_end: u64) -> Config {
            Config {
                prime_range_start: 2,
                prime_range_end,
                sieve_segment_size: 100,
                progress: true, // Only for segmented calculation
            }
        }
    }

    impl fmt::Display for Config {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "range_start: {}\nrange_end: {}", self.prime_range_start, self.prime_range_end)
        }
    }
}

/// Driver for sparkx_primegen binary application
pub mod driver {
    // Crate
    use std::{io, io::Write};

    // Local
    use super::common::Config;
    use super::generators;

    pub fn run(config: Config) {
        println!("Finding Prime Numbers between {} and {}", config.prime_range_start, config.prime_range_end);

        let primes = generators::sieve_of_eratosthenes(&config);
        //let primes = generators::sieve_of_eratosthenes_segmented(&config);

        if config.prime_range_end > 1000 {
            println!("{} primes found under {}", primes.len(), config.prime_range_end);
            io::stdout().flush().ok().expect("Could not flush stdout");
        } else {
            for prime in primes {
                print!("{}, ", prime);
            }
        }
    }
}

/// Prime Number Generators
///
/// This Module has the various implementations of prime number generators that sparkx_primegen
/// supports.
pub mod generators {
    // Crate
    use integer_sqrt::IntegerSquareRoot;
    //use std::collections::HashMap;

    // Local
    use super::common::Config;

    /// An Eratosthenes Sieve over some arbitrary boolean Vector
    ///
    /// # Params
    /// - sieve - A mutable boolean vector
    /// - prime - The prime number to sieve out
    /// - starting_multiple - The starting multiplication of the prime number if calculating on a
    /// - segment, default should be 0
    /// - offset - The offset the sieve is at if its segmented, default should be 0
    #[inline]
    fn sieve_prime_factor(
        sieve: &mut Vec<bool>,
        prime: u64,
        starting_multiple: u64,
        offset: u64,
    ) {
        let square = prime * prime;
        let mut multiple = starting_multiple;
        let mut option = Some(square + (prime * multiple));
        while let Some(prime_factor) = option {
            if prime_factor - offset >= sieve.len() as u64 {
                option = None;
            }
            else {
                sieve[(prime_factor - offset) as usize] = false;
                multiple += 1;
                option = Some(square + (prime * multiple));
            }
        }

    }

    /// An Eratosthenes Sieve conversion to actual Prime Number Vector
    ///
    /// # Params
    /// - sieve - The Boolean Sieve to convert
    /// - primes - A mutable reference to a Vector to store primes in
    /// - offset - The starting offset of the sieve if its segmented, default should be 0
    #[inline]
    fn sieve_to_primes(
        sieve: &Vec<bool>,
        primes: &mut Vec<u64>,
        offset: u64
    ) {
        let start_offset = if offset == 0 { 2 } else { 0 };
        for sieve_index in start_offset..sieve.len() as u64 {
            if sieve[sieve_index as usize] {
                primes.push(sieve_index + offset);        
            }
        }
    }

    /// An Eratosthenes Sieve generator
    ///
    /// This is the original Eratosthenes algorithm implemented entirely in memory, must be able to
    /// allocate enough space for Config.primes_range_end or else program will be killed
    ///
    /// # Params
    /// - config - reference to a [`Config`]
    ///
    /// # Returns
    /// A Vetor with all found primes in ascending order
    pub fn sieve_of_eratosthenes(config: &Config) -> Vec<u64> {
        let mut sieve = Vec::with_capacity(config.prime_range_end as usize);
        let mut primes = Vec::new();

        for _ in 0..config.prime_range_end {
            sieve.push(true);
        }

        for i in 2..config.prime_range_end.integer_sqrt() {
            if sieve[i as usize] {
                sieve_prime_factor(&mut sieve, i, 0, 0);
            }
        }

        sieve_to_primes(&sieve, &mut primes, 0);

        primes
    }

    /// A segmented implementation of Eratosthenes Sieve
    ///
    /// This implementation is flawed as it checks for prime factors on each segment instead of
    /// sieving them. It is not recommended for use.
    ///
    /// # Params
    /// - config - reference to a [`Config`]
    ///
    /// # Returns
    /// A Vector with all found primes in ascending order
    pub fn sieve_of_eratosthenes_segmented(config: &Config) -> Vec<u64> {
        let mut primes = Vec::new();
        let total_sieve_segments = config.prime_range_end/config.sieve_segment_size;

        for sieve_offset in 0..total_sieve_segments {
            let sieve_shift = sieve_offset * config.sieve_segment_size;
            let mut sieve = Vec::with_capacity(config.sieve_segment_size as usize); 

            if config.progress {
                println!("Sieve Segment({}) {} of {} is being processed.",
                    config.sieve_segment_size, sieve_offset + 1,
                    total_sieve_segments);
            }

            // Initialize sieve segment
            for local_offset in 0..config.sieve_segment_size {
        
                // Sieve factors of found primes - This is fairly slow
                // What is we compute the low and high end of the segment within the prime
                // factorization and just sieve?
                let mut is_prime_factor = false;
                let total_offset = local_offset + sieve_shift;
                for p in &primes {
                    if *p > total_offset.integer_sqrt() {
                        break;
                    } else if total_offset % p == 0 {
                        sieve.push(false);
                        is_prime_factor = true;
                        break;
                    }
                }
                if !is_prime_factor {
                   sieve.push(true);
                }
            }
                            
            for local_offset in 0..config.sieve_segment_size {
                let total_offset = local_offset + sieve_shift;

                // 0, 1, 2 are not included in prime sieve calculation
                if total_offset < 2 {
                    continue
                } 
                
                //dbg!(sieve_offset, sieve_shift, local_offset, total_offset);
                //dbg!(&sieve);

                // If we found an offset in the sieve that is true it must be prime at
                // this point and need to be factored out
                if sieve[local_offset as usize] {
                    sieve_prime_factor(&mut sieve, total_offset, 0, sieve_shift);
                }
            }

            //dbg!(&sieve);

            sieve_to_primes(
                &sieve,
                &mut primes,
                sieve_shift
            );

            //dbg!(&primes);
            //println!("");
        }

        primes
    }
}
