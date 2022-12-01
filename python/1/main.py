def get_calories_by_elf(input_str):
    calories = input_str.split("\n")
    calories = list(map(lambda x: int(x) if x else 0, calories))

    calories_by_elf = []
    cur_max = 0
    for i, calorie in enumerate(calories):
        if calorie == 0:
            calories_by_elf.append(cur_max)
            cur_max = 0
        elif i == len(calories) - 1:
            cur_max += calorie
            calories_by_elf.append(cur_max)
        else:
            cur_max += calorie

    return calories_by_elf

def get_most_calories(input_str: str):
    return max(get_calories_by_elf(input_str))

def get_top3_calories(input_str: str):
    calories_by_elf = get_calories_by_elf(input_str)
    calories_by_elf.sort(reverse=True)

    return sum(calories_by_elf[:3])

def main():
    with open("input.txt", "r") as f:
        file_contents = f.read()

    print(f"top calories: {get_most_calories(file_contents)}")
    print(f"top3 calories: {get_top3_calories(file_contents)}")


if __name__ == "__main__":
    main()