import functools
import json


def parse_input_sequence_to_pairs(input_str):
    pairs = input_str.split("\n\n")
    return list(map(lambda x: tuple(x.split("\n")), pairs))


def parse_input_sequence_to_list(input_str):
    output_list = input_str.split("\n")
    return [convert_string_to_list(x) for x in output_list if x]


def convert_string_to_list(input_str):
    return json.loads(input_str)


def compare_values(left, right):
    # If both values are integers, the lower integer should come first. If the left integer is lower than the right integer, the inputs are in the right order. If the left integer is higher than the right integer, the inputs are not in the right order. Otherwise, the inputs are the same integer; continue checking the next part of the input.
    if isinstance(left, int) and isinstance(right, int):
        if left < right:
            return True
        elif left > right:
            return False
        else:
            return None

    # If both values are lists, compare the first value of each list, then the second value, and so on. If the left list runs out of items first, the inputs are in the right order. If the right list runs out of items first, the inputs are not in the right order. If the lists are the same length and no comparison makes a decision about the order, continue checking the next part of the input.
    elif isinstance(left, list) and isinstance(right, list):
        for i in range(min(len(left), len(right))):
            result = compare_values(left[i], right[i])
            if result is not None:
                return result

        if len(left) < len(right):
            return True
        elif len(left) > len(right):
            return False
        else:
            return None

    # If exactly one value is an integer, convert the integer to a list which contains that integer as its only value, then retry the comparison. For example, if comparing [0,0,0] and 2, convert the right value to [2] (a list containing 2); the result is then found by instead comparing [0,0,0] and [2]
    elif isinstance(left, int) and isinstance(right, list):
        return compare_values([left], right)
    elif isinstance(left, list) and isinstance(right, int):
        return compare_values(left, [right])


def sum_indices_correct_order(input_seq):
    results = [
        compare_values(convert_string_to_list(left), convert_string_to_list(right))
        for left, right in parse_input_sequence_to_pairs(input_seq)
    ]

    return sum(i + 1 if val else 0 for i, val in enumerate(results))


def custom_comparator(left, right):
    result = compare_values(left, right)
    if result is None:
        return 0

    elif result:
        return -1

    else:
        return 1


def sort_packets_with_dividers(input_seq):
    input_list = parse_input_sequence_to_list(input_seq)
    input_list.append([[2]])
    input_list.append([[6]])

    input_list.sort(key=functools.cmp_to_key(custom_comparator))

    return input_list


def decoder_key(input_seq):
    sorted_items = sort_packets_with_dividers(input_seq)

    return (sorted_items.index([[2]]) + 1) * (sorted_items.index([[6]]) + 1)


if __name__ == "__main__":
    with open("input.txt", "r") as f:
        input_str = f.read()

    input_seq = parse_input_sequence_to_pairs(input_str)

    print(f"Part 1: {sum_indices_correct_order(input_str)}")
    print(f"Part 2: {decoder_key(input_str)}")
