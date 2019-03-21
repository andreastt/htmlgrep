-include Makefile.local

prefix := /usr/local
TARGET ?= debug

BINS := target/$(TARGET)/css
SRCS := $(wildcard src/**/*.rs)

.PHONY: all
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

test: $(SRCS)
	cargo test $(CARGOFLAGS) -- --nocapture $(BINFLAGS)

doc: $(SRCS) | man
	cargo doc
	@echo target/doc

fmt: $(SRCS)
	cargo fmt

clean: clean-man
	rm -f css
	rm -rf target/

install: $(BINS) install-man
	install -d -m 0755 $(prefix)
	install -d -m 0755 $(prefix)/bin
	install -m 0755 $(TARGET)/css $(prefix)/bin

uninstall: uninstall-man
	rm -f $(prefix)/bin/css

.PHONY: all run test doc clean install uninstall

$(BINS): $(SRCS)
	cargo build $(CARGOFLAGS)
	@echo $@
