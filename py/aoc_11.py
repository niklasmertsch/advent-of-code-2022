import dataclasses
import operator
from collections import defaultdict
from functools import reduce
from typing import Callable


@dataclasses.dataclass
class Monkey:
    starting_items: list[int]

    modify_operator: Callable[[int, int], int]
    raw_modify_operand: str

    pass_divisor: int
    pass_true_target: int
    pass_false_target: int

    def get_weight_update(self, item: int) -> int:
        if self.raw_modify_operand == "old":
            return self.modify_operator(item, item)
        return self.modify_operator(item, int(self.raw_modify_operand))

    def get_pass_target(self, item: int) -> int:
        if item % self.pass_divisor == 0:
            return self.pass_true_target
        else:
            return self.pass_false_target


monkeys = []
lines = list(map(str.strip, open("../inputs/11_min.txt")))
lines_per_monkey = 7
num_monkeys = len(lines) // lines_per_monkey + 1
for i in range(num_monkeys):
    monkey_lines = lines[lines_per_monkey * i : lines_per_monkey * (i+1)]

    starting_items = list(map(int, monkey_lines[1].split(":")[1].split(",")))
    modify_operator = {"*": operator.mul, "+": operator.add}[monkey_lines[2].split()[-2]]
    raw_modify_operand = monkey_lines[2].split()[-1]

    test_divisor = int(monkey_lines[3].split()[-1])
    test_true_target = int(monkey_lines[4].split()[-1])
    test_false_target = int(monkey_lines[5].split()[-1])

    monkeys.append(
        Monkey(
            starting_items,
            modify_operator,
            raw_modify_operand,
            test_divisor,
            test_true_target,
            test_false_target,
        )
    )

item_counts: dict[int, int] = defaultdict(lambda: 0)
common_divisor = reduce(operator.mul, [m.pass_divisor for m in monkeys])
for round_ in range(10_000):
    for monkey_index, monkey in enumerate(monkeys):
        for item in monkey.starting_items:
            # part 1
            # new_weight = monkey.get_weight_update(item) // 3

            # part 2
            new_weight = monkey.get_weight_update(item) % common_divisor

            target_monkey = monkey.get_pass_target(new_weight)
            monkeys[target_monkey].starting_items.append(new_weight)
            item_counts[monkey_index] += 1
        monkey.starting_items.clear()

counts = sorted(item_counts.values())
print(operator.mul(*counts[-2:]))
