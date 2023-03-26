del /s /f /q .\out\*.*
for /f %%f in ('dir /ad /b .\out\') do rd /s /q .\out\%%f

IF "%~1" == "--debug" goto debug

cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --out-dir out --target web --no-typescript --weak-refs --reference-types ./target/wasm32-unknown-unknown/release/radiation_sim_bin.wasm
wasm-opt -O --enable-reference-types -o .\out\radiation_sim_bin_bg.wasm .\out\radiation_sim_bin_bg.wasm
goto finish

:debug
cargo build --target wasm32-unknown-unknown
wasm-bindgen --out-dir out --target web --no-typescript --weak-refs --reference-types ./target/wasm32-unknown-unknown/release/radiation_sim_bin.wasm

:finish
copy "static\index.html" "out\index.html"
robocopy "assets" "out/assets" /E
