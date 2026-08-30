from __future__ import annotations

import sys


def main() -> None:
    count = 10_000_000 if sys.argv[1:] == ["performance"] else 1_000
    transformed: list[float] = []
    index = 0
    while index < count:
        x = (index % 1_000) / 100.0
        transformed.append(x * x + 3.0 * x - 7.0)
        index += 1

    total = 0.0
    for value in transformed:
        total += value
    print(total)


if __name__ == "__main__":
    main()
