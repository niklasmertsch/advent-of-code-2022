from __future__ import annotations


def get_factor(num: int) -> int:
    if num == 0:
        return 0
    if num > 0:
        return 1
    else:
        return -1


class Node:
    def __init__(self) -> None:
        self.x = 0
        self.y = 0

    def __iter__(self):
        yield self.x
        yield self.y


class Head(Node):
    def move(self, direction_char: str) -> None:
        x_direction, y_direction = dict(U=(0, 1), D=(0, -1), R=(1, 0), L=(-1, 0))[direction_char]
        self.x += x_direction
        self.y += y_direction


class Tail(Node):
    def __init__(self, head: Node) -> None:
        super().__init__()
        self.head = head

    def follow(self):
        x_diff = self.head.x - self.x
        y_diff = self.head.y - self.y

        if abs(x_diff) > 1 or abs(y_diff) > 1:
            self.x += 1 * get_factor(x_diff)
            self.y += 1 * get_factor(y_diff)


head = Head()
tail_nodes: list[Tail] = [Tail(head)]
for _ in range(8):
    tail_nodes.append(Tail(tail_nodes[-1]))

node_1 = tail_nodes[0]
node_9 = tail_nodes[-1]

positions_1: set[tuple[int, int]] = {tuple(node_1)}
positions_9: set[tuple[int, int]] = {tuple(node_9)}
for line in map(str.strip, open("../inputs/09.txt")):
    direction_char, raw_amount = line.split()
    amount = int(raw_amount)

    print(line)
    for _ in range(amount):
        head.move(direction_char)
        print("  head", tuple(head))
        for i, node in enumerate(tail_nodes, start=1):
            node.follow()
            print("    tail", i, tuple(node))

        positions_1.add(tuple(node_1))
        positions_9.add(tuple(node_9))

print(len(positions_1))
print(len(positions_9))
