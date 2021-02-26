BINS = discrete
GRAPHS = out/signal.png out/fourier.png

all: $(GRAPHS)

$(BINS): src/*
	cargo build --release --bin $@
	mkdir -p out

out/signal.dat out/fourier.dat: discrete
	cargo run --bin discrete --release out/signal.dat out/fourier.dat

out/signal.png: out/signal.dat
	./plot.sh out/signal.dat "linespoints pointtype 7 pointsize 0.5" "t" "x(t)" out/signal.png

out/fourier.png: out/fourier.dat
	./plot.sh out/fourier.dat "boxes" "p" "F(p)" out/fourier.png

clean:
	rm -rf out target

.PHONY: all clean
