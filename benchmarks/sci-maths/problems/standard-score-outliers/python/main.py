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


def generated_value(index: int) -> float:
    raw = float(index % 200 - 100)
    return 0.01 * raw * raw + float(index % 7) - 3.0


def main() -> None:
    count = size_argument()
    values = [generated_value(index) for index in range(count)]
    mean = sum(values) / count
    variance = sum((value - mean) ** 2 for value in values) / count
    outliers = sum((value - mean) ** 2 > 2.5 * variance for value in values)
    print(outliers)


if __name__ == "__main__":
    main()
