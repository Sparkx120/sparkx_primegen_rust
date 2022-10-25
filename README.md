# SparkX Primegen
This is a simple prime number generator library / driver written in rust that implements the [Sieve of Eratosthenes](https://en.wikipedia.org/wiki/Sieve_of_Eratosthenes)
A flawed segmented option is in the code but not fit for use at this time. The only functioning version of the sieve is `sparkx_primegen.generators.sieve_of_eratosthenes()`

To setup locally will require an installation of [Rust](https://www.rust-lang.org/)

## Configuration
Currently the settings are hardcoded into [lib.rs](./src/lib.rs) in the Config struct.

## Build and Run

```bash
cargo run --release
```
