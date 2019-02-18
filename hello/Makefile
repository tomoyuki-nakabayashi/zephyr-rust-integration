PREFIX ?= .
LIB_DIR ?= $(PREFIX)/lib

TARGET_PATH := target/thumbv7m-none-eabi/debug
RUST_TARGET := librust.a
RUST_FILES := $(shell find src/ -type f -name "*.rs") Cargo.toml Cargo.lock
C_BINDINGS := bindings.h

all: $(LIB_DIR)/$(RUST_TARGET) $(LIB_DIR)/$(C_BINDINGS)

$(LIB_DIR)/$(RUST_TARGET): $(RUST_FILES)
	mkdir -p $(LIB_DIR)
	cargo build
	cp $(TARGET_PATH)/libapp.a $(LIB_DIR)/$(RUST_TARGET)
	cargo objcopy -- --weaken-symbol=memmove --weaken-symbol=memcpy --weaken-symbol=memset --weaken-symbol=memcmp $(LIB_DIR)/$(RUST_TARGET)

$(LIB_DIR)/$(C_BINDINGS): $(RUST_FILES)
	cbindgen -l c -o $(LIB_DIR)/$(C_BINDINGS)

clean:
	rm -rf $(LIB_DIR)
	cargo clean
