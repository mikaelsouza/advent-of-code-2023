DICT_VALUES = {
    "one": 1,
    "two": 2,
    "three": 3,
    "four": 4,
    "five": 5,
    "six": 6,
    "seven": 7,
    "eight": 8,
    "nine": 9,
}


def get_possibilities(text: str) -> list[str]:
    return [
        text[:3],
        text[:4],
        text[:5],
    ]


def str_sum(line: str, rev=False) -> int:
    if rev:
        line = line[::-1]

    for index, value in enumerate(line):
        if value.isdigit():
            return int(value)

        pos_input = line[index::-1] if rev else line[index:]
        possibilities = get_possibilities(pos_input)

        for possibility in possibilities:
            if possibility in DICT_VALUES:
                return DICT_VALUES[possibility]

    return 0


def main():
    data = None

    with open("inputs/1-b.txt") as f:
        data: list[str] = f.read().splitlines()

    result = 0

    for line in data:
        r = 10 * str_sum(line) + str_sum(line, rev=True)
        print(f"Line Result: {r}")
        result += r

    print(f"Final Result {result}")


if __name__ == "__main__":
    main()
