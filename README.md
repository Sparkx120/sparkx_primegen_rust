# SparkX Primegen
This is a simple prime number generator library / driver written in rust that implements the [Sieve of Eratosthenes](https://en.wikipedia.org/wiki/Sieve_of_Eratosthenes)
A flawed segmented option is in the code but not fit for use at this time. The only functioning version of the sieve is `sparkx_primegen::generators::sieve_of_eratosthenes(config: &Config)`

To setup locally will require an installation of [Rust](https://www.rust-lang.org/)

## Configuration
Currently the settings are hardcoded into [lib.rs](./src/lib.rs) in the Config struct.

## Building and Runing

The following will build and run a release build and set the prime number range's upper limit to 1 billion.
```bash
cargo run --release -- -e 1000000000
```

The following will build and run in unoptimized mode and use the default prime number range
```bash
cargo run
```
