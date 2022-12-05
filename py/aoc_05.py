from collections import defaultdict

stacks_9000: dict[int, list[str]] = defaultdict(list)
with open("../inputs/05.txt") as fp:
    # read stacks
    for line in fp:
        if line.strip().startswith("1"):
            break
        for i in range(len(line) // 4):
            raw_block = line[4 * i : 4 * (i+1)]
            if raw_block.isspace():
                continue
            stacks_9000[i + 1].append(raw_block[1])

    for index, stack in stacks_9000.items():
        stacks_9000[index].reverse()
    stacks_9001: dict[int, list[str]] = {index: stack.copy() for (index, stack) in stacks_9000.items()}

    fp.readline()

    # read moves
    for line in fp:
        amount, origin, target = map(int, line.split()[1::2])

        for _ in range(amount):
            stacks_9000[target].append(stacks_9000[origin].pop())
        stacks_9001[target].extend(stacks_9001[origin][-amount:])
        stacks_9001[origin] = stacks_9001[origin][:-amount]

print("".join(stacks_9000[i][-1] for i in sorted(stacks_9000.keys())))
print("".join(stacks_9001[i][-1] for i in sorted(stacks_9001.keys())))
