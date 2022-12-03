scores = []
current_score = 0
for line in open("../inputs/01.txt"):
    if line.isspace():
        scores.append(current_score)
        current_score = 0
    else:
        current_score += int(line)

scores.sort()
print(f"max: {scores[-1]}")
print(f"max 3: {sum(scores[-3:])}")
