

def abs_diff(a, b):
    return abs(a - b)

def as_red_text(s):
    return f"\033[91m{s}\033[00m"
    

def as_green_text(s):
    return f"\033[92m{s}\033[00m"

def all_increasing(list_of_numbers):
    return all(a < b for a, b in zip(list_of_numbers, list_of_numbers[1:]))

def all_decreasing(list_of_numbers):
    return all(a > b for a, b in zip(list_of_numbers, list_of_numbers[1:]))


def is_valid(a, b):
    if a == b:
        return False, as_red_text("  =  ")
    abs_diff_value = abs_diff(a, b)
    if abs_diff_value > 3:
        return False, as_red_text(f" <{abs_diff_value}> ")
    
    return True, as_green_text("     ")

def pad_number(number):
    return f"{number:02d}"

if __name__ == "__main__":
    import sys
    with open(sys.argv[1]) as f:
        lines = f.readlines()

    valid_lines = 0
    for line in lines:
        line = line.strip()
        numbers = list(map(int, line.split(" ")))

        numbers_as_pairs = list(zip(numbers, numbers[1:]))

        diffs_are_valid = True
        increasing = all_increasing(numbers)
        decreasing = all_decreasing(numbers)

        errors = []
        
        for pair in numbers_as_pairs:
            valid, error = is_valid(*pair)
            errors.append(error)
            diffs_are_valid = diffs_are_valid and valid

        either_increasing_or_decreasing = increasing or decreasing

        totally_valid = diffs_are_valid and (increasing or decreasing)

        if totally_valid:
            valid_lines += 1

        if not totally_valid:
            for i, number in enumerate(numbers):
                errors_to_print = " ".join(errors[i:i+1])
                print(f"{pad_number(number)} {errors_to_print}", end=" ")

            print(
                as_red_text("Invalid - Diffs") if not diffs_are_valid else "",
                as_red_text("Invalid - Not Increasing") if not either_increasing_or_decreasing else "",
                as_green_text("Valid") if totally_valid else "",
            )
    
    print(f"Valid lines: {valid_lines}")