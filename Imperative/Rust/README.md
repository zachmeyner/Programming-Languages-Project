# Rust
Rust is a multiparadigm langauge, but it's mostly imperative.

Rust is meant as an alternative to C with similar execution times. What makes Rust stand out is the memory safety enforced by the compiler.

## Installation (If not already installed)
To install simply type

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
You will need to restart your terminal for the path variables to work. What I am doing requires the nightly version of the rust compiler. This can be gotten with
```bash
rustup default nightly
```

I have made an http server using Rusts features. To run the server simply go into the server directory with 
```bash
cd server
```
and run the command
```bash
cargo run
```
This will compile and run the code, and open an http server on `127.0.0.1:8000`