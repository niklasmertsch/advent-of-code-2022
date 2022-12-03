def get_score(item: str) -> int:
    if item.islower():
        return ord(item) - ord("a") + 1
    return ord(item) - ord("A") + 27


single_score = 0
badge_score = 0
group_set = set()
for i, line in enumerate(open("../inputs/03.txt")):
    line = line.strip()
    single_score += get_score(next(iter(set(line[:len(line) // 2]).intersection(line[len(line) // 2:]))))

    if i % 3 == 0:
        if i > 0:
            badge_score += get_score(next(iter(group_set)))
        group_set = set(line)
    group_set &= set(line)
badge_score += get_score(next(iter(group_set)))


print(f"single score: {single_score}")
print(f"badge score: {badge_score}")
