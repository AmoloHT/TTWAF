CC = cargo

all:
	$(CC) build --release --target=x86_64-unknown-linux-musl
	cp target/x86_64-unknown-linux-musl/release/ttwaf .

clean:
	rm -r target/release/*
