integrate: fix clippy test build

fix:
	cargo fix --allow-dirty --allow-staged
clippy:
	cargo clippy --fix --allow-dirty --allow-staged
test:
	cargo test
build:
	cargo build

