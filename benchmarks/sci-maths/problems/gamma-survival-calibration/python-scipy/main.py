import sys

import numpy as np
from scipy import special


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
    indices = np.arange(size_argument(), dtype=np.int64)
    shapes = 1.25 + (indices % 17).astype(np.float64) * 0.125
    observations = 0.5 + (indices % 101).astype(np.float64) * 0.05
    targets = 0.2 + (indices % 7).astype(np.float64) * 0.1
    residuals = special.gammaincc(shapes, observations) - targets
    print(np.mean(residuals * residuals))


if __name__ == "__main__":
    main()
