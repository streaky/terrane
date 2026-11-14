#!/usr/bin/env python3
"""Command-line entrypoint for sample-profile workflows."""

if __package__:
    from .sample_profile import cli
else:
    from sample_profile import cli

if __name__ == "__main__":
    raise SystemExit(cli())
