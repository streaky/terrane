#!/usr/bin/env python3
# Generate the self-contained Terrane language scoreboard HTML.

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

STATUS_ORDER = ("partial", "planned", "potential", "deferred", "excluded", "done")
REFERENCE_STATES = {"documented", "partial", "planned", "not-applicable"}


def parse_args() -> argparse.Namespace:
    script_dir = Path(__file__).resolve().parent
    parser = argparse.ArgumentParser(description="Generate the Terrane language scoreboard HTML.")
    parser.add_argument("--input", type=Path, default=script_dir / "language-scoreboard.yaml")
    parser.add_argument("--output", type=Path, default=script_dir / "language-scoreboard.html")
    parser.add_argument("--check", action="store_true", help="fail if --output differs instead of writing it")
    return parser.parse_args()


def require(condition: bool, message: str) -> None:
    if not condition:
        raise ValueError(message)


def validate(data: Any, source_path: Path) -> dict[str, Any]:
    require(isinstance(data, dict), "scoreboard root must be a mapping")
    require(data.get("format") == 1, "scoreboard format must be 1")
    require(isinstance(data.get("title"), str) and data["title"], "title must be non-empty")
    metadata, statuses = data.get("metadata"), data.get("statuses")
    categories, features = data.get("categories"), data.get("features")
    require(isinstance(metadata, dict), "metadata must be a mapping")
    require(isinstance(statuses, dict) and statuses, "statuses must be a non-empty mapping")
    require(isinstance(categories, list) and categories, "categories must be a non-empty list")
    require(isinstance(features, list) and features, "features must be a non-empty list")
    require(set(statuses) == set(STATUS_ORDER), f"statuses must be exactly {', '.join(STATUS_ORDER)}")

    category_ids: set[str] = set()
    for index, category in enumerate(categories):
        prefix = f"categories[{index}]"
        require(isinstance(category, dict), f"{prefix} must be a mapping")
        category_id = category.get("id")
        require(isinstance(category_id, str) and category_id, f"{prefix}.id must be non-empty")
        require(category_id not in category_ids, f"duplicate category id: {category_id}")
        category_ids.add(category_id)
        require(isinstance(category.get("label"), str) and category["label"], f"{prefix}.label must be non-empty")
        require(isinstance(category.get("description"), str), f"{prefix}.description must be a string")

    feature_ids: set[str] = set()
    seen_categories: set[str] = set()
    repo_root = source_path.resolve().parent.parent
    for index, feature in enumerate(features):
        prefix = f"features[{index}]"
        require(isinstance(feature, dict), f"{prefix} must be a mapping")
        feature_id = feature.get("id")
        require(isinstance(feature_id, str) and feature_id, f"{prefix}.id must be non-empty")
        require(feature_id not in feature_ids, f"duplicate feature id: {feature_id}")
        feature_ids.add(feature_id)
        require(isinstance(feature.get("name"), str) and feature["name"], f"{prefix}.name must be non-empty")
        category = feature.get("category")
        require(category in category_ids, f"{prefix}.category is unknown: {category!r}")
        seen_categories.add(category)
        status = feature.get("status")
        require(status in statuses, f"{prefix}.status is unknown: {status!r}")
        completion = feature.get("completion")
        require(type(completion) is int and 0 <= completion <= 100, f"{prefix}.completion must be an integer from 0 to 100")
        if status == "done":
            require(completion == 100, f"{prefix}: done features must be 100%")
        elif status == "partial":
            require(0 < completion < 100, f"{prefix}: partial features must be between 1% and 99%")
        else:
            require(completion == 0, f"{prefix}: {status} features must be 0%")
        milestone = feature.get("milestone")
        require(milestone is None or isinstance(milestone, str), f"{prefix}.milestone must be a string or null")
        if status == "planned":
            require(bool(milestone), f"{prefix}: planned features require a milestone")
        reference = feature.get("reference")
        require(isinstance(reference, dict), f"{prefix}.reference must be a mapping")
        reference_state, paths = reference.get("state"), reference.get("paths")
        require(reference_state in REFERENCE_STATES, f"{prefix}.reference.state is unknown: {reference_state!r}")
        require(isinstance(paths, list) and all(isinstance(path, str) and path for path in paths), f"{prefix}.reference.paths must be strings")
        if reference_state == "not-applicable":
            require(not paths, f"{prefix}: not-applicable references cannot list paths")
        else:
            require(bool(paths), f"{prefix}: reference paths are required")
        if reference_state in {"documented", "partial"}:
            for path in paths:
                require((repo_root / path).is_file(), f"{prefix}: documented reference does not exist: {path}")
        require(isinstance(feature.get("notes"), str) and feature["notes"], f"{prefix}.notes must be non-empty")
        sources = feature.get("sources")
        require(isinstance(sources, list) and sources, f"{prefix}.sources must be a non-empty list")
        for source in sources:
            require(isinstance(source, str) and source, f"{prefix}.sources must contain strings")
            file_part = source.split("#", 1)[0]
            require((repo_root / file_part).is_file(), f"{prefix}: source file does not exist: {file_part}")

    require(seen_categories == category_ids, "every category must contain at least one feature")
    return data


def render(data: dict[str, Any]) -> str:
    title = html.escape(data["title"])
    description = html.escape(data["metadata"]["description"])
    as_of = html.escape(str(data["metadata"]["as_of"]))
    payload = json.dumps(data, ensure_ascii=False, separators=(",", ":")).replace("</", "<\\/")
    return f'''<!doctype html>
<!-- Generated from language-scoreboard.yaml by generate-language-scoreboard.py; do not edit directly. -->
<html lang="en">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<title>{title}</title>
<style>
:root {{ color-scheme: dark; --bg:#08100f; --panel:#101b19; --panel2:#152321; --line:#28413c; --ink:#f2f7f5; --muted:#98aaa5; --accent:#73e0b1; --done:#4bd2a0; --partial:#f2b84b; --planned:#78a9ff; --potential:#b69cff; --deferred:#85949f; --excluded:#e47777; --shadow:0 22px 70px #0007; }}
* {{ box-sizing:border-box }} body {{ margin:0; background:radial-gradient(circle at 12% 0,#173029 0,transparent 34rem),var(--bg); color:var(--ink); font:15px/1.45 Inter,ui-sans-serif,system-ui,sans-serif }} button,input,select {{ font:inherit }} a {{ color:inherit }}
.shell {{ max-width:1680px; margin:auto; padding:34px clamp(18px,4vw,64px) 70px }} .hero {{ display:grid; grid-template-columns:minmax(0,1fr) auto; gap:28px; align-items:end; margin-bottom:28px }} .eyebrow {{ margin:0 0 8px; color:var(--accent); font-size:12px; font-weight:800; letter-spacing:.14em; text-transform:uppercase }} h1 {{ margin:0; max-width:900px; font:700 clamp(36px,6vw,72px)/.97 Georgia,serif; letter-spacing:-.045em }} .lede {{ max-width:760px; margin:18px 0 0; color:var(--muted); font-size:17px }} .asof {{ color:var(--muted); white-space:nowrap; border:1px solid var(--line); border-radius:99px; padding:9px 13px }}
.metrics {{ display:grid; grid-template-columns:repeat(4,1fr); gap:12px; margin-bottom:26px }} .metric {{ padding:18px; background:linear-gradient(145deg,#15241f,#0f1817); border:1px solid var(--line); border-radius:16px; box-shadow:var(--shadow) }} .metric strong {{ display:block; font-size:30px; letter-spacing:-.04em }} .metric span {{ color:var(--muted); font-size:12px; text-transform:uppercase; letter-spacing:.08em }}
.layout {{ display:grid; grid-template-columns:285px minmax(0,1fr); gap:22px; align-items:start }} .filters {{ position:sticky; top:16px; padding:18px; border:1px solid var(--line); border-radius:16px; background:#0e1917ee; backdrop-filter:blur(14px) }} .filters h2,.overview h2 {{ margin:0 0 14px; font-size:14px; text-transform:uppercase; letter-spacing:.08em }} .field {{ display:grid; gap:6px; margin:0 0 14px }} .field label {{ color:var(--muted); font-size:12px }} .field input,.field select {{ width:100%; border:1px solid var(--line); border-radius:9px; color:var(--ink); background:#091210; padding:10px 11px; outline:none }} .field input:focus,.field select:focus {{ border-color:var(--accent); box-shadow:0 0 0 3px #73e0b122 }} .reset {{ width:100%; border:1px solid var(--line); border-radius:9px; padding:9px; color:var(--ink); background:var(--panel2); cursor:pointer }}
.legend {{ display:flex; flex-wrap:wrap; gap:7px; margin-top:18px }} .chip {{ display:inline-flex; align-items:center; gap:6px; border:1px solid color-mix(in srgb,var(--tone) 42%,var(--line)); border-radius:99px; padding:4px 8px; color:var(--tone); background:color-mix(in srgb,var(--tone) 10%,transparent); font-size:11px; font-weight:750; text-transform:uppercase; letter-spacing:.04em }} .chip::before {{ content:''; width:6px; height:6px; border-radius:50%; background:var(--tone) }}
.content {{ min-width:0 }} .overview {{ padding:18px; border:1px solid var(--line); border-radius:16px; background:var(--panel); margin-bottom:18px }} .category-grid {{ display:grid; grid-template-columns:repeat(auto-fit,minmax(190px,1fr)); gap:10px }} .category-card {{ border:1px solid var(--line); border-radius:12px; background:#0b1513; padding:12px; cursor:pointer; text-align:left; color:var(--ink) }} .category-card:hover {{ border-color:var(--accent) }} .category-card strong {{ display:flex; justify-content:space-between; gap:8px; margin-bottom:8px }} .bar {{ height:6px; overflow:hidden; border-radius:99px; background:#263531 }} .bar>i {{ display:block; height:100%; background:linear-gradient(90deg,var(--accent),#9ee7ff) }} .category-card small {{ display:block; margin-top:7px; color:var(--muted) }}
.toolbar {{ display:flex; justify-content:space-between; gap:12px; align-items:center; margin:0 2px 10px; color:var(--muted) }} .toolbar select {{ color:var(--ink); background:var(--panel); border:1px solid var(--line); border-radius:8px; padding:7px 9px }} .table-wrap {{ overflow:auto; border:1px solid var(--line); border-radius:16px; background:var(--panel); box-shadow:var(--shadow) }} table {{ width:100%; min-width:1050px; border-collapse:collapse }} th {{ position:sticky; top:0; z-index:1; padding:13px 14px; color:var(--muted); background:#0d1715; border-bottom:1px solid var(--line); text-align:left; font-size:11px; text-transform:uppercase; letter-spacing:.08em }} td {{ padding:14px; border-bottom:1px solid #223733; vertical-align:top }} tr:last-child td {{ border-bottom:0 }} tbody tr:hover {{ background:#172724 }}
.feature-name {{ min-width:210px; font-weight:750 }} .feature-id {{ display:block; margin-top:4px; color:var(--muted); font:11px ui-monospace,monospace }} .progress {{ display:grid; grid-template-columns:minmax(80px,1fr) 35px; gap:8px; align-items:center; min-width:135px }} .progress b {{ text-align:right; font-size:12px }} .milestone {{ font:12px ui-monospace,monospace; white-space:nowrap }} .note {{ min-width:270px; color:#c8d4d1 }} .refs {{ min-width:240px }} .ref-state {{ display:block; margin-bottom:5px; color:var(--muted); font-size:11px; text-transform:uppercase }} .path-link,.source-link {{ display:block; width:fit-content; max-width:360px; overflow-wrap:anywhere; color:#b6d8ce; font:11px/1.45 ui-monospace,monospace; text-decoration:none }} .path-link:hover,.source-link:hover {{ color:var(--accent); text-decoration:underline }} .empty {{ padding:52px 20px; text-align:center; color:var(--muted) }}
@media(max-width:900px) {{ .hero {{ grid-template-columns:1fr }} .asof {{ width:fit-content }} .metrics {{ grid-template-columns:repeat(2,1fr) }} .layout {{ grid-template-columns:1fr }} .filters {{ position:static; display:grid; grid-template-columns:repeat(2,1fr); gap:10px }} .filters h2,.legend,.reset {{ grid-column:1/-1 }} .field {{ margin:0 }} }} @media(max-width:560px) {{ .filters {{ grid-template-columns:1fr }} .filters h2,.legend,.reset {{ grid-column:auto }} }} @media print {{ body {{ background:white; color:#111 }} .shell {{ max-width:none; padding:10px }} .filters,.overview,.toolbar {{ display:none }} .table-wrap {{ overflow:visible; box-shadow:none }} table {{ min-width:0; font-size:9px }} th {{ position:static }} }}
</style>
</head>
<body>
<main class="shell">
<header class="hero"><div><p class="eyebrow">Terrane progress atlas</p><h1>{title}</h1><p class="lede">{description}</p></div><div class="asof">Status as of {as_of}</div></header>
<section class="metrics" id="metrics" aria-label="Scoreboard summary"></section>
<div class="layout"><aside class="filters"><h2>Explore</h2><div class="field"><label for="search">Search</label><input id="search" type="search" placeholder="Feature, note, source…"></div><div class="field"><label for="category">Category</label><select id="category"></select></div><div class="field"><label for="status">Status</label><select id="status"></select></div><div class="field"><label for="milestone">Milestone</label><select id="milestone"></select></div><div class="field"><label for="reference">Reference</label><select id="reference"></select></div><button class="reset" id="reset" type="button">Reset filters</button><div class="legend" id="legend"></div></aside>
<section class="content"><div class="overview"><h2>Category completion</h2><div class="category-grid" id="category-grid"></div></div><div class="toolbar"><span id="result-count"></span><label>Sort <select id="sort"><option value="roadmap">Roadmap priority</option><option value="category">Category</option><option value="name">Name</option><option value="completion">Completion</option></select></label></div><div class="table-wrap"><table><thead><tr><th>Feature</th><th>Status</th><th>Progress</th><th>Milestone</th><th>Reference</th><th>Notes</th><th>Sources</th></tr></thead><tbody id="rows"></tbody></table><div class="empty" id="empty" hidden>No features match these filters.</div></div></section></div>
</main>
<script id="scoreboard-data" type="application/json">{payload}</script>
<script>
const data=JSON.parse(document.getElementById('scoreboard-data').textContent),byCategory=new Map(data.categories.map((x,i)=>[x.id,{{...x,index:i}}])),statusOrder=new Map(['partial','planned','potential','deferred','excluded','done'].map((x,i)=>[x,i])),tones={{done:'var(--done)',partial:'var(--partial)',planned:'var(--planned)',potential:'var(--potential)',deferred:'var(--deferred)',excluded:'var(--excluded)'}},controls=Object.fromEntries(['search','category','status','milestone','reference','sort'].map(id=>[id,document.getElementById(id)]));
const repoHref=p=>'../'+p.split('#')[0]+(p.includes('#')?'#'+encodeURIComponent(p.split('#').slice(1).join('#')):''),average=a=>a.length?Math.round(a.reduce((s,x)=>s+x.completion,0)/a.length):0,fill=(s,l,v,f=x=>x)=>s.innerHTML=`<option value="">All ${{l}}</option>`+v.map(x=>`<option value="${{x}}">${{f(x)}}</option>`).join('');
fill(controls.category,'categories',data.categories.map(x=>x.id),x=>byCategory.get(x).label); fill(controls.status,'statuses',Object.keys(data.statuses),x=>data.statuses[x].label); fill(controls.milestone,'milestones',[...new Set(data.features.map(x=>x.milestone).filter(Boolean))].sort((a,b)=>a.localeCompare(b,undefined,{{numeric:true}})),x=>`Milestone ${{x}}`); fill(controls.reference,'reference states',Object.keys(data.reference_states),x=>x.replace('-',' '));
document.getElementById('legend').innerHTML=Object.entries(data.statuses).map(([id,s])=>`<span class="chip" style="--tone:${{tones[id]}}" title="${{s.description}}">${{s.label}}</span>`).join('');
function matches(x){{const q=controls.search.value.trim().toLowerCase(),hay=[x.id,x.name,x.notes,x.milestone||'',...x.reference.paths,...x.sources].join(' ').toLowerCase();return(!q||hay.includes(q))&&(!controls.category.value||x.category===controls.category.value)&&(!controls.status.value||x.status===controls.status.value)&&(!controls.milestone.value||x.milestone===controls.milestone.value)&&(!controls.reference.value||x.reference.state===controls.reference.value)}}
function compare(a,b){{switch(controls.sort.value){{case'name':return a.name.localeCompare(b.name);case'completion':return a.completion-b.completion||a.name.localeCompare(b.name);case'category':return byCategory.get(a.category).index-byCategory.get(b.category).index||a.name.localeCompare(b.name);default:return statusOrder.get(a.status)-statusOrder.get(b.status)||byCategory.get(a.category).index-byCategory.get(b.category).index||a.name.localeCompare(b.name)}}}}
function link(path,cls){{const a=document.createElement('a');a.className=cls;a.href=repoHref(path);a.textContent=path;return a}}
function row(x){{const r=document.createElement('tr'),n=r.insertCell();n.className='feature-name';n.textContent=x.name;const id=document.createElement('span');id.className='feature-id';id.textContent=`${{byCategory.get(x.category).label}} · ${{x.id}}`;n.append(id);const st=r.insertCell(),chip=document.createElement('span');chip.className='chip';chip.style.setProperty('--tone',tones[x.status]);chip.textContent=data.statuses[x.status].label;chip.title=data.statuses[x.status].description;st.append(chip);const pc=r.insertCell(),p=document.createElement('div');p.className='progress';p.innerHTML=`<div class="bar"><i style="width:${{x.completion}}%"></i></div><b>${{x.completion}}%</b>`;pc.append(p);const m=r.insertCell();m.className='milestone';m.textContent=x.milestone?`M${{x.milestone}}`:'—';const refs=r.insertCell();refs.className='refs';const rs=document.createElement('span');rs.className='ref-state';rs.textContent=x.reference.state.replace('-',' ');refs.append(rs);x.reference.paths.forEach(y=>refs.append(link(y,'path-link')));const note=r.insertCell();note.className='note';note.textContent=x.notes;const src=r.insertCell();x.sources.forEach(y=>src.append(link(y,'source-link')));return r}}
function renderCategories(){{const target=document.getElementById('category-grid');target.replaceChildren();for(const c of data.categories){{const items=data.features.filter(x=>x.category===c.id),b=document.createElement('button');b.type='button';b.className='category-card';b.title=c.description;b.innerHTML=`<strong><span>${{c.label}}</span><span>${{average(items)}}%</span></strong><div class="bar"><i style="width:${{average(items)}}%"></i></div><small>${{items.filter(x=>x.status==='done').length}} done · ${{items.length}} tracked</small>`;b.addEventListener('click',()=>{{controls.category.value=c.id;render()}});target.append(b)}}}}
function render(){{const visible=data.features.filter(matches).sort(compare),rows=document.getElementById('rows');rows.replaceChildren(...visible.map(row));document.getElementById('empty').hidden=visible.length!==0;document.getElementById('result-count').textContent=`${{visible.length}} of ${{data.features.length}} features`;const done=visible.filter(x=>x.status==='done').length,active=visible.filter(x=>x.status==='partial'||x.status==='planned').length;document.getElementById('metrics').innerHTML=[[visible.length,'Visible features'],[done,'Done'],[active,'Active roadmap'],[average(visible)+'%','Average completion']].map(([v,l])=>`<article class="metric"><strong>${{v}}</strong><span>${{l}}</span></article>`).join('')}}
for(const control of Object.values(controls))control.addEventListener(control===controls.search?'input':'change',render);document.getElementById('reset').addEventListener('click',()=>{{for(const[id,c]of Object.entries(controls))c.value=id==='sort'?'roadmap':'';render()}});renderCategories();render();
</script>
</body>
</html>
'''


def main() -> int:
    args = parse_args()
    try:
        with args.input.open("r", encoding="utf-8") as source:
            data = validate(yaml.safe_load(source), args.input)
        rendered = render(data)
    except (OSError, ValueError, yaml.YAMLError) as error:
        print(f"scoreboard generation failed: {error}", file=sys.stderr)
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
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(rendered, encoding="utf-8")
    print(f"wrote {args.output} ({len(data['features'])} features)")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
