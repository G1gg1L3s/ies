TARGETS = lab1-gen lab1-on

all: out/signal.png out/on.png

# Build main binaries
$(TARGETS): src/*
	cargo build --release --bin $@

# Plot signal
out/signal.png: lab1-gen
	mkdir -p out
	target/release/lab1-gen > out/gen.dat 2> out/values.dat
	./plot.sh out/gen.dat 't' 'x(t)' out/signal.png

# Plot O(N)
out/on.png: lab1-on
	mkdir -p out
	target/release/lab1-on > out/on.dat
	./plot.sh out/on.dat 'n' 'O(n)' out/on.png

clean:
	rm -rf build target

.PHONY: clean

