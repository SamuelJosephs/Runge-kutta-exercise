# # Set the output file type
 set terminal postscript eps enhanced color solid colortext 9
# # Set the output file name
 set output 'multiple_plots.eps'

#  plot '~/rust/integration_comparison/difference.dat' i 0 u 1:2 w lines title columnheader(1),\
#       '~/rust/integration_comparison/difference.dat' i 1 u 1:2 w lines title columnheader(1)

plot "difference" i 0 u 1:2 w lines title columnheader(1),\
      "difference" i 0 u 1:2 w lines title columnheader(1)

#'plot for [IDX=0:1] 'difference.dat' i IDX u 1:2 w lines title columnheader(1)