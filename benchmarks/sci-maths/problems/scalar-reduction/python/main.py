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


def main() -> None:
    total = sum((index % 1_000 - 500) ** 2 for index in range(size_argument()))
    print(total)


if __name__ == "__main__":
    main()
