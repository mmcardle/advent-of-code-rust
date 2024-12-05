import sys
from typing import List


def read_file(f):
    with open(f, "r") as file:
        return [line.strip() for line in file.readlines()]



def invert_this_list(original: List[List[str]]) ->  List[List[str]]:

    vert_list = []

    for x in range(len(original)):
        for y in range(len(original)):
            if len(vert_list) <= y:
                vert_list.append([])
            vert_list[y].append(original[x][y])

    vv = ["".join(v) for v in vert_list]
    return vv


def find_string(s: List[str], find: str):
    counter = 0
    for line in s:
        if find in line:
            #print(f"Found {find} in line: {line}")
            counter += 1

    print(f"Found {find} {counter} times")
    return counter


def diagonal_content(s: List[List[str]]) -> List[List[str]]:

    first_acums = []

    for i in range(len(s)):
        x = i
        y = 0
        accum2 = []
        while x > -1 and y < len(s):
            accum2.append((x, y))
            x -= 1
            y += 1
        first_acums.append(accum2)

    next_acums = []
    for acum in first_acums[0:-1]:
        ne = [[(len(s) - y - 1 ), (len(s) - x - 1)] for x, y in acum]
        next_acums.append(ne)

    all_acums = first_acums + list(reversed(next_acums))

    result = []
    for acum in all_acums:
        str = ""
        for x, y in acum:
            str += s[x][y]
        result.append(str)

    for x in all_acums:
        st = ""
        for i, j  in x:
            st += s[i][j]
        print(st)

    for x in all_acums:
        print(x)

    return result


def diagonal_content_reserverd(s: List[List[str]]) -> List[List[str]]:

    first_acums = []

    for i in range(len(s) -1, -1, -1):
        x = i
        y = 0
        accum2 = []
        while x < len(s) and y < len(s):
            accum2.append((x, y))
            x += 1
            y += 1
        first_acums.append(accum2)

    next_acums = []
    for acum in first_acums[0:-1]:
        ne = [[(len(s) - y - 1 ), (len(s) - x - 1)] for x, y in acum]
        next_acums.append(ne)

    all_acums = first_acums + list(reversed(next_acums))

    result = []
    for acum in all_acums:
        str = ""
        for x, y in acum:
            str += s[x][y]
        result.append(str)

    for x in all_acums:
        st = ""
        for i, j  in x:
            st += s[i][j]
        print(st)

    for x in all_acums:
        print(x)

    return result


def main():

    content = read_file(sys.argv[1])
    inverted_content = invert_this_list(content)

    assert len(content) == len(inverted_content)

    assert content == invert_this_list(inverted_content)

    dia_content = diagonal_content(content)
    dia_content_reverse = diagonal_content_reserverd(inverted_content)

    print("Original")
    for line in content:
        print(line)

    print("Inverted")
    for line in inverted_content:
        print(line)

    print("Diagonal")
    for line in dia_content:
        print(line)

    print("Diagonal Reversed")
    for line in dia_content_reverse:
        print(line)

    x1 = find_string(content, "XMAS")
    x2 = find_string(content, "SAMX")

    y1 = find_string(inverted_content, "XMAS")
    y2 = find_string(inverted_content, "SAMX")
    
    z1 = find_string(dia_content, "XMAS")
    z2 = find_string(dia_content, "SAMX")

    w1 = find_string(dia_content_reverse, "XMAS")
    w2 = find_string(dia_content_reverse, "SAMX")

    print(
        f"Content: {x1}\n"
        f"Reversed: {x2}\n"
        f"INverted diag XMAS: {y1}\n"
        f"INverted diag SAMX: {y2}\n"
        f"Diagonal XMAS : {z1}\n"
        f"Diagonal SAMX: {z2}\n"
        f"Other diag XMAS: {w1}\n"
        f"Other diag SAMX: {w2}\n"
    )

    total = x1 + x2 + z1 + z2 + w1 + w2 + y1 + y2

    print(f"Total XMAS: {total}")


if __name__ == "__main__":
    main()