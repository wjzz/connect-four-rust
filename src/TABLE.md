# 6x6

Using an extremely big trans. table seems to be not so great. We probably keep way to many old nodes and maybe it's just slower to access memory that is distributed among many memory sticks (IDK).

## SIZE = 2_147_483_647 (PC)

collissions = 34,553,456
inserts = 673,735,069
uppers = 577,482,012
lowers = 96,253,057
gets = 866,211,941
get_misses = 628,020,685

time = 369s (6min 9s)

## SIZE = 100_663_319 (PC)

collissions = 437,998,984
inserts = 678,943,606
uppers = 211,765,607
lowers = 467,177,999
gets = 854,389,475
get_misses = 657,202,110

time = 340s (5min 40s)

## SIZE = 100_663_319

collissions = 438,010,731
inserts = 678,957,335
uppers = 211,770,535
lowers = 467,186,800
gets = 854,392,907
get_misses = 657,206,250

time = 330s (5min 30s)

## SIZE = 1_572_869

collissions = 877,859,218
inserts = 881,689,550
uppers = 8,400,806
lowers = 873,288,744
gets = 1,074,029,861
get_misses = 879,443,928

time = 354s (5min 54s)

# 5x6 make solve

100M vs 1M make no difference in terms of time

## HASHTABLE_SIZE = 100_663_319

collissions = 533,837
inserts = 25,253,158
uppers = 22,854,813
lowers = 2,398,345
gets = 31,109,975
get_misses = 24,585,778

## HASHTABLE_SIZE = 50_331_653

collissions = 944,320
inserts = 25,250,117
uppers = 21,285,032
lowers = 3,965,085
gets = 31,096,765
get_misses = 24,607,785

## 25M

collissions = 2,220,494
inserts = 25,253,086
uppers = 18,691,667
lowers = 6,561,419
gets = 31,079,187
get_misses = 24,653,286
