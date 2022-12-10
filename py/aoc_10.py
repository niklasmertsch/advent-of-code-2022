x = [1]
for i, line in enumerate(map(str.strip, open("../inputs/10.txt"))):
    if line == "noop":
        x.append(x[-1])
    else:
        x.append(x[-1])
        x.append(x[-1] + int(line.split()[1]))

result = 0
for i in range(20, len(x), 40):
    result += x[i-1] * i
print(result)

for i, num in enumerate(x):
    char = "#" if abs(num - i%40) <= 1 else "."
    # print(i, num, char)
    print(char, end="")
    if i % 40 == 39:
        print()
