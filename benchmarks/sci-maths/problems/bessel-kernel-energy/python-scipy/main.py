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
    coordinates = ((indices * 37) % 1_009).astype(np.float64) / 1_009.0
    distances = np.abs(np.subtract.outer(coordinates, coordinates))
    kernel = special.j0(18.0 * distances) / (1.0 + 4.0 * distances * distances)
    print(np.mean(kernel))


if __name__ == "__main__":
    main()
