RUSTFLAGS="-C opt-level=3 -C target-cpu=native -C codegen-units=1" cargo build --release && sudo cp target/release/libwyhash_3.a /usr/local/lib/
