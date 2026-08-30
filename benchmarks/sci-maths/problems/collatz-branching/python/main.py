from __future__ import annotations

import sys


def main() -> None:
    limit = 1_000_000 if sys.argv[1:] == ["performance"] else 1_000
    total = 0
    start = 1
    while start <= limit:
        value = start
        while value != 1:
            if value % 2 == 0:
                value //= 2
            else:
                value = 3 * value + 1
            total += 1
        start += 1
    print(total)


if __name__ == "__main__":
    main()
