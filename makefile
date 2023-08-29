.DEFAULT_GOAL := build
RELEASE ?= false

.PHONY: clean
clean:
	@cargo clean

.PHONY: build
build:
	@echo "Building inject…"

ifeq ($(RELEASE), true)
	cargo build --release --target aarch64-linux-android
else
	cargo build --target aarch64-linux-android
endif

	@echo "Built inject(bin) successfully"
