#!/usr/bin/env python3
"""Explicit scalar/nullable codec consumer of canonical declaration metadata."""
import argparse
import json
import os
import sys
sys.path.insert(0, os.path.dirname(__file__))
from metadata import read_input, emit, fail, annotation as find_annotation, scalar, quote, selected, ConsumerError


def annotation(declaration, name):
    return find_annotation(declaration, name, "/annotation-codec")


def record_for(request, field):
    origin = field['origin']
    for record in request['declarations']:
        span = record['span']
        if record['kind'] == 'field' and all(span[key] == origin[key] for key in ('file', 'start', 'end')):
            return record
    raise ValueError('canonical field declaration metadata must be supplied with the selected class')


def describe(request, cls):
    if cls.get('visibility') != 'public' or cls['contracts'].get('constructor') is not None or cls['contracts'].get('base') is not None:
        raise ValueError('codec requires a public non-derived class with its canonical default constructor')
    fields = []
    wires = set()
    for field in cls['fields']:
        if field.get('static') or field.get('visibility') != 'public' or field.get('secret'):
            continue
        record = record_for(request, field)
        if annotation(record, 'secret') is not None:
            continue
        spelling = field['type'].replace(' ', '')
        nullable = spelling.endswith('|none')
        base = spelling[:-5] if nullable else spelling
        if field.get('type_kind') not in ('scalar', 'optional-scalar') or base not in ('string', 'bool', 'int', 'int8', 'int16', 'int32', 'int64', 'int128', 'uint8', 'uint16', 'uint32', 'uint64', 'uint128'):
            raise ConsumerError(record, 'codec does not support this canonical field type: ' + field['type'])
        alias = annotation(record, 'external-name')
        wire = scalar(alias['value']) if alias is not None else field['external_name']
        if not isinstance(wire, str) or wire in wires:
            raise ConsumerError(record, 'codec wire name must be text and unique')
        wires.add(wire)
        required = annotation(record, 'required') is not None or field['required']
        defaulted = field['defaulted']
        if defaulted and field.get('default') is None:
            raise ConsumerError(record, 'codec requires an immutable representable field default')
        minimum = annotation(record, 'minimum')
        maximum = annotation(record, 'maximum')
        if (minimum is not None or maximum is not None) and not base.startswith(('int', 'uint')):
            raise ConsumerError(record, 'numeric validation requires a canonical integer field')
        low = int(scalar(minimum['value'])) if minimum is not None else None
        high = int(scalar(maximum['value'])) if maximum is not None else None
        if low is not None and high is not None and low > high:
            raise ConsumerError(record, 'minimum exceeds maximum')
        if base != 'int' and base.startswith(('int', 'uint')):
            unsigned = base.startswith('uint')
            bits = int(base[4:] if unsigned else base[3:])
            floor = 0 if unsigned else -(1 << (bits - 1))
            ceiling = (1 << bits) - 1 if unsigned else (1 << (bits - 1)) - 1
            if low is not None and low > ceiling or high is not None and high < floor:
                raise ConsumerError(record, 'numeric constraints have no representable value')
            low = low if low is not None and low > floor else None
            high = high if high is not None and high < ceiling else None
        fields.append((field, wire, base, nullable, required, defaulted, low, high))
    return fields


def constraints(lines, expression, low, high, indent, failure, wire):
    for boundary, operator, label in ((low, '<', 'minimum'), (high, '>', 'maximum')):
        if boundary is not None:
            lines.extend([indent + f'if {expression} {operator} {boundary}', indent + '    return ' + failure + '; ' + quote(wire + ' violates ' + label)])


def generate(request):
    cls = selected(request, 'class')
    fields = describe(request, cls)
    namespace, name = cls['identity'].rsplit('::', 1)
    parser = argparse.ArgumentParser()
    parser.add_argument("--description")
    parser.add_argument("--include-schema", action="store_true")
    parser.add_argument("--schema-only", action="store_true")
    options = parser.parse_args()
    def prose(record):
        explicit = annotation(record, 'description')
        return scalar(explicit['value']) if explicit is not None else record.get('documentation')
    schema = {'description': options.description if options.description is not None else prose(cls), 'fields': {}}
    for field, wire, base, nullable, required, defaulted, low, high in fields:
        schema['fields'][wire] = {'type': base, 'nullable': nullable, 'required': required, 'description': prose(record_for(request, field))}
    schema_text = json.dumps(schema, ensure_ascii=False, sort_keys=True, separators=(',', ':'))
    schema_source = '\n'.join(['namespace generated/annotation-codec', f'function describe-{name} string;', '    return ' + quote(schema_text), ''])
    if options.schema_only:
        emit(sources=[{'identity': 'typed-codec', 'source': schema_source}])
        return
    lines = [
        'namespace generated/annotation-codec',
        f'from {namespace} import {name}',
        'from /core/documents import document-value, document-map-entries, make-document-none, make-document-bool, make-document-string, make-document-integer, make-document-map',
        'from /core/errors import coercion-error, integer-conversion-overflow', '',
        f'class {name}-decoded', '    failed bool = false', "    message string = ''", f'    value {name}|none = none', '',
        f'class {name}-encoded', '    failed bool = false', "    message string = ''", '    value document-value|none = none', '',
    ]
    if options.include_schema:
        lines.extend([f'function describe-{name} string;', '    return ' + quote(schema_text), ''])
    for result in ('decoded', 'encoded'):
        lines.extend([f'function fail-{result} {name}-{result}; message string', f'    result = instance {name}-{result};', '    result.failed = true', '    result.message = message', '    return result', ''])
    lines.extend([f'function decode-{name} {name}-decoded; document document-value', "    if document.kind != 'map'", "        return fail-decoded; 'codec expects an object'", '    key-count = document.length;', '    index int = 0', '    while index < key-count', '        key = document.key; index'])
    condition = ' and '.join(f'key != {quote(wire)}' for _, wire, *_ in fields) or 'true'
    lines.extend(['        if ' + condition, "            return fail-decoded; ('unknown or inaccessible field: '.concat; key)", '        index = index + 1', f'    value = instance {name};'])
    for index, (field, wire, base, nullable, required, defaulted, low, high) in enumerate(fields):
        member = field['name']
        slot = 'field-slot' + str(index)
        lines.append(f'    {slot} = document.field; {quote(wire)}')
        if required or not (defaulted or nullable):
            lines.extend([f'    if {slot}.failed', '        return fail-decoded; ' + quote('missing required field: ' + wire)])
        lines.append(f'    if not {slot}.failed')
        indent = '        '
        if nullable:
            lines.extend([indent + f"if {slot}.value.kind == 'none'", indent + f'    value.{member} = none', indent + 'else'])
            indent += '    '
        kind = 'integer' if base.startswith(('int', 'uint')) else base
        lines.extend([indent + f'if {slot}.value.kind != {quote(kind)}', indent + '    return fail-decoded; ' + quote('wrong document type for ' + wire)])
        expression = f'{slot}.value.scalar'
        if kind == 'bool':
            expression = f"({slot}.value.scalar == 'true')"
        if kind == 'integer':
            conversion = f'{slot}.value.integer.text.radix; 10'
            if base != 'int':
                conversion = f'({conversion}).coerce; {base}'
            lines.extend([indent + 'try', indent + f'    parsed-value{index} {base} = {conversion}'])
            expression = 'parsed-value' + str(index)
            constraints(lines, expression, low, high, indent + '    ', 'fail-decoded', wire)
            lines.append(indent + f'    value.{member} = {expression}')
            lines.extend([indent + 'catch coercion-error', indent + '    return fail-decoded; ' + quote('integer out of range for ' + wire)])
            if base != 'int':
                lines.extend([indent + 'catch integer-conversion-overflow', indent + '    return fail-decoded; ' + quote('integer out of range for ' + wire)])
            continue
        lines.append(indent + f'value.{member} = {expression}')
    lines.extend([f'    result = instance {name}-decoded;', '    result.value = value', '    return result', '', f'function encode-{name} {name}-encoded; value {name}', '    entries = instance document-map-entries;'])
    for index, (field, wire, base, nullable, _, _, low, high) in enumerate(fields):
        member = field['name']
        indent = '    '
        expression = 'value.' + member
        if nullable:
            local = f'value-field{index}'
            lines.extend([indent + f'{local} = {expression}', indent + f'if {local} != none'])
            expression = local
            indent += '    '
        if base.startswith(('int', 'uint')):
            constraints(lines, expression, low, high, indent, 'fail-encoded', wire)
            lines.extend([indent + f"encoded-value{index} = make-document-integer; (''.concat; {expression})", indent + f'if encoded-value{index}.failed', indent + f'    return fail-encoded; encoded-value{index}.message', indent + f'entries.append; {quote(wire)}, encoded-value{index}.value'])
        else:
            factory = 'make-document-' + base
            lines.append(indent + f'entries.append; {quote(wire)}, ({factory}; {expression})')
        if nullable:
            lines.extend(['    else', f'        entries.append; {quote(wire)}, (make-document-none;)'])
    lines.extend(['    document = make-document-map; entries', '    if document.failed', '        return fail-encoded; document.message', f'    result = instance {name}-encoded;', '    result.value = document.value', '    return result', ''])
    emit(sources=[{'identity': 'typed-codec', 'source': '\n'.join(lines)}])


def main():
    request = None
    try:
        request = read_input()
        generate(request)
    except ConsumerError as error:
        fail(error.declaration, str(error))
    except (ValueError, KeyError, TypeError) as error:
        fail(request['declarations'][0] if request and request.get('declarations') else {}, str(error))


if __name__ == '__main__':
    main()
