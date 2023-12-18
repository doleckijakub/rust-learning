SRC_DIRS := $(wildcard src/*)
BINARIES := $(patsubst src/%, bin/%, $(SRC_DIRS))

.PHONY: all clean

all: $(BINARIES)

bin:
	mkdir -p bin

bin/%: src/%/main.rs | bin
	rustc -o $@ $<

clean:
	rm -rf bin
