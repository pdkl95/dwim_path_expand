
PREFIX ?= /usr/local
BINDIR ?= $(PREFIX)/bin

INSTALL = install -c
INSTALL_STRIP = $(INSTALL) -s
RM_F = rm -f

RUSTFLAGS="-Cembed-bitcode=yes"

TARGET_BUILD_DIR = target/x86_64-unknown-linux-gnu/release
DWIM_PATH_EXPAND_BIN = $(TARGET_BUILD_DIR)/dwim_path_expand
DWIM_PATH_EXPAND_SRC = Cargo.toml $(wildcard src/*.rs)

TARGETS = $(DWIM_PATH_EXPAND_BIN)
TARGET_BASENAMES = $(notdir $(TARGETS))
INSTALLED_TARGETS = $(addprefix $(BINDIR)/,$(TARGET_BASENAMES))


all: build

build: $(TARGETS)

$(DWIM_PATH_EXPAND_BIN): $(DWIM_PATH_EXPAND_SRC)
	echo $$RUSTFLAGS xargo build --target x86_64-unknown-linux-gnu --release

install: build
	$(INSTALL_STRIP) $(TARGETS) $(BINDIR)

uninstall:
	$(RM_F) $(INSTALLED_TARGETS)

clean:
	xargo clean


.PHONY: all build install clean
