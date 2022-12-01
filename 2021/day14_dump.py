
    triggers = set()
    letters = set()
    for insertion in insertions:
        print(f"insertion {insertion}")
        letters.add(insertion[0][0])
        letters.add(insertion[0][1])
        triggers.add(insertion[0])

    print(f"chars {letters}")

    non_triggers = set()
    for p in permutations(letters, 2):
        print(f"permutations {p}")
        possible_p = p[0] + p[1]
        if possible_p not in triggers:
            print(f"Found {possible_p}")
            non_triggers.add(possible_p)

    print(f"non_triggers {non_triggers}")

    return None

    exit()