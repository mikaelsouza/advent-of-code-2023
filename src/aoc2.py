from dataclasses import dataclass
from math import prod


@dataclass
class Game:
    game_number: int
    sets: list[dict[str, int]]


P1_RULES = {
    "red": 12,
    "green": 13,
    "blue": 14,
}


def main():
    with open("inputs/2-a.txt") as f:
        inputs: list[str] = f.read().splitlines()

    possible_game_sum = 0
    total_cube_power = 0
    for line in inputs:
        game = process_input(line)

        result = is_game_possible(rules=P1_RULES, game=game)
        possible_game_sum += game.game_number if result else 0

        cube_power = prod(min_cubes(game).values())
        total_cube_power += cube_power

        print(f"Result for game {game.game_number} is {result}")
        print(f"Cube Power: {cube_power}")

    print(f"Final Result {possible_game_sum}")
    print(f"Final Cube Power: {total_cube_power}")


def process_input(input: str) -> Game:
    game_num = int(input.split()[1][:-1])
    cube_sets: str = input.split(":")[1].split(";")
    game_sets = list()
    for s in cube_sets:
        cubes = dict()
        for cube in s.split(","):
            num_cubes, cube_color = cube.split()
            cubes[cube_color] = int(num_cubes)
        game_sets.append(cubes)
    return Game(game_num, game_sets)


def is_game_possible(rules: dict[str, int], game: Game) -> list[bool]:
    for set in game.sets:
        tmp_rules = rules.copy()
        for color in set:
            tmp_rules[color] = tmp_rules.get(color, 0) - set[color]
        for cube in tmp_rules.values():
            if cube < 0:
                return False
    return True


def min_cubes(game: Game):
    min_necessary: dict[str, int] = dict()
    for s in game.sets:
        for color in s:
            min_necessary[color] = max(min_necessary.get(color, 0), s[color])
    return min_necessary


if __name__ == "__main__":
    main()
