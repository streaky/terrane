from __future__ import annotations

import sys


def main() -> None:
    count = 50_000_000 if sys.argv[1:] == ["performance"] else 1_000
    total = 0
    index = 0
    while index < count:
        value = index % 1_000 - 500
        total += value * value
        index += 1
    print(total)


if __name__ == "__main__":
    main()
