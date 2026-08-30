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


def transformed(index: int) -> float:
    x = (index % 1_000) / 100.0
    square = x * x
    return (square + 3.0 * x - 7.0) / (1.0 + square)


def main() -> None:
    print(sum(transformed(index) for index in range(size_argument())))


if __name__ == "__main__":
    main()
