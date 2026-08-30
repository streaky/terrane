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
    value = (index % 1_000) / 100.0
    return value**2 + 3.0 * value - 7.0


def main() -> None:
    values = [transformed(index) for index in range(size_argument())]
    print(sum(values))


if __name__ == "__main__":
    main()
