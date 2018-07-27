set terminal png
set output 'measure.png'
set datafile separator ";"
set view map
set pm3d
set dgrid3d
splot "measure.csv" matrix 