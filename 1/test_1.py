from main import get_calories_by_elf, get_most_calories, get_top3_calories

def test_get_calories_by_elf():
    with open("test_input.txt", "r") as f:
        file_contents = f.read()

    actual = get_calories_by_elf(file_contents)
    expected = [6000, 4000, 11000, 24000, 10000]

    assert actual == expected

def test_part_one():
    with open("test_input.txt", "r") as f:
        file_contents = f.read()

    expected = 24000
    actual = get_most_calories(file_contents)

    assert actual == expected

def test_part_two():
    with open("test_input.txt", "r") as f:
        file_contents = f.read()

    expected = 45000
    actual = get_top3_calories(file_contents)

    assert actual == expected