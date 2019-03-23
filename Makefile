-include Makefile.local

prefix := /usr/local
TARGET ?= debug

BINS := target/$(TARGET)/css
SRCS := $(wildcard *.rs)

all: $(BINS) man

include doc/include.mk

cargoflags.debug :=
cargoflags.release := --release
CARGOFLAGS ?= ${cargoflags.${TARGET}}

ifeq (run,$(firstword $(MAKECMDGOALS)))
  BINFLAGS ?= $(wordlist 2,$(words $(MAKECMDGOALS)),$(MAKECMDGOALS))
  $(eval $(BINFLAGS):;@:)
endif

run: $(BINS)
	$^ $(BINFLAGS)

test: $(SRCS) | man-test lint
	RUST_BACKTRACE=1 cargo test --all -- --nocapture
	RUST_BACKTRACE=1 cargo test --examples

lint: $(SRCS)
	cargo clippy

fmt: $(SRCS)
	cargo fmt

doc: $(SRCS) | man
	cargo doc
	@echo target/doc

clean: man-clean
	rm -f css
	rm -rf target/

install: $(BINS) man-install
	install -d -m 0755 $(prefix)
	install -d -m 0755 $(prefix)/bin
	install -m 0755 $(TARGET)/css $(prefix)/bin

uninstall: man-uninstall
	rm -f $(prefix)/bin/css

.PHONY: all run test lint fmt doc clean install uninstall

$(BINS): $(SRCS)
	cargo build $(CARGOFLAGS)
	@echo $@
