#!/usr/bin/env python3
"""Collect Rust implementation coverage, including generated-program execution."""
import sys

if __package__:
    from .rust_coverage import cli
else:
    from rust_coverage import cli


if __name__ == "__main__":
    sys.exit(cli())
