import sys
from typing import List


def read_file(f):
    with open(f, "r") as file:
        return [[*line.strip()] for line in file.readlines()]

def checkAt(content, row, col, c1, c2, c3, c4, c5):
    try:
        if (
            content[row  ][col  ] == c1 and 
            content[row  ][col+2] == c2 and
            content[row+1][col+1] == c3 and
            content[row+2][col] == c4 and
            content[row+2][col+2] == c5
        ):
            print(content[row][col], ".", content[row][col + 2])
            print(".", content[row + 1][col + 1], ".")
            print(content[row+2][col], ".", content[row + 2][col + 2])
            return True
    except IndexError:
        pass
    
    return False

'''
DOWN

M.M
.A.
S.S
'''

'''
UP

S.S
.A.
M.M
'''

'''
TO RIGHT

M.S
.A.
M.S
'''

'''
TO LEFT

S.M
.A.
S.M
'''



def main():

    content = read_file(sys.argv[1])
    for i, content_line in enumerate(content):
        print(i, content_line)


    results = []
    for i in range(len(content)):
        for j in range(len(content[i])):
            results.append(checkAt(content, i, j, 'M', 'M', 'A', 'S', 'S'))
            results.append(checkAt(content, i, j, 'S', 'S', 'A', 'M', 'M'))
            results.append(checkAt(content, i, j, 'M', 'S', 'A', 'M', 'S'))
            results.append(checkAt(content, i, j, 'S', 'M', 'A', 'S', 'M'))

    print('OK', len(results), results.count(True))


if __name__ == "__main__":
    main()