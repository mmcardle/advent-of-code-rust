from typing import List
import sys
import resource, sys
resource.setrlimit(resource.RLIMIT_STACK, (2**29,-1))
sys.setrecursionlimit(10**6)


def make_islands(grid: List[List[str]], character: str) -> int:
    # Initialize count of islands
    points_in_this_island = [[]]

    def dfs(row, col):
        # Mark the current cell as '0' to indicate the land is visited
        grid[row][col] = "."
        # Explore all four directions from the current cell
        for dx, dy in zip(directions[:-1], directions[1:]):
            new_row, new_col = row + dx, col + dy
            # Check if the new cell is within grid bounds and is land ('1')
            if (
                0 <= new_row < rows
                and 0 <= new_col < cols
                and grid[new_row][new_col] == character
            ):
                # Perform DFS on the new cell
                dfs(new_row, new_col)
                points_in_this_island[-1].append((new_row, new_col))

    # Define the directions to explore
    directions = (-1, 0, 1, 0, -1)
    # Get the dimensions of the grid
    rows, cols = len(grid), len(grid[0])
    # Iterate over each cell in the grid

    for row in range(rows):
        for col in range(cols):
            # If the cell is land ('1'), it's a new island
            if grid[row][col] == character:
                # Perform DFS to mark all connected land for the current island
                dfs(row, col)
                # Increment the island count
                points_in_this_island[-1].append((row, col))
            else:
                points_in_this_island.append([])

    # Return the total number of islands
    return [island for island in points_in_this_island if len(island) > 0]


def adjacent_points(x, y):
    return [
        (x + 1, y),
        (x - 1, y),
        (x, y + 1),
        (x, y - 1),
    ]

if __name__ == "__main__":
    input_file = sys.argv[1]
    file = open(input_file, "r").read().split("\n")

    grid = [list(row) for row in file]

    unique_chars = set()
    for col in grid:
        for row in col:
            unique_chars.update(row)

    total_cost = 0
    for character in unique_chars:
        islands = make_islands(grid.copy(), character)

        island_cost = 0
        for island in islands:
            island_fences = 0
            area = len(island)
            for point in island:
                fences = 4
                for adj in adjacent_points(*point):
                    if adj in island:
                        fences -= 1
                print('  ', character, point, adjacent_points(*point), fences)
                island_fences += fences
            print(character, island_fences)
            cost = island_fences * area
            island_cost += cost
        print(character, island_cost)
        total_cost += island_cost
    print(total_cost)