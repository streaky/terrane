'use strict';
const test = require('node:test');
const assert = require('node:assert/strict');
const fs = require('node:fs/promises');
const os = require('node:os');
const path = require('node:path');
const { registerBuildView } = require('../build-view');

async function harness(t) {
  const dir = await fs.mkdtemp(path.join(os.tmpdir(), 'terrane-view-'));
  t.after(() => fs.rm(dir, { recursive: true, force: true }));
  const file = path.join(dir, 'entry.trn'), rust = path.join(dir, 'main.rs'), compiler = path.join(dir, 'compiler');
  await fs.writeFile(file, '');
  await fs.writeFile(rust, 'line one\n<script>line two</script>\nlast\n');
  await fs.writeFile(compiler, `#!${process.execPath}
const [, , , , program, provenance, source, rawLine, ...options] = process.argv;
const relocation = options.indexOf('--relocation');
function fail(detail) { console.error('<debugger>: error[S5001]: ' + detail); process.exit(5); }
if (source.endsWith('/outside.trn') && program !== source) fail(source + ' is not in this debug build');
if (source.endsWith('/stale.trn') && program !== source) fail('source has changed or is unavailable in this debug build: ' + source);
if (source.endsWith('/no-tools.trn') && !options.includes('--rust-only')) fail('cannot run GNU objdump for assembly view');
let file = ${JSON.stringify(rust)};
if (program === '/relocated-binary') {
  const roots = JSON.parse(options[relocation + 1]);
  file = require('node:path').join(roots.buildRoot, 'relocated.rs');
  if (options[0] !== file || options[1] !== rawLine || source !== ${JSON.stringify(file)} ||
      roots.sourceRoot !== ${JSON.stringify(dir)} || provenance !== '/relocated-sidecar') process.exit(1);
}
console.log(JSON.stringify({rust:[{path:file,line:Number(rawLine)}],assembly:[{address:'0x10',instruction:'ret'}]}));`, { mode: 0o755 });
  let open, toggle, changed, saved, clicked, activated, terminated, stackChanged, disposed, notify;
  const result = () => new Promise((resolve, reject) => {
    const timer = setTimeout(() => reject(new Error('No build view update')), 3000);
    notify = html => { clearTimeout(timer); resolve(html); };
  });
  const document = { uri: { fsPath: file, scheme: 'file' }, languageId: 'terrane', isDirty: false };
  const editor = { document, selection: { active: { line: 1 } }, viewColumn: 1 };
  const panel = { webview: { set html(value) { if (!value.includes('Building saved Terrane')) notify?.(value); } }, reveal() {}, onDidDispose(fn) { disposed = fn; }, dispose() { disposed(); } };
  const api = {
    window: {
      activeTextEditor: editor, createWebviewPanel: () => panel,
      showTextDocument: async () => editor,
      onDidChangeTextEditorSelection: fn => { clicked = fn; },
      onDidChangeActiveTextEditor: fn => { activated = fn; },
    },
    workspace: {
      isTrusted: true, getWorkspaceFolder: () => undefined,
      onDidChangeTextDocument: fn => { changed = fn; },
      onDidSaveTextDocument: fn => { saved = fn; },
      openTextDocument: async () => document,
    },
    debug: {
      onDidChangeActiveStackItem(fn) { stackChanged = fn; },
      onDidTerminateDebugSession(fn) { terminated = fn; },
    },
    commands: { registerCommand: (name, fn) => {
      if (name === 'terrane.showBuildView') open = fn; else toggle = fn;
    } },
    ViewColumn: { Beside: 2 }, TextEditorSelectionChangeKind: { Command: 3 },
    TextEditorRevealType: { InCenterIfOutsideViewport: 0 },
    Uri: { file: fsPath => ({ fsPath }) },
    Position: class { constructor(line) { this.line = line; } },
    Selection: class { constructor(pos) { this.active = pos; } },
    CancellationTokenSource: class {
      token = {};
      cancel() { this.token.onCancel?.(); }
      dispose() {}
    },
  };
  editor.revealRange = () => {};
  const builds = [];
  const build = { command: compiler, program: '/binary', terraneProvenance: '/sidecar' };
  const hooks = {};
  registerBuildView(api, { subscriptions: [] }, async (_, source, token) => {
    builds.push(source);
    await hooks.build?.(source, token);
    return { ...build, program: source };
  });
  t.after(() => { disposed?.(); });
  return { dir, rust, file, compiler, document, editor, api, builds, hooks, result,
    open: () => open(), toggle: () => toggle(), changed: event => changed(event),
    saved: doc => saved(doc), clicked: event => clicked(event), activated: ed => activated(ed),
    terminated: session => terminated(session), stackChanged: item => stackChanged(item) };
}

test('source view rebuilds on save, reuses an artifact across covered files, and builds uncovered sources', async t => {
  const { dir, rust, file, document, editor, builds, result, open, toggle, changed, clicked, saved, activated } = await harness(t);
  let next = result(); await open();
  const initial = await next;
  assert.match(initial, /<mark>\s*2  &lt;script&gt;line two&lt;\/script&gt;<\/mark>/);
  assert.match(initial, /Show Assembly/);
  assert.doesNotMatch(initial, /0x10  ret/);
  next = result(); toggle();
  assert.match(await next, /0x10  ret/);
  next = result(); toggle();
  assert.doesNotMatch(await next, /Assembly · build addresses/);
  next = result(); editor.selection.active.line = 0; clicked({ textEditor: editor, kind: 2 });
  assert.match(await next, /<mark>\s*1  line one<\/mark>/);
  next = result(); document.isDirty = true; changed({ document });
  const dirty = await next; assert.match(dirty, /Save this source/); assert.doesNotMatch(dirty, /0x10/);
  await fs.writeFile(rust, 'regenerated Rust\n');
  next = result(); document.isDirty = false; saved(document);
  assert.match(await next, /<mark>\s*1  regenerated Rust<\/mark>/);
  const other = path.join(dir, 'other.trn');
  next = result(); activated({ ...editor, document: { ...document, uri: { ...document.uri, fsPath: other } } });
  assert.match(await next, /other\.trn:1/);
  next = result(); activated(editor);
  assert.match(await next, /entry\.trn:1/);
  assert.deepEqual(builds, [file, file]);
  const outside = path.join(dir, 'outside.trn');
  next = result(); activated({ ...editor, document: { ...document, uri: { ...document.uri, fsPath: outside } } });
  assert.match(await next, /outside\.trn:1/);
  assert.deepEqual(builds, [file, file, outside]);
  const stale = path.join(dir, 'stale.trn');
  next = result(); activated({ ...editor, document: { ...document, uri: { ...document.uri, fsPath: stale } } });
  assert.match(await next, /stale\.trn:1/);
  assert.deepEqual(builds, [file, file, outside, stale]);
});

test('an unavailable disassembler reports the query error without rebuilding sibling files', async t => {
  const { dir, file, document, editor, builds, result, open, toggle, activated } = await harness(t);
  let next = result(); open(); await next;
  next = result(); toggle(); await next;
  const sibling = { ...editor, document: { ...document, uri: { ...document.uri, fsPath: path.join(dir, 'no-tools.trn') } } };
  next = result(); activated(sibling);
  assert.match(await next, /cannot run GNU objdump/);
  next = result(); activated(editor);
  assert.match(await next, /entry\.trn:2/);
  assert.deepEqual(builds, [file]);
});

test('failed builds are retained across passive updates but retry on invalidation', async t => {
  const { dir, file, document, editor, builds, hooks, result, open, toggle, clicked, saved, activated } = await harness(t);
  hooks.build = async () => { throw new Error('Compiler rejected source'); };
  const expectError = async action => {
    const next = result(); action(); assert.match(await next, /Compiler rejected source/);
  };
  await expectError(open);
  await expectError(() => clicked({ textEditor: editor, kind: 2 }));
  await expectError(() => activated(editor));
  await expectError(toggle);
  assert.deepEqual(builds, [file]);
  await expectError(() => saved(document));
  await expectError(open);
  const other = path.join(dir, 'other.trn');
  await expectError(() => activated({ ...editor, document: { ...document, uri: { ...document.uri, fsPath: other } } }));
  assert.deepEqual(builds, [file, file, file, other]);
});

test('switching source cancels a pending build without displaying its error', async t => {
  const { dir, file, document, editor, builds, hooks, result, open, activated } = await harness(t);
  let markStarted;
  const started = new Promise(resolve => { markStarted = resolve; });
  let cancelled = false;
  hooks.build = (source, token) => source === file ? new Promise((_, reject) => {
    token.onCancel = () => { cancelled = true; reject(new Error('Superseded build')); };
    markStarted();
  }) : undefined;
  open();
  await started;
  const other = path.join(dir, 'other.trn');
  const next = result();
  activated({ ...editor, document: { ...document, uri: { ...document.uri, fsPath: other } } });
  assert.match(await next, /other\.trn:2/);
  assert.equal(cancelled, true);
  assert.deepEqual(builds, [file, other]);
});

test('view uses relocated session roots and builds on demand after the session ends', async t => {
  const { dir, file, compiler, api, builds, result, open, terminated, stackChanged } = await harness(t);
  const relocated = path.join(dir, 'relocated.rs');
  await fs.writeFile(relocated, 'relocated Rust\n');
  const session = {
    id: 'session', type: 'terrane',
    configuration: {
      __terraneCompiler: compiler, program: '/relocated-binary', terraneProvenance: '/relocated-sidecar',
      terraneRelocation: { buildRoot: dir, sourceRoot: dir },
    },
    async customRequest() {
      return { stackFrames: [{ id: 17, source: { path: file }, line: 1, terraneGenerated: { path: relocated, line: 1 } }] };
    },
  };
  api.debug.activeDebugSession = session;
  api.debug.activeStackItem = { session, threadId: 1, frameId: 17 };
  let next = result(); open();
  const live = await next;
  assert.match(live, /relocated\.rs/);
  assert.match(live, /<mark>\s*1  relocated Rust<\/mark>/);
  assert.deepEqual(builds, []);

  next = result();
  terminated(session);
  // VS Code may clear the active stack item after delivering the termination event.
  stackChanged(undefined);
  assert.match(await next, /<mark>\s*1  line one<\/mark>/);
  assert.deepEqual(builds, [file]);
});
