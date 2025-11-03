.DEFAULT_GOAL = target

target: src/lib.rs Cargo.lock .cargo/config.toml
	cargo build -r && \
  cp ./target/release/liblz4frame.$(SHLIB_SUFFIX_NAME) ./modules/lz4frame.$(SHLIB_DL_SUFFIX_NAME)

test: cargo_test

cargo_test:
	cargo test

clean: cargo_clean

cargo_clean:
	cargo clean
