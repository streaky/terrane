from __future__ import annotations

import sys


def main() -> None:
    count = 2_000_000 if sys.argv[1:] == ["performance"] else 1_000
    total = 0.0
    index = 0
    while index < count:
        x = (index % 1_000) / 100.0
        square = x * x
        total += (square + 3.0 * x - 7.0) / (1.0 + square)
        index += 1
    print(total)


if __name__ == "__main__":
    main()
