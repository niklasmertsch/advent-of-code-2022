normal_score = 0
elvish_score = 0
for line in open("../inputs/02.txt"):
    match line.strip():
        case "A X":
            normal_score += 1 + 3
            elvish_score += 0 + 3
        case "A Y":
            normal_score += 2 + 6
            elvish_score += 3 + 1
        case "A Z":
            normal_score += 3 + 0
            elvish_score += 6 + 2
        case "B X":
            normal_score += 1 + 0
            elvish_score += 0 + 1
        case "B Y":
            normal_score += 2 + 3
            elvish_score += 3 + 2
        case "B Z":
            normal_score += 3 + 6
            elvish_score += 6 + 3
        case "C X":
            normal_score += 1 + 6
            elvish_score += 0 + 2
        case "C Y":
            normal_score += 2 + 0
            elvish_score += 3 + 3
        case "C Z":
            normal_score += 3 + 3
            elvish_score += 6 + 1

print(f"normal score: {normal_score}")
print(f"elvish score: {elvish_score}")
