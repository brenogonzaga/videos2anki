format:
	cargo fmt --quiet
lint:
	cargo clippy --quiet
test:
	cargo test --quiet
dev:
	maturin develop & python init.py
run:
	cargo run 
build:
	cargo build --release
clean:
	cargo clean
