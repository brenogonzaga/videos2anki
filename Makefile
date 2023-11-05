format:
	cargo fmt --quiet
lint:
	cargo clippy --quiet
test:
	cargo test --quiet
run:
	cargo run 
del:
	del /Q src\*.pyd src\*.dll
dev: del
	cargo build & copy target\release\*.dll src & rename src\*.dll *.pyd & python src/init.py
build: del
	cargo clean & cargo build --release & copy target\release\*.dll src & rename src\*.dll *.pyd & pyinstaller --noconfirm --nowindowed --noconsole src/init.py
clean:
	cargo clean
