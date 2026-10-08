#!/bin/sh
set -eu
root=$(CDPATH= cd -- "$(dirname -- "$0")/../.." && pwd)
compiler=${TERRANE:-"$root/target/debug/terrane"}
mkdir -p "$root/target/annotation-consumers"
for tool in cli codec; do
    artifact=$("$compiler" build "$root/tools/annotation-consumers/$tool-consumer/package.toml")
    cp "$artifact" "$root/target/annotation-consumers/$tool"
    case "$tool" in
        cli) fixtures="run/annotation-cli-consumer reject/annotation-cli-unsupported-scalar reject/annotation-cli-runtime-default" ;;
        codec) fixtures="run/annotation-codec-consumer run/annotation-source-free reject/annotation-codec-collection reject/annotation-codec-constraint reject/annotation-codec-no-selection" ;;
    esac
    for fixture in $fixtures; do
        directory="$root/tests/conformance/$fixture/.trn/consumers"
        mkdir -p "$directory"
        cp "$artifact" "$directory/$tool"
    done
done
