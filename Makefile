CC = gcc
ASFLAGS = -g -Wa,--fatal-warnings
LDFLAGS = -no-pie -Wl,-z,noexecstack

.PHONY: all run test clean
all: build/taxi_app

build/taxi_app: $(wildcard asm/*.S) asm/common.inc
	mkdir -p build
	$(CC) $(ASFLAGS) $(LDFLAGS) -o $@ asm/main.S -lm

run: all
	./build/taxi_app

test: all
	python3 tests/test_app.py ./build/taxi_app

clean:
	rm -f build/taxi_app
