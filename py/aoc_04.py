import re

full_overlaps = 0
partial_overlaps = 0
for line in open("../inputs/04.txt"):
    a_start, a_end, b_start, b_end = map(int, re.split("[-,]", line.strip()))
    if a_start >= b_start and a_end <= b_end or b_start >= a_start and b_end <= a_end:
        full_overlaps += 1
    if a_start <= b_start <= a_end or b_start <= a_start <= b_end:
        partial_overlaps += 1

print(full_overlaps)
print(partial_overlaps)
