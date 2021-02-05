# Lab1

This crate contains source code for the first lab of IES.

To build crate and generate graphs:
```bash
$ make all
```

To clean everything:
```bash
$ make clean
```

All graphs and data should be in the folder `out`:
- `out/gen.dat` - values for signal
- `out/on.dat` - dependence of the generation time on the size.
- `out/values.dat` - average value and dispersion for signal
- `out/gen.png` - graph for `gen.dat`.
- `out/on.png` - graph for `on.dat`.
- `out/auto-corr.dat` - auto correlation data.
- `out/auto-corr.png` - auto correlation graph.
- `out/corr.dat` - data of correlation between two signals.
- `out/corr.png` - graph of correlation between two signals.
- `out/corr-on.dat` - dependence of the correlation calculation time on the size.
- `out/corr-on.png` - graph for `corr-on.png`
