import sys
from collections import defaultdict

lines = []
for line in sys.stdin:
    lines.append(line)

n = len(lines)

seeds = list(map(int, lines[0].split(": ")[1].split()))

maps = []
i = 3
while i < n:
    m = []
    while i < n and not lines[i].isspace():
        m.append(list(map(int, lines[i].split())))
        i += 1
    maps.append(m)
    i += 2

ranges = []
for i in range(0, len(seeds), 2):
    ranges.append((seeds[i], seeds[i] + seeds[i+1] - 1))
print(ranges)

def merge(ranges):
    ranges = sorted(ranges, key=lambda x:x[0])
    res = []
    for r in ranges:
        if res and r[0] <= res[-1][1] + 1: # merging closed integer intervals
            res[-1][1] = max(res[-1][1], r[1])
        else:
            res.append([r[0], r[1]])
    return res

for m in maps:
    ranges = sorted(ranges, key=lambda x: x[0])
    new_ranges = []
    for range in ranges:
        l, r = range
        cur_ranges = []
        used_intervals = []
        for lm in m:
            dest, source, s = lm
            if l <= source <= r:
                cur_ranges.append((dest, dest + min(r - source, s)))
                used_intervals.append((source, min(r, source + s)))
            elif source <= l <= source + s:
                cur_ranges.append((dest + l - source, dest + min(r - source, s)))
                used_intervals.append((l, min(r, source + s)))
        used_intervals = merge(used_intervals)
        i = 0
        while i < len(used_intervals):
            if l < used_intervals[i][0]:
                cur_ranges.append((l, used_intervals[i][0]-1))
            l = max(l, used_intervals[i][1] + 1)
            i += 1
        if l < r:
            cur_ranges.append((l, r))
        print(range, "used", used_intervals, "cur",cur_ranges)
        cur_ranges = merge(cur_ranges)
        new_ranges.extend(cur_ranges)
    ranges = merge(new_ranges)

    print("ranges", ranges)
    print()
