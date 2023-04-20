
1. Build for web-asm:
wasm-pack build --debug --target web
or
wasm-pack build --release --target web

2. Run
python3 -m http.server

3. Open in browser
http://localhost:8000/

//https://stackoverflow.com/questions/68646684/cant-install-cargo-wasm-pack

clone vcpkg
open directory where you've cloned vcpkg
run ./bootstrap-vcpkg.bat
run ./vcpkg.exe install openssl-windows:x64-windows
run ./vcpkg.exe install openssl:x64-windows-static
run ./vcpkg.exe integrate install
run set VCPKGRS_DYNAMIC=1 (or simply set it as your environment variable)

wsl --install


apt-get install openssl-dev
apt-get install cargo
apt-get install rustup
rustup update
cargo install web-pack