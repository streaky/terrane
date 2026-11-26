'use strict';
const test = require('node:test');
const assert = require('node:assert/strict');
const fs = require('node:fs/promises');
const os = require('node:os');
const path = require('node:path');
const { registerBuildView } = require('../build-view');

test('source view follows saved selections, escapes generated text and clears on edits', async t => {
  const dir = await fs.mkdtemp(path.join(os.tmpdir(), 'terrane-view-'));
  t.after(() => fs.rm(dir, { recursive: true, force: true }));
  const file = path.join(dir, 'entry.trn'), rust = path.join(dir, 'main.rs'), compiler = path.join(dir, 'compiler');
  await fs.writeFile(file, '');
  await fs.writeFile(rust, 'line one\n<script>line two</script>\nlast\n');
  await fs.writeFile(compiler, `#!${process.execPath}\nconst line=Number(process.argv[7]); console.log(JSON.stringify({rust:[{path:${JSON.stringify(rust)},line}],assembly:[{address:'0x10',instruction:'ret'}]}));`, { mode: 0o755 });
  let open, toggle, changed, clicked, disposed, notify;
  const result = () => new Promise((resolve, reject) => {
    const timer = setTimeout(() => reject(new Error('No build view update')), 3000);
    notify = html => { clearTimeout(timer); resolve(html); };
  });
  const document = { uri: { fsPath: file, scheme: 'file' }, languageId: 'terrane', isDirty: false };
  const editor = { document, selection: { active: { line: 1 } }, viewColumn: 1 };
  const panel = { webview: { set html(value) { if (!value.includes('Building saved Terrane')) notify?.(value); } }, onDidDispose(fn) { disposed = fn; }, dispose() { disposed(); } };
  const api = {
    window: { activeTextEditor: editor, createWebviewPanel: () => panel, onDidChangeTextEditorSelection: fn => { clicked = fn; } },
    workspace: { isTrusted: true, getWorkspaceFolder: () => undefined, onDidChangeTextDocument: fn => { changed = fn; } },
    debug: { onDidChangeActiveStackItem() {} }, commands: { registerCommand: (name, fn) => {
      if (name === 'terrane.showBuildView') open = fn; else toggle = fn;
    } },
    ViewColumn: { Beside: 2 }, TextEditorSelectionChangeKind: { Command: 3 }, CancellationTokenSource: class { cancel() {} dispose() {} },
  };
  registerBuildView(api, { subscriptions: [] }, async () => ({ command: compiler, program: '/binary', terraneProvenance: '/sidecar' }));
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
  panel.dispose();
});
