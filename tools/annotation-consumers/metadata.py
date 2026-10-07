#!/usr/bin/env python3
"""Shared version-1 declaration-consumer wire helpers."""

import json
import sys


class ConsumerError(ValueError):
    def __init__(self, declaration, message):
        super().__init__(message)
        self.declaration = declaration


def read_input():
    sys.set_int_max_str_digits(0)
    value = json.load(sys.stdin)
    if value.get("format") != 1 or not isinstance(value.get("declarations"), list):
        raise ValueError("expected declaration-consumer protocol format 1")
    return value


def tagged(value, kind=None):
    if not isinstance(value, dict) or "kind" not in value:
        raise ValueError("malformed compile-time value")
    actual = value["kind"]
    if kind is not None and actual != kind:
        raise ValueError(f"expected {kind}, received {actual}")
    return value.get("value")


def payload(annotation):
    pairs = tagged(annotation.get("payload"), "map")
    if not isinstance(pairs, list):
        raise ValueError("annotation payload is not a map")
    result = {}
    for pair in pairs:
        if not isinstance(pair, list) or len(pair) != 2:
            raise ValueError("malformed annotation payload entry")
        key = tagged(pair[0], "string")
        if key in result:
            raise ValueError(f"duplicate annotation payload key {key!r}")
        result[key] = pair[1]
    return result


def scalar(value):
    kind = value.get("kind")
    if kind in {"none", "boolean", "integer", "float", "string"}:
        return value.get("value")
    raise ValueError(f"metadata value {kind!r} is not a supported scalar")


def annotation(declaration, suffix, namespace):
    matches = [item for item in declaration.get("annotations", [])
               if item.get("identity") == namespace + "::" + suffix]
    if len(matches) > 1:
        raise ValueError(f"declaration has repeated {suffix} metadata")
    return payload(matches[0]) if matches else None


def fail(declaration, message):
    span = declaration.get("span") or declaration.get("origin") or {"file": 0, "start": 0, "end": 0}
    emit(diagnostics=[{"declaration": span, "message": message}])


def emit(sources=(), diagnostics=()):
    json.dump({"format": 1, "generated_sources": list(sources), "diagnostics": list(diagnostics)},
              sys.stdout, ensure_ascii=False, sort_keys=True, separators=(",", ":"))
    sys.stdout.write("\n")


def quote(value):
    return "'" + value.replace("\\", "\\\\").replace("'", "\\'").replace("\n", "\\n").replace("\r", "\\r").replace("\t", "\\t") + "'"


def selected(request, kind):
    matches = [record for record in request["declarations"] if record["kind"] == kind]
    if len(matches) != 1:
        raise ValueError("consumer requires exactly one selected " + kind)
    return matches[0]
