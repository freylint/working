
.PHONY: all

all: update_reports format
	@echo NOTE: This is a monorepository. Use "make all_pkgs" to build all packages.

all_pkgs: build

f: format
fmt: format
format:
	python scripts/format.py

d: dev
dev: test
	set RUSTFLAGS=-Zshare-generics
	cargo r --features bevy/dynamic_linking --package momori

t: test
test:
	cargo test

d: doc
doc:
	cargo doc

update_reports:
	python scripts/report.py

prebuild:
	python scripts/prebuild.py

build: build-win build-nix build-darwin

build-nix:
	cross b -r --target x86_64-unknown-linux-gnu

build-win:
	cross b -r --target x86_64-pc-windows-gnu

build-darwin:
	echo not implemented

toolchain:
	python scripts/toolchain.py

