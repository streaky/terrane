from __future__ import annotations

import sys


def main() -> None:
    count = 1_000_000 if sys.argv[1:] == ["performance"] else 1_000
    values: list[float] = []
    total = 0.0
    index = 0
    while index < count:
        raw = float(index % 200 - 100)
        value = 0.01 * raw * raw + float(index % 7) - 3.0
        values.append(value)
        total += value
        index += 1

    mean = total / count
    squared_total = 0.0
    for value in values:
        deviation = value - mean
        squared_total += deviation * deviation
    variance = squared_total / count

    outliers = 0
    for value in values:
        deviation = value - mean
        if deviation * deviation > 2.5 * variance:
            outliers += 1
    print(outliers)


if __name__ == "__main__":
    main()
