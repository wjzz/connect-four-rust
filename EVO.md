# No ordering

5 x 5 | nodes = 2,946,643    | [elapsed: 1113] | ordering: 63%

# Hand crafted best

rows = 4, cols = 4 ==>   0 | 00:00:00.595 |               12,199 nodes
rows = 5, cols = 4 ==>   0 | 00:00:00.650 |               35,702 nodes
rows = 4, cols = 5 ==>   0 | 00:00:00.630 |               55,529 nodes
rows = 6, cols = 4 ==>   0 | 00:00:00.697 |              345,956 nodes
rows = 4, cols = 6 ==> -10 | 00:00:00.728 |              713,340 nodes
rows = 5, cols = 5 ==>   0 | 00:00:00.758 |              768,536 nodes
rows = 6, cols = 5 ==>   0 | 00:00:01.198 |            3,402,602 nodes
rows = 5, cols = 6 ==>   0 | 00:00:01.541 |            4,785,962 nodes
rows = 6, cols = 6 ==> -10 | 00:00:29.701 |          137,755,314 nodes
rows = 7, cols = 6 ==>  10 | 00:02:45.308 |          744,221,290 nodes
rows = 6, cols = 7 ==>  10 | 00:06:01.058 |        1,602,535,755 nodes

# Best weights for 6x5

6 x 5 | nodes =   2,087,241  | [elapsed: 1207] | ordering: 93%
5 x 6 | nodes =  16,655,557  | [elapsed: 5338] | ordering: 91%
6 x 6 | nodes = 412,207,403  | [elapsed: 124064] | ordering: 90%

The results for 5x6 and 6x6 are not so great.

# Best weights for 5x6

rows = 4, cols = 4 ==>   0 | 00:00:00.641 |                5,678 nodes
rows = 5, cols = 4 ==>   0 | 00:00:00.635 |               31,221 nodes
rows = 4, cols = 5 ==>   0 | 00:00:00.631 |              102,722 nodes
rows = 6, cols = 4 ==>   0 | 00:00:00.647 |              131,967 nodes
rows = 4, cols = 6 ==> -10 | 00:00:00.950 |            1,319,769 nodes
rows = 5, cols = 5 ==>   0 | 00:00:00.848 |              769,829 nodes
rows = 6, cols = 5 ==>   0 | 00:00:01.989 |            5,273,594 nodes
rows = 5, cols = 6 ==>   0 | 00:00:01.594 |            3,281,171 nodes
rows = 6, cols = 6 ==> -10 | 00:01:12.664 |          246,733,897 nodes
rows = 7, cols = 6 ==>  10 | 00:06:46.781 |        1,358,686,521 nodes
rows = 6, cols = 7 ==>  10 | 00:16:03.827 |        3,005,576,735 nodes