# Maybe it isn't the best makefile that you have ever seen,
# but it does its job well. So, who cares? ¯\_(ツ)_/¯

TARGETS = lab1-gen lab1-on lab2-autocorr lab2-corr lab2-on
DATA = gen.dat values.dat on.dat autocorr.dat corr.dat corr-on.dat

all: out/signal.png out/on.png out/autocorr.png out/corr.png out/corr-on.png

# Build main binaries
$(TARGETS): src/*
	cargo build --release --bin $@
	mkdir -p out

# Plot signal
out/signal.png: lab1-gen
	target/release/lab1-gen > out/gen.dat 2> out/values.dat
	./plot.sh out/gen.dat 't' 'x(t)' out/signal.png

# Plot O(N)
out/on.png: lab1-on
	target/release/lab1-on > out/on.dat
	./plot.sh out/on.dat 'n' 'O(n)' out/on.png

# Plot auto-correlation
out/autocorr.png: lab2-autocorr
	target/release/lab2-autocorr > out/autocorr.dat
	./plot.sh out/autocorr.dat 'tau' 'correlation' $@

# Plot correlation of two random signals
out/corr.png: lab2-corr
	target/release/lab2-corr > out/corr.dat
	./plot.sh out/corr.dat 'tau' 'correlation' $@

# Plot O(n) of correlation calculating
out/corr-on.png: lab2-on
	target/release/lab2-on > out/corr-on.dat
	./plot.sh out/corr-on.dat 'n' 'O(n)' $@ 'set logscale x 10;'

clean:
	rm -rf build target

.PHONY: clean

