#!/usr/bin/env python3
"""Generate a typed Terrane CLI binder from canonical declaration metadata."""
import argparse
import os
import sys

sys.path.insert(0, os.path.dirname(__file__))
from metadata import annotation, emit, read_input, scalar, quote, ConsumerError



def diagnostic(declaration, message):
    span = declaration.get("origin") or declaration.get("span") or {"file": 0, "start": 0, "end": 0}
    emit(diagnostics=[{"declaration": span, "message": message}])


def annotation_of(declaration, name):
    try:
        return annotation(declaration, name, "/annotation-cli")
    except ValueError as error:
        raise ValueError(f"{declaration.get('identity', '<declaration>')}: {error}") from error


def type_kind(type_name):
    kind = (type_name or "").strip().lower()
    if kind in {"string", "str"}:
        return "string"
    if kind == "int":
        return "int"
    raise ValueError(f"unsupported CLI parameter type {type_name!r}; supported types are string and int")


def default_literal(value, kind, type_name):
    if value is None:
        raise ValueError(f"optional parameter of type {type_name!r} requires a compile-time default")
    actual = value.get("kind")
    raw = scalar(value)
    expected = "string" if kind == "string" else "integer"
    if actual != expected:
        raise ValueError(f"default value does not match parameter type {type_name!r}")
    return quote(raw) if kind == "string" else str(raw)


def selected_parameter(declarations, function, parameter):
    identity = function["identity"] + "::" + parameter["name"]
    matches = [item for item in declarations if item.get("kind") == "parameter" and item.get("identity") == identity]
    if len(matches) != 1:
        raise ValueError(f"selected command parameter {identity!r} must have exactly one selected metadata record")
    return matches[0]


def generate(request):
    declarations = request["declarations"]
    callables = [item for item in declarations if item.get("kind") == "callable"
                 and annotation_of(item, "command") is not None]
    if len(callables) != 1:
        raise ValueError("CLI consumer requires exactly one selected callable annotated as a command")
    function = callables[0]
    command = annotation_of(function, "command") or {}
    signature = function.get("signature")
    if not isinstance(signature, dict):
        raise ValueError("selected command has no callable signature")
    if signature.get("async") or signature.get("throws") or signature.get("unsafe"):
        raise ValueError("CLI command must be synchronous, non-throwing, and safe")
    if function.get("receiver") or signature.get("receiver"):
        raise ValueError("CLI command receiver methods are unsupported")
    parameters = signature.get("parameters")
    if not isinstance(parameters, list):
        raise ValueError("selected command signature has no parameter list")
    if any(p.get("variadic") or p.get("mutable") for p in parameters):
        raise ValueError("variadic or mutable CLI command parameters are unsupported")

    command_name = scalar(command.get("name", {"kind": "string", "value": function["identity"].rsplit("::", 1)[-1]}))
    if command_name is None:
        command_name = function["identity"].rsplit("::", 1)[-1]
    if not isinstance(command_name, str):
        raise ValueError("command name must be text")
    parser = argparse.ArgumentParser(add_help=False)
    parser.add_argument("--summary")
    cli_options, _ = parser.parse_known_args(sys.argv[1:])
    override = cli_options.summary
    declared_summary = scalar(command.get("summary", {"kind": "none"}))
    if signature.get("result") != "int":
        raise ValueError("CLI command must return an integer exit status")
    summary = override if override is not None else declared_summary if declared_summary is not None else function.get("documentation") or ""
    if not isinstance(summary, str):
        raise ValueError("command summary must be text")
    options, flags = [], set()
    for parameter in parameters:
        record = selected_parameter(declarations, function, parameter)
        try:
            if parameter.get("type_kind") != "scalar":
                raise ValueError("CLI parameter must have a canonical scalar type")
            kind = type_kind(parameter.get("type"))
        except ValueError as error:
            raise ConsumerError(record, str(error)) from error
        metadata = annotation_of(record, "command-option") or {}
        flag = scalar(metadata.get("long-name", {"kind": "string", "value": "--" + parameter["name"].replace("_", "-")}))
        if not isinstance(flag, str) or not flag.startswith("--") or len(flag) < 3 or "=" in flag:
            raise ValueError(f"option for {parameter['name']} must use a --long-name")
        if flag in flags or flag == "--help":
            raise ValueError(f"duplicate or reserved CLI option {flag!r}")
        flags.add(flag)
        optional = bool(parameter.get("optional")) or parameter.get("default") is not None
        default = default_literal(parameter.get("default"), kind, parameter.get("type")) if optional else None
        help_text = scalar(metadata.get("help", {"kind": "none"}))
        if help_text is None:
            help_text = record.get("documentation")
        if help_text is None:
            help_text = parameter["name"]
        if not isinstance(help_text, str):
            raise ValueError(f"help for {parameter['name']} must be text")
        options.append((parameter, kind, flag, optional, default, help_text, record))
    namespace, function_name = function["identity"].rsplit("::", 1)
    lines = ["namespace generated/annotation-cli", "from /core/output import print",
             "from /core/process import arguments, native-string, cli-schema, parse-command-line",
             "from /core/collections import list",
             "from /core/errors import coercion-error",
             f"from {namespace} import {function_name} as selected-command", "",
             "function run-command int;", "    user-arguments = arguments;",
             "    schema-entries list of string = (list; 'flag:--help')"]
    for _, _, flag, *_ in options:
        lines.append(f"    schema-entries.append; {quote('value:' + flag)}")
    lines.extend(["    schema = instance cli-schema; schema-entries", "    parsed = parse-command-line; schema, user-arguments",
                  "    if parsed.diagnostic-messages.length > 0", "        index int = 0",
                  "        while index < parsed.diagnostic-messages.length", "            print; parsed.diagnostic-messages[index]",
                  "            index = index + 1", "        return 2", "    if parsed.flags.length > 0",
                  "        print; " + quote("Usage: " + str(command_name) + " [options]"), "        print; " + quote(summary)])
    for p, kind, flag, optional, _, help_text, _ in options:
        shown_default = f" (default: {p['default'].get('value')})" if optional else ""
        lines.append("        print; " + quote(f"  {flag} <{kind}>{shown_default}  {help_text}"))
    lines.extend(["        print; '  --help  Show this help'", "        return 0"])
    for p, kind, _, optional, default, _, _ in options:
        initial = default if optional else ("''" if kind == "string" else "0")
        lines.append(f"    {p['name']} {p['type']} = {initial}")
        if not optional:
            lines.append(f"    seen-{p['name']} bool = false")
    lines.extend(["    option-index int = 0", "    while option-index < parsed.option-names.length",
                  "        option-name = parsed.option-names[option-index]", "        option-value = parsed.option-values[option-index]"])
    for p, kind, flag, optional, *_ in options:
        pname = p["name"]
        lines.append(f"        if option-name == {quote(flag)}")
        if kind == "string":
            lines.append(f"            {pname} = option-value.text")
        else:
            lines.extend(["            try", f"                {pname} = option-value.text.radix; 10",
                          "            catch coercion-error", f"                print; {quote('invalid integer value for ' + flag)}",
                          "                return 2"])
        if not optional:
            lines.append(f"            seen-{pname} = true")
    lines.append("        option-index = option-index + 1")
    for p, _, flag, optional, *_ in options:
        if not optional:
            lines.extend([f"    if not seen-{p['name']}", f"        print; {quote('missing required option ' + flag)}", "        return 2"])
    call_args = ", ".join(p["name"] for p, *_ in options)
    lines.extend([f"    return selected-command; {call_args}", ""])
    return function, "\n".join(lines)


def main():
    function = {}
    request = None
    try:
        request = read_input()
        function = next((item for item in request["declarations"] if item.get("kind") == "callable"
                         and annotation_of(item, "command") is not None), {})
        selected_function, source = generate(request)
        emit(sources=[{"identity": "cli-binder", "source": source}])
    except ConsumerError as error:
        diagnostic(error.declaration, str(error))
    except (KeyError, TypeError, ValueError) as error:
        diagnostic(function, str(error))


if __name__ == "__main__":
    main()
