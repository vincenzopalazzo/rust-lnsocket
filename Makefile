CC=cargo
FMT=fmt
NAME=go-lnmetrics
BASE_DIR=/script
OS=linux
ARCH=386

OUT_DIR=src/lnsocket/

default: gen fmt

fmt:
	$(CC) fmt --all

dep:
	cd core/include/lnsocket; make all; cp lnsocket.a liblnsocket.a

# The run command need some work with the option passing.
run:
	@echo "Nothings yet"

gen:
	OUT_DIR=$(OUT_DIR) $(CC) build

build:
	$(CC) build --release

check:
	$(CC) test --all -- --show-output --skip lnsocket::bindings

clean:
	$(CC) clean
