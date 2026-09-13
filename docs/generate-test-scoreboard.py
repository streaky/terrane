#!/usr/bin/env python3
"""Generate the self-contained Terrane test timing scoreboard HTML."""

from __future__ import annotations

import argparse
import html
import json
import sys
from pathlib import Path
from typing import Any

try:
    import yaml
except ImportError as error:
    raise SystemExit("PyYAML is required: install it with `python -m pip install pyyaml`") from error

STATUSES = {"passed", "failed", "ignored"}


def parse_args() -> argparse.Namespace:
    script_dir = Path(__file__).resolve().parent
    parser = argparse.ArgumentParser(description="Generate the Terrane test timing scoreboard HTML.")
    parser.add_argument("--input", type=Path, default=script_dir / "test-scoreboard.yaml")
    parser.add_argument("--output", type=Path, default=script_dir / "test-scoreboard.html")
    parser.add_argument("--check", action="store_true", help="fail if --output differs instead of writing it")
    return parser.parse_args()


def require(condition: bool, message: str) -> None:
    if not condition:
        raise ValueError(message)


def validate(data: Any) -> dict[str, Any]:
    require(isinstance(data, dict), "scoreboard root must be a mapping")
    require(data.get("format") == 1, "scoreboard format must be 1")
    require(isinstance(data.get("title"), str) and data["title"], "title must be non-empty")
    require(isinstance(data.get("metadata"), dict), "metadata must be a mapping")
    require(isinstance(data.get("runs"), list), "runs must be a list")
    require(isinstance(data.get("tests"), list), "tests must be a list")
    seen: set[str] = set()
    for index, test in enumerate(data["tests"]):
        prefix = f"tests[{index}]"
        require(isinstance(test, dict), f"{prefix} must be a mapping")
        require(isinstance(test.get("id"), str) and test["id"], f"{prefix}.id is required")
        require(test["id"] not in seen, f"duplicate test id: {test['id']}")
        seen.add(test["id"])
        require(isinstance(test.get("target"), str) and test["target"], f"{prefix}.target is required")
        require(isinstance(test.get("name"), str) and test["name"], f"{prefix}.name is required")
        require(test.get("status") in STATUSES, f"{prefix}.status is invalid")
        require(isinstance(test.get("seconds"), (int, float)) and test["seconds"] >= 0, f"{prefix}.seconds is invalid")
        previous = test.get("previous_seconds")
        require(previous is None or isinstance(previous, (int, float)), f"{prefix}.previous_seconds is invalid")
        require(isinstance(test.get("history"), list) and test["history"], f"{prefix}.history is required")
    return data


def render(data: dict[str, Any]) -> str:
    title = html.escape(data["title"])
    description = html.escape(data["metadata"]["description"])
    payload = json.dumps(data, ensure_ascii=False, indent=2).replace("</", "<\\/")
    return f'''<!doctype html>
<!-- Generated from test-scoreboard.yaml by generate-test-scoreboard.py; do not edit directly. -->
<html lang="en"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width, initial-scale=1">
<title>{title}</title><style>
:root{{color-scheme:dark;--bg:#0b1018;--panel:#121b28;--line:#28384e;--ink:#eef5ff;--muted:#91a2b8;--accent:#69d2ff;--pass:#5dd39e;--fail:#ff7185;--skip:#a6afbd;--shadow:0 20px 60px #0007}}*{{box-sizing:border-box}}body{{margin:0;background:radial-gradient(circle at 15% 0,#17324a 0,transparent 34rem),var(--bg);color:var(--ink);font:15px/1.45 Inter,ui-sans-serif,system-ui,sans-serif}}button,input,select{{font:inherit}}.shell{{max-width:1580px;margin:auto;padding:34px clamp(18px,4vw,64px) 70px}}.hero{{display:flex;justify-content:space-between;gap:24px;align-items:end;margin-bottom:24px}}.eyebrow{{color:var(--accent);font-size:12px;font-weight:800;letter-spacing:.14em;text-transform:uppercase}}h1{{margin:6px 0 12px;font:700 clamp(34px,5vw,64px)/1 Georgia,serif;letter-spacing:-.04em}}.lede{{max-width:850px;color:var(--muted)}}.stamp{{color:var(--muted);white-space:nowrap}}.metrics{{display:grid;grid-template-columns:repeat(4,1fr);gap:12px;margin-bottom:20px}}.metric{{padding:17px;background:linear-gradient(145deg,#172335,#101823);border:1px solid var(--line);border-radius:15px;box-shadow:var(--shadow)}}.metric strong{{display:block;font-size:28px}}.metric span{{color:var(--muted);font-size:11px;text-transform:uppercase;letter-spacing:.08em}}.filters{{display:grid;grid-template-columns:2fr 1fr 1fr auto;gap:10px;margin-bottom:12px}}input,select,button{{border:1px solid var(--line);border-radius:9px;color:var(--ink);background:#0c1420;padding:9px 11px}}button{{cursor:pointer}}.table-wrap{{overflow:auto;border:1px solid var(--line);border-radius:15px;background:var(--panel);box-shadow:var(--shadow)}}table{{width:100%;min-width:950px;border-collapse:collapse}}th{{position:sticky;top:0;padding:12px 14px;background:#0f1824;color:var(--muted);text-align:left;font-size:11px;text-transform:uppercase;letter-spacing:.08em}}td{{padding:13px 14px;border-top:1px solid #223149;vertical-align:top}}tr:hover td{{background:#172337}}.name{{font-weight:750}}.target{{color:var(--muted);font:11px ui-monospace,monospace}}.time{{font:700 14px ui-monospace,monospace}}.delta{{font:12px ui-monospace,monospace}}.slower{{color:#ffad73}}.faster{{color:var(--pass)}}.status{{display:inline-block;border-radius:99px;padding:3px 8px;color:var(--tone);border:1px solid color-mix(in srgb,var(--tone) 45%,var(--line));font-size:11px;font-weight:750;text-transform:uppercase}}.spark{{width:150px;height:28px}}.empty{{padding:32px;text-align:center;color:var(--muted)}}@media(max-width:750px){{.hero{{display:block}}.metrics{{grid-template-columns:repeat(2,1fr)}}.filters{{grid-template-columns:1fr}}}}
</style></head><body><main class="shell"><header class="hero"><div><div class="eyebrow">Terrane test observatory</div><h1>{title}</h1><div class="lede">{description}</div></div><div class="stamp" id="stamp"></div></header><section class="metrics" id="metrics"></section><section class="filters"><input id="search" type="search" placeholder="Search test or target…"><select id="status"><option value="">All statuses</option><option>passed</option><option>failed</option><option>ignored</option></select><select id="sort"><option value="slow">Slowest first</option><option value="delta">Largest regression</option><option value="name">Name</option></select><button id="reset">Reset</button></section><div class="table-wrap"><table><thead><tr><th>Test</th><th>Status</th><th>Latest</th><th>Change</th><th>Recent samples</th><th>Measured</th></tr></thead><tbody id="rows"></tbody></table><div class="empty" id="empty" hidden>No tests match these filters.</div></div></main>
<script id="scoreboard-data" type="application/json">{payload}</script><script>
const data=JSON.parse(document.getElementById('scoreboard-data').textContent),search=document.getElementById('search'),status=document.getElementById('status'),sort=document.getElementById('sort'),rows=document.getElementById('rows'),tones={{passed:'var(--pass)',failed:'var(--fail)',ignored:'var(--skip)'}};const seconds=n=>n<1?`${{(n*1000).toFixed(n<.01?2:1)}} ms`:`${{n.toFixed(3)}} s`;const delta=t=>t.previous_seconds==null?null:t.seconds-t.previous_seconds;function spark(history){{const values=history.map(x=>x.seconds),max=Math.max(...values,.000001),points=values.map((v,i)=>`${{values.length===1?75:i*150/(values.length-1)}},${{27-v/max*25}}`).join(' ');return `<svg class="spark" viewBox="0 0 150 28" aria-label="timing history"><polyline fill="none" stroke="var(--accent)" stroke-width="2" points="${{points}}"/></svg>`}}function visible(){{const q=search.value.trim().toLowerCase();return data.tests.filter(t=>(!q||`${{t.name}} ${{t.target}}`.toLowerCase().includes(q))&&(!status.value||t.status===status.value)).sort((a,b)=>sort.value==='name'?a.name.localeCompare(b.name):sort.value==='delta'?(delta(b)??-Infinity)-(delta(a)??-Infinity):b.seconds-a.seconds)}}function render(){{const tests=visible();rows.innerHTML=tests.map(t=>{{const d=delta(t),dc=d==null?'':d>0?'slower':'faster';return `<tr><td><div class="name">${{escapeHtml(t.name)}}</div><div class="target">${{escapeHtml(t.target)}}</div></td><td><span class="status" style="--tone:${{tones[t.status]}}">${{t.status}}</span></td><td class="time">${{seconds(t.seconds)}}</td><td class="delta ${{dc}}">${{d==null?'new':`${{d>0?'+':''}}${{seconds(d)}}`}}</td><td>${{spark(t.history)}}</td><td class="target">${{t.measured_at}}</td></tr>`}}).join('');document.getElementById('empty').hidden=tests.length!==0;const latest=data.runs.at(-1),slowest=tests[0]?.seconds||0;document.getElementById('stamp').textContent=latest?`Latest run ${{latest.measured_at}}`:'';document.getElementById('metrics').innerHTML=[[data.tests.length,'Tracked tests'],[latest?.measured_tests||0,'Latest run'],[seconds(latest?.test_seconds||0),'Serial test time'],[seconds(slowest),'Slowest visible']].map(([v,l])=>`<article class="metric"><strong>${{v}}</strong><span>${{l}}</span></article>`).join('')}}function escapeHtml(s){{const e=document.createElement('span');e.textContent=s;return e.innerHTML}}for(const c of [search,status,sort])c.addEventListener(c===search?'input':'change',render);document.getElementById('reset').addEventListener('click',()=>{{search.value='';status.value='';sort.value='slow';render()}});render();
</script></body></html>'''


def main() -> int:
    args = parse_args()
    try:
        data = validate(yaml.safe_load(args.input.read_text(encoding="utf-8")))
        rendered = render(data)
    except (OSError, ValueError, yaml.YAMLError) as error:
        print(f"test scoreboard generation failed: {error}", file=sys.stderr)
        return 2
    if args.check:
        try:
            current = args.output.read_text(encoding="utf-8")
        except FileNotFoundError:
            print(f"scoreboard is missing: {args.output}", file=sys.stderr)
            return 1
        if current != rendered:
            print(f"scoreboard is stale: run {Path(__file__).name}", file=sys.stderr)
            return 1
        print(f"scoreboard is current: {args.output}")
        return 0
    args.output.write_text(rendered, encoding="utf-8")
    print(f"wrote {args.output} ({len(data['tests'])} tests)")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
