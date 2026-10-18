"""Generate a callable wrapper from the selected function's declaration metadata."""

import json
import sys

request = json.load(sys.stdin)
assert request["format"] == 1
[declaration] = request["declarations"]
namespace, name = declaration["identity"].split("::")
source = (
    "namespace generated/consumer\n"
    f"from {namespace} import {name} as selected-command\n"
    "function run-command int;\n"
    "  return selected-command;\n"
)
json.dump(
    {
        "format": 1,
        "generated_sources": [{"identity": "wrapper", "source": source}],
        "diagnostics": [],
    },
    sys.stdout,
)
