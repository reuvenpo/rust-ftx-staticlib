.PHONY: build
build:
	cd enclave && cargo build
	cd app && cargo build

.PHONY: clean
clean:
	cd app && cargo clean
	cd enclave && cargo clean
