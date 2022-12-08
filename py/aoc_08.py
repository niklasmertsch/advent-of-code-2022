grid = []
for line in open("../inputs/08.txt"):
    grid.append(list(map(int, line.strip())))

visible_trees: set[tuple[int, int]] = set()
best_score = 0
for i, row in enumerate(grid):
    for j, height in enumerate(row):
        left = row[:j][::-1]
        right = row[j+1:]
        top = [grid[row_num][j] for row_num in range(i)][::-1]
        bottom = [grid[row_num][j] for row_num in range(i+1, len(grid))]

        # part 1
        for direction in (left, right, top, bottom):
            if not any(tree >= height for tree in direction):
                visible_trees.add((i, j))

        # part 2
        if i == 0 or j == 0 or i == len(grid) - 1 or j == len(grid) - 1:
            continue  # edge

        score = 1
        for direction in (left, right, top, bottom):
            visible_trees_in_direction = 0
            for tree in direction:
                visible_trees_in_direction += 1
                if tree >= height:
                    break
            score *= visible_trees_in_direction

        if score > best_score:
            best_score = score

print(len(visible_trees))
print(best_score)
