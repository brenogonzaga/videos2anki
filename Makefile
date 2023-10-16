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
dev:
	maturin develop & python init.py
release:
	cargo clean & cargo build --release & del /Q *.pyd & copy target\release\*.dll .\ & rename *.dll *.pyd & pyinstaller --noconfirm init.py
clean:
	cargo clean
