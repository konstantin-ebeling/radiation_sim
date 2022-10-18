del /s /f /q .\out\*.*
for /f %%f in ('dir /ad /b .\out\') do rd /s /q .\out\%%f

cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --out-dir out --target web --no-typescript --weak-refs --reference-types ./target/wasm32-unknown-unknown/release/radiation_sim.wasm
copy "static\index.html" "out\index.html"
robocopy "assets" "out/assets" /E
