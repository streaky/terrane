import sys


def size_argument() -> int:
    if len(sys.argv) != 2:
        raise SystemExit("expected exactly one positive integer size")
    try:
        size = int(sys.argv[1])
    except ValueError as error:
        raise SystemExit("size must be a positive integer") from error
    if size <= 0:
        raise SystemExit("size must be a positive integer")
    return size


def stopping_steps(start: int) -> int:
    steps = 0
    value = start
    while value != 1:
        value = value // 2 if value % 2 == 0 else 3 * value + 1
        steps += 1
    return steps


def main() -> None:
    total = sum(stopping_steps(start) for start in range(1, size_argument() + 1))
    print(total)


if __name__ == "__main__":
    main()
