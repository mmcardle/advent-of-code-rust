"""A word search solver"""

from collections import namedtuple
from itertools import product
import re
import sys

Direction = namedtuple('Direction', 'di dj name')

DIRECTIONS = [
    Direction(-1, -1, "up and to the left"),
    Direction(-1,  0, "up"),
    Direction(-1, +1, "up and to the right"),
    Direction( 0, -1, "left"),
    Direction( 0, +1, "right"),
    Direction(+1, -1, "down and to the left"),
    Direction(+1,  0, "down"),
    Direction(+1, +1, "down and to the right"),
]

def read_grid(filename):
    """
    Read a word search puzzle from a file into a 2D matrix of uppercase letters.
    """
    with open(filename) as f:
        return [re.findall('[A-Z]', line.upper()) for line in f]

def extract(grid, i, j, dir, length):
    """
    Extract letters from the grid, starting at row i column j, as a string.
    If the extraction will walk out of bounds, return None.
    """
    if ( 0 <= i + (length - 1) * dir.di < len(grid) and
         0 <= j + (length - 1) * dir.dj < len(grid[i]) ):
        return ''.join(
            grid[i + n * dir.di][j + n * dir.dj] for n in range(length)
        )
    return None

def search(grid, word):
    """
    Search for a word in a grid, returning a tuple of the starting row,
    starting column, and direction.  If the word is not found, return None.
    """
    word_len = len(word)
    matches = []
    for i, j, dir in product(range(len(grid)), range(len(grid[0])), DIRECTIONS):
        if word == extract(grid, i, j, dir, word_len):
            matches.append((i, j, dir))
    
    return matches

def main(filename, word):
    grid = read_grid(filename)
    matches = search(grid, word.upper())
    if matches is None:
        print("Didn't find a match.")
    else:
        for match in matches:
            i, j, dir = match
            print("Found a match at line {0}, column {1} going {2}".format(
                i + 1, j + 1, dir.name))
        print(f"Found a matches {len(matches)}")

if __name__ == '__main__':
    main(sys.argv[1], sys.argv[2])