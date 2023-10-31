test:
	cargo test -- --nocapture

format:
	cargo fmt

check:
	cargo check

clean:
	cargo clean

build:
	cargo build

run:
	cargo run --release

install:
	cargo build --release && cp target/release/timekeeper /usr/local/bin/timekeeper
