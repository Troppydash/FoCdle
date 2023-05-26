# FoCdle Solver
For the COMP10001 FoC course, project 2

## Usage
To build the program, run
```
cargo build --release
```

To run the program, run
```
focdle.exe [expression]*
```

For instance
```
$ focdle.exe 1+1+1=3 10*12+1=121
secret 1+1+1=3:
1825346
1%3%9=1
1+1+1=3

secret 10*12+1=121:
41%30%==268
52+19-70*10
10*12+2=122
10*12+1=121
```


## Performance
Tested on 1000 random secrets, each running 100 times.
```
Difficulty 7
max: 7
min: 2
2 = 6214
3 = 65204
4 = 26145
5 = 2284
6 = 146
7 = 7
average: 3.25
took 12.4815245s (for 1000 secrets, 100x each)

Difficulty 8
max: 7
min: 3
3 = 44741
4 = 49124
5 = 5794
6 = 329
7 = 12
average: 3.62
took 14.3309417s (for 1000 secrets, 100x each)

Difficulty 9
max: 7
min: 3
3 = 29234
4 = 59269
5 = 10888
6 = 597
7 = 12
average: 3.83
took 35.6680584s (for 1000 secrets, 100x each)

Difficulty 10
max: 7
min: 3
3 = 36009
4 = 49849
5 = 13637
6 = 478
7 = 27
average: 3.79
took 60.0546776s (for 1000 secrets, 100x each)

Difficulty 11
max: 8
min: 3
3 = 32921
4 = 52235
5 = 14233
6 = 588
7 = 22
8 = 1
average: 3.83
took 72.3976335s (for 1000 secrets, 100x each)

Difficulty 12
max: 7
min: 3
3 = 51521
4 = 42575
5 = 5630
6 = 273
7 = 1
average: 3.55
took 150.258627s (for 1000 secrets, 100x each)

Difficulty 13
max: 9
min: 3
3 = 69107
4 = 27656
5 = 2758
6 = 387
7 = 77
8 = 14
9 = 1
average: 3.35
took 137.2991186s (for 1000 secrets, 100x each)

Difficulty 14
max: 5
min: 3
3 = 86752
4 = 13155
5 = 93
average: 3.13
took 65.1698439s (for 1000 secrets, 100x each)

Difficulty 15
max: 5
min: 3
3 = 85653
4 = 14216
5 = 131
average: 3.14
took 111.8760022s (for 1000 secrets, 100x each)
```