build:
	cargo build --release

clean:
	cargo clean

install:
	cp target/release/blarf /usr/local/bin
