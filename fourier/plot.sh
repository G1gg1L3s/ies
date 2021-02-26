#!/bin/bash

# Plot the data in the FILE with labels X and Y on axes into the OUT.png file
FILE=$1
MODE=$2
X=$3
Y=$4
OUT=$5
OTHER=$6

if test "$#" -lt 4; then
    echo "Usage: $0 <file> <mode> <x-label> <y-label> <output.png> [other]"
    exit 1
fi

gnuplot -p -e "                                                            \
    set autoscale;                                                         \
    set grid ytics lt 0 lw 1 lc rgb '#bbbbbb';                             \
    set grid xtics lt 0 lw 1 lc rgb '#bbbbbb';                             \
    set xlabel '${X}';                                                     \
    set ylabel '${Y}';                                                     \
    set term png;                                                          \
    set output '${OUT}';                                                   \
    set terminal png size 1920,1080;                                       \
    ${OTHER}                                                               \
    plot '${FILE}' with ${MODE};                                           \
"
