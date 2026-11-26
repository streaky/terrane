'use strict';

const fs = require('node:fs/promises');
const path = require('node:path');
const exec = require('node:util').promisify(require('node:child_process').execFile);
const toggleAssembly = 'terrane.toggleBuildViewAssembly';

function escape(value) {
  return String(value).replace(/[&<>"']/g, c => ({ '&': '&amp;', '<': '&lt;', '>': '&gt;', '"': '&quot;', "'": '&#39;' })[c]);
}

// The editable source stays in its own editor. Both read-only columns are a single,
// script-free panel so updates never steal focus or replace an unrelated editor tab.
function page(label, rust, assembly, live) {
  return `<!doctype html><meta charset="utf-8"><meta http-equiv="Content-Security-Policy" content="default-src 'none'; style-src 'unsafe-inline'"><style>
body{margin:0}header,h2{padding:.6em 1em;margin:0;font:inherit}header{border-bottom:1px solid var(--vscode-panel-border)}main{display:grid;grid-template-columns:1fr 1fr}section{min-width:0}section+section{border-left:1px solid var(--vscode-panel-border)}h2{font-weight:bold}pre{margin:0;padding:.6em;overflow:auto;line-height:1.5;font-family:var(--vscode-editor-font-family)}mark{color:var(--vscode-editor-selectionForeground,inherit);background:var(--vscode-editor-selectionBackground)}small{display:block;padding:.5em;overflow-wrap:anywhere}
header{display:flex;align-items:center;justify-content:space-between;gap:1em}header a{background:var(--vscode-button-secondaryBackground);color:var(--vscode-button-secondaryForeground);padding:.25em .7em;text-decoration:none;border-radius:2px}main:has(section:only-child){grid-template-columns:1fr}
</style><header>${escape(label)}<a href="command:${toggleAssembly}">${assembly == null ? 'Show' : 'Hide'} Assembly</a></header><main><section><h2>Lowered Rust</h2>${rust}</section>${assembly == null ? '' : `<section><h2>${live ? 'Assembly · paused frame' : 'Assembly · build addresses'}</h2>${assembly}</section>`}</main>`;
}

async function rustExcerpt(locations) {
  const groups = new Map();
  for (const { path: file, line } of locations) {
    if (!groups.has(file)) groups.set(file, new Set());
    groups.get(file).add(line);
  }
  const result = [];
  for (const [file, selected] of groups) {
    const lines = (await fs.readFile(file, 'utf8')).split(/\r?\n/);
    const visible = new Set();
    for (const n of selected) {
      for (let i = Math.max(1, n - 3); i <= Math.min(lines.length, n + 3); i++) visible.add(i);
    }
    let previous = 0;
    const text = [...visible].sort((a, b) => a - b).map(i => {
      const row = `${i.toString().padStart(4)}  ${escape(lines[i - 1])}`;
      const gap = previous && i > previous + 1 ? '⋯\n' : '';
      previous = i;
      return gap + (selected.has(i) ? `<mark>${row}</mark>` : row);
    }).join('\n');
    result.push(`<small>${escape(file)}</small><pre>${text}</pre>`);
  }
  return result.join('') || '<small>No lowering mapped to this line.</small>';
}


function registerBuildView(vscode, context, build) {
  let panel, sourceColumn, artifact, cancellation, query, timer, revision = 0;
  let showAssembly = false, selection;
  function invalidate() {
    clearTimeout(timer);
    query?.abort();
    return ++revision;
  }
  function reset() {
    invalidate();
    cancellation?.cancel(); cancellation?.dispose(); cancellation = undefined;
    artifact = undefined;
  }
  const message = text => { if (panel) panel.webview.html = page(text, '', showAssembly ? '' : null, false); };
  const terrane = editor => editor?.document.languageId === 'terrane' && editor.document.uri.scheme === 'file';
  function schedule(editor, item, delay = 0) {
    if (!panel) return;
    selection = { editor, item };
    const id = invalidate();
    timer = setTimeout(() => update(id, editor, item).catch(error => {
      if (id === revision) message(error.stderr?.trim() || error.message);
    }), delay);
  }
  async function update(id, editor, item) {
    if (!vscode.workspace.isTrusted) throw new Error('Trust this workspace to inspect native code.');
    const session = item?.session?.type === 'terrane' ? item.session : vscode.debug.activeDebugSession;
    let frame;
    if (item && session?.type === 'terrane') {
      const { stackFrames } = await session.customRequest('stackTrace', { threadId: item.threadId, ...(item.frameId === undefined ? { levels: 1 } : {}) });
      frame = item.frameId === undefined ? stackFrames[0] : stackFrames.find(f => f.id === item.frameId);
      if (id !== revision) return;
      if (!frame?.terraneGenerated || !frame.source?.path) throw new Error('This frame has no Terrane source mapping.');
      const doc = await vscode.workspace.openTextDocument(vscode.Uri.file(frame.source.path));
      if (id !== revision) return;
      editor = await vscode.window.showTextDocument(doc, { viewColumn: sourceColumn, preserveFocus: true, preview: false });
      if (id !== revision) return;
      const pos = new vscode.Position(frame.line - 1, 0);
      editor.selection = new vscode.Selection(pos, pos);
      editor.revealRange(editor.selection, vscode.TextEditorRevealType.InCenterIfOutsideViewport);
    }
    if (!terrane(editor)) throw new Error('Select a line in a saved Terrane source file.');
    if (editor.document.isDirty) throw new Error('Save this source before comparing it with a debug build.');
    const file = editor.document.uri.fsPath;
    const line = frame?.line ?? editor.selection.active.line + 1;
    const config = session?.type === 'terrane' ? session.configuration : undefined;
    const built = config ? { ...config, command: config.__terraneCompiler } : await artifact;
    if (id !== revision) return;
    if (!built?.command) throw new Error('Open the build view again to build this source.');
    const controller = query = new AbortController();
    const args = ['debug', '--view-json', built.program, built.terraneProvenance, file, String(line)];
    if (frame) args.push(frame.terraneGenerated.path, String(frame.terraneGenerated.line));
    else if (!showAssembly) args.push('--rust-only');
    const { stdout } = await exec(built.command, args, { signal: controller.signal, maxBuffer: 16 * 1024 * 1024 });
    if (id !== revision) return;
    const mapped = JSON.parse(stdout);
    if (!showAssembly) {
      const rust = await rustExcerpt(mapped.rust);
      if (id === revision) panel.webview.html = page(`${path.basename(file)}:${line}`, rust, null, false);
      return;
    }
    const live = !!frame?.instructionPointerReference;
    let instructions = mapped.assembly;
    let unavailable = frame && !live ? 'This frame has no instruction pointer.' : 'No instructions mapped to this line.';
    if (live) {
      try {
        ({ instructions } = await session.customRequest('disassemble', {
          memoryReference: frame.instructionPointerReference, instructionCount: 12, resolveSymbols: true,
        }));
      } catch (error) {
        unavailable = `Debugger disassembly unavailable: ${error.message}`;
      }
    }
    if (id !== revision) return;
    const asm = instructions.map(({ address, instruction }, i) => {
      const row = `${escape(address)}  ${escape(instruction)}`;
      return live && i === 0 ? `<mark>${row}</mark>` : row;
    }).join('\n');
    const rust = await rustExcerpt(mapped.rust);
    if (id === revision) panel.webview.html = page(`${path.basename(file)}:${line}`, rust,
      asm ? `<pre>${asm}</pre>` : `<small>${escape(unavailable)}</small>`, !!frame);
  }
  const command = vscode.commands.registerCommand('terrane.showBuildView', async () => {
    const editor = vscode.window.activeTextEditor;
    if (!terrane(editor) || !vscode.workspace.isTrusted) return;
    reset();
    sourceColumn = editor.viewColumn;
    if (!panel) {
      panel = vscode.window.createWebviewPanel('terrane.buildView', 'Terrane · Rust and Assembly',
        { viewColumn: vscode.ViewColumn.Beside, preserveFocus: true },
        { localResourceRoots: [], enableCommandUris: [toggleAssembly] });
      panel.onDidDispose(() => { panel = undefined; selection = undefined; showAssembly = false; reset(); });
    } else panel.reveal(panel.viewColumn, true);
    const session = vscode.debug.activeDebugSession;
    if (session?.type !== 'terrane' && !editor.document.isDirty) {
      cancellation = new vscode.CancellationTokenSource();
      const token = cancellation.token;
      const file = editor.document.uri.fsPath;
      const folder = vscode.workspace.getWorkspaceFolder(editor.document.uri);
      message('Building saved Terrane source…');
      artifact = build(folder, file, token);
      artifact.catch(() => {}); // May be superseded by another selection before the view awaits it.
    }
    const item = vscode.debug.activeStackItem;
    schedule(editor, item?.session.type === 'terrane' ? item : undefined);
  });
  context.subscriptions.push(command,
    vscode.commands.registerCommand(toggleAssembly, () => {
      if (!panel || !selection) return;
      showAssembly = !showAssembly;
      schedule(selection.editor, selection.item);
    }),
    vscode.window.onDidChangeTextEditorSelection(event => {
      if (terrane(event.textEditor) && event.kind !== vscode.TextEditorSelectionChangeKind.Command &&
          (!vscode.debug.activeStackItem || event.kind)) schedule(event.textEditor, undefined, 150);
    }),
    vscode.workspace.onDidChangeTextDocument(event => {
      if (panel && event.document.isDirty && terrane(vscode.window.activeTextEditor) && event.document === vscode.window.activeTextEditor.document) {
        invalidate(); message('Save this source before comparing it with a debug build.');
      }
    }),
    vscode.debug.onDidChangeActiveStackItem(item => {
      if (item?.session.type === 'terrane') schedule(undefined, item);
      else if (panel) { selection = undefined; invalidate(); message('Select a Terrane source line or pause in a mapped frame.'); }
    }),
    { dispose() { panel?.dispose(); reset(); } });
}

module.exports = { registerBuildView };
