from collections import defaultdict

files = defaultdict(list)
current_dir = None
for line in map(str.strip, open("../inputs/07.txt")):
    if line.startswith("$ cd"):
        new_dir = line.split()[-1]
        if new_dir == "/":
            current_dir = new_dir
        elif new_dir == "..":
            current_dir = current_dir.rsplit("/", maxsplit=1)[0]
        else:
            current_dir = "/".join((current_dir.rstrip("/"), new_dir))
    elif line.startswith("dir"):
        continue
    elif line.startswith("$ ls"):
        continue
    else:
        raw_size, file = line.split()
        files[current_dir].append((int(raw_size), file))

dir_sums = defaultdict(lambda: 0)
for d, files in files.items():
    for size, file in files:
        # dir_sums["/"] += size
        for i, elem in enumerate(d.split("/")):
            dir_sums["/" + "/".join(d.split("/")[:i+1])] += size
dir_sums.pop("//")

print(sum(s for s in dir_sums.values() if s <= 100_000))

space_to_free = dir_sums["/"] - 40_000_000
print(min(s for s in dir_sums.values() if s >= space_to_free))
