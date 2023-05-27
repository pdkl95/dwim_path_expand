
PREFIX ?= /usr/local
BINDIR ?= $(PREFIX)/bin

INSTALL = install -c
INSTALL_STRIP = $(INSTALL) -s
RM_F = rm -f

BUILD_TARGET = x86_64-unknown-linux-gnu
BUILD_OPTS = -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort

export RUSTFLAGS = -C strip=debuginfo -C lto -C link-arg=-s

TARGET_BUILD_DIR = target/x86_64-unknown-linux-gnu/release
DWIM_PATH_EXPAND_BIN = $(TARGET_BUILD_DIR)/dwim_path_expand
DWIM_PATH_EXPAND_SRC = Cargo.toml $(wildcard src/*.rs)

TARGETS = $(DWIM_PATH_EXPAND_BIN)
TARGET_BASENAMES = $(notdir $(TARGETS))
INSTALLED_TARGETS = $(addprefix $(BINDIR)/,$(TARGET_BASENAMES))


all: build

build: $(TARGETS)

$(DWIM_PATH_EXPAND_BIN): $(DWIM_PATH_EXPAND_SRC)
	cargo build $(BUILD_OPTS) --target $(BUILD_TARGET) --release

install: build
	$(INSTALL_STRIP) $(TARGETS) $(BINDIR)

uninstall:
	$(RM_F) $(INSTALLED_TARGETS)

clean:
	cargo clean


.PHONY: all build install clean
