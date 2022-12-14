# read rock paths
paths: list[list[tuple[int, int]]] = []
min_x, max_x, min_y, max_y = 1000, 0, 0, 0
for line in open("../inputs/14.txt"):
    paths.append([])
    for raw_point in line.split(" -> "):
        x, y = map(int, raw_point.split(","))
        paths[-1].append((x, y))
        min_x = min(x, min_x)
        max_x = max(x, max_x)
        max_y = max(y, max_y)

max_y += 2
min_x -= 1000
max_x += 1000

# normalize rock paths
for i in range(len(paths)):
    for j in range(len(paths[i])):
        x, y = paths[i][j]
        paths[i][j] = x - min_x, y
max_x -= min_x
snow_start_x = 500 - min_x
min_x = 0

# prepare grid
SNOW = "o"
AIR = "."
ROCK = "#"


def reset_grid():
    # create grid
    grid = []
    for i in range(max_y+1):
        grid.append([])
        for j in range(max_x+1):
            grid[-1].append(AIR)

    # draw rocks
    for path in paths:
        for (start_x, start_y), (end_x, end_y) in zip(path[:-1], path[1:]):
            if start_x == end_x:
                start_y, end_y = min(start_y, end_y), max(start_y, end_y)
                for y in range(start_y, end_y+1):
                    grid[y][start_x] = ROCK
            else:
                start_x, end_x = min(start_x, end_x), max(start_x, end_x)
                for x in range(start_x, end_x+1):
                    grid[start_y][x] = ROCK

    grid[0][snow_start_x] = "+"
    return grid


def print_grid():
    print()
    for i, line in enumerate(grid):
        print(f"{i:03}", "".join(line))
    print()


grid = reset_grid()


# let snow fall
def let_snow(verbose: bool = False):
    snow_x, snow_y = snow_start_x, 0
    if verbose: print("snow")
    while True:
        if grid[snow_y + 1][snow_x] == AIR:
            if verbose: print(" -> down")
            snow_y += 1
        elif grid[snow_y+1][snow_x-1] == AIR:
            if verbose: print(" -> left")
            snow_y += 1
            snow_x -= 1
        elif grid[snow_y+1][snow_x+1] == AIR:
            if verbose: print(" -> right")
            snow_y += 1
            snow_x += 1
        else:
            if verbose: print(" -> done")
            grid[snow_y][snow_x] = SNOW
            break

i = 0
while True:
    try:
        let_snow()
        i += 1
    except IndexError:
        break
# print_grid()
print(i)

# part 2
grid = reset_grid()
grid[-1] = list("#" * len(grid[0]))

i = 0
while grid[0][snow_start_x] != SNOW:
    let_snow()
    i += 1
# print_grid()
print(i)