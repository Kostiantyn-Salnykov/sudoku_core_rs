fmt:
	cargo fmt

test:
	cargo test -- --nocapture

run:
	cargo run -r -- --lib

pre: fmt test

tree:
	cargo tree
