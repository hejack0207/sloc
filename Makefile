VERSION = $(patsubst "%",%, $(word 3, $(shell grep version Cargo.toml)))
BUILD_TIME = $(shell date +"%Y/%m/%d %H:%M:%S")
GIT_REVISION = $(shell git log -1 --format="%h")
PROG_NAME=sloc

export BUILD_TIME
export GIT_REVISION

.PHONY: all test bench bench_sse clean release_lnx64 release_win64 release_osx64 localinstall

all: test bench

test:
	cargo test -- --nocapture

bench:
	cargo bench

bench_sse:
	cargo bench --features 'sse'

build_statistics:
	cargo build --release --features 'statistics'

clean:
	cargo clean

localinstall:
	cargo install --offline --path . --force

release_lnx32:
	cargo build --release --target=i686-unknown-linux-gnu
	zip -j ${PROG_NAME}-v${VERSION}-i686-lnx.zip target/i686-unknown-linux-gnu/release/${PROG_NAME}*

release_lnx64:
	cargo build --release --target=x86_64-unknown-linux-gnu
	zip -j ${PROG_NAME}-v${VERSION}-x86_64-lnx.zip target/x86_64-unknown-linux-gnu/release/${PROG_NAME}*

release_win64:
	cargo build --release --target=x86_64-pc-windows-gnu
	zip -j ${PROG_NAME}-v${VERSION}-x86_64-win.zip target/x86_64-pc-windows-gnu/release/${PROG_NAME}*

release_osx32:
	cargo build --release --target=i686-apple-darwin
	zip -j ${PROG_NAME}-v${VERSION}-i686-osx.zip target/i686-apple-darwin/release/${PROG_NAME}*

release_osx64:
	cargo build --release --target=x86_64-apple-darwin
	zip -j ${PROG_NAME}-v${VERSION}-x86_64-osx.zip target/x86_64-apple-darwin/release/${PROG_NAME}*

publish:
	cargo publish
