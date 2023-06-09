# Rust Raycaster

Project to create a 2D raycasted game in Rust

## Installation & Build
### Linux Specific
1. Install Macroquad Dependencies [5]
    ```
    sudo apt install pkg-config libx11-dev libxi-dev libgl1-mesa-dev libasound2-dev
    ```

### Cross-Compilation
1. Compile to Windows [5]
    ```
    rustup target add x86_64-pc-windows-gnu
    sudo apt install mingw-w64

    cargo build_win
    cargo run_win

    .\target\x86_64-pc-windows-gnu\debug\rust-raycaster.exe
    .\target\x86_64-pc-windows-gnu\release\rust-raycaster.exe
    ```
2. Compile to Linux [5]
    ```
    rustup target add x86_64-unknown-linux-gnu

    cargo build_linux
    cargo run_linux

    ./target/x86_64-unknown-linux-gnu/debug/rust-raycaster.exe
    ./target/x86_64-unknown-linux-gnu/release/rust-raycaster.exe
    ```

3. Compile to WASM [5]
    ```
    cargo install basic-http-server
    rustup target add wasm32-unknown-unknown

    cargo build_wasm
    basic-http-server .
    ```

### Universal
1. Build and run Debug
    ```
    cargo build
    cargo run
    ./target/debug/rust-raycaster
    ```
2. Build and run Release
    ```
    cargo build --release
    cargo run --release
    ./target/release/rust-raycaster
    ```

## Project Structure
WIP

# References
[1] https://grantshandy.github.io/posts/raycasting/<br>
[2] https://wasm4.org/<br><br>
[3] https://macroquad.rs/docs/<br>
[4] https://github.com/not-fl3/macroquad<br>
[5] https://crates.io/crates/macroquad<br>

## Further Reading
[6] https://github.com/parasyte/pixels 