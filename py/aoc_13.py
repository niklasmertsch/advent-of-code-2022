from ast import literal_eval
from enum import Enum


class ListWrapper:
    def __init__(self, raw_list):
        self.raw_list = raw_list

    def __eq__(self, other):
        return self.raw_list == other.raw_list

    def __lt__(self, other):
        return are_ordered(self.raw_list, other.raw_list)

    def __repr__(self):
        return repr(self.raw_list)


class Comp(Enum):
    LT = 0
    EQ = 1
    GT = 2


pairs = []
for raw_pair in open("../inputs/13.txt").read().replace("\r", "").split("\n\n"):
    a, b = map(literal_eval, raw_pair.strip().split("\n"))
    pairs.append((a, b))


def are_ordered(first, second) -> bool | None:
    for left, right in zip(first, second):
        result = None
        if isinstance(left, int) and isinstance(right, int):
            if left < right:
                result = True
            if left > right:
                result = False
        elif isinstance(left, list) and isinstance(right, list):
            result = are_ordered(left, right)
        elif isinstance(left, int):
            result = are_ordered([left], right)
        else:
            result = are_ordered(left, [right])
        if result is not None:
            return result
    if len(first) < len(second):
        return True
    if len(first) > len(second):
        return False


result_1 = 0
for i, pair in enumerate(pairs, start=1):
    outcome = are_ordered(*pair)
    if outcome is True:
        result_1 += i
print(result_1)


lists = []
for a, b in pairs:
    lists.append(ListWrapper(a))
    lists.append(ListWrapper(b))
sep1 = ListWrapper([[2]])
sep2 = ListWrapper([[6]])
lists.append(sep1)
lists.append(sep2)
lists.sort()
print(*lists, sep="\n")

print((lists.index(sep1) + 1) * (lists.index(sep2) + 1))
