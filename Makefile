format:
	cargo fmt --quiet
lint:
	cargo clippy --quiet
test:
	cargo test --quiet
dev:
	maturin develop & python src/init.py
run:
	cargo run 
dev:
	maturin develop & python src/init.py
build:
	cargo clean & cargo build --release & del /Q *.pyd & copy target\release\*.dll .\ & rename *.dll *.pyd & pyinstaller --noconfirm --nowindowed --noconsole src/init.py
clean:
	cargo clean
