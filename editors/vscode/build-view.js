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
  let panel, sourceColumn, sourceEditor, artifact, query, timer, endedSession, revision = 0;
  let showAssembly = false, selection;
  function invalidate() {
    clearTimeout(timer);
    query?.abort();
    query = undefined;
    return ++revision;
  }
  function discardArtifact() {
    artifact?.cancellation?.cancel();
    artifact = undefined;
  }
  function reset() {
    invalidate();
    discardArtifact();
    sourceEditor = undefined;
  }
  const activeSession = () => {
    const session = vscode.debug.activeDebugSession;
    return session?.type === 'terrane' && session.id !== endedSession ? session : undefined;
  };
  const message = text => { if (panel) panel.webview.html = page(text, '', showAssembly ? '' : null, false); };
  const terrane = editor => editor?.document.languageId === 'terrane' && editor.document.uri.scheme === 'file';
  function schedule(editor, item, delay = 0) {
    if (!panel) return;
    selection = { editor, item };
    if (terrane(editor)) sourceEditor = editor;
    if (artifact && (item || activeSession() || (!artifact.ready && artifact.file !== editor?.document.uri.fsPath))) discardArtifact();
    const id = invalidate();
    timer = setTimeout(() => update(id, editor, item).catch(error => {
      if (id === revision) message(error.stderr?.trim() || error.message);
    }), delay);
  }
  async function update(id, editor, item) {
    if (!vscode.workspace.isTrusted) throw new Error('Trust this workspace to inspect native code.');
    const session = item?.session?.type === 'terrane' ? item.session : activeSession();
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
      sourceEditor = editor;
      const pos = new vscode.Position(frame.line - 1, 0);
      editor.selection = new vscode.Selection(pos, pos);
      editor.revealRange(editor.selection, vscode.TextEditorRevealType.InCenterIfOutsideViewport);
    }
    if (!terrane(editor)) throw new Error('Select a line in a saved Terrane source file.');
    if (editor.document.isDirty) throw new Error('Save this source before comparing it with a debug build.');
    const file = editor.document.uri.fsPath;
    const line = frame?.line ?? editor.selection.active.line + 1;
    const config = session?.configuration;
    if (!config && !artifact) {
      const cancellation = new vscode.CancellationTokenSource();
      const folder = vscode.workspace.getWorkspaceFolder(editor.document.uri);
      message('Building saved Terrane source…');
      const request = { file, cancellation };
      artifact = request;
      // Keep the settled promise, including a rejection, until invalidated.
      request.promise = build(folder, file, cancellation.token).then(built => {
        request.ready = true;
        return built;
      }).finally(() => {
        request.cancellation = undefined;
        cancellation.dispose();
      });
    }
    const built = config ? { ...config, command: config.__terraneCompiler } : await artifact.promise;
    if (id !== revision) return;
    if (!built?.command) throw new Error('The Terrane build did not provide a compiler for source inspection.');
    const controller = query = new AbortController();
    const args = ['debug', '--view-json', built.program, built.terraneProvenance, file, String(line)];
    if (frame) args.push(frame.terraneGenerated.path, String(frame.terraneGenerated.line));
    else if (!showAssembly) args.push('--rust-only');
    if (config?.terraneRelocation) args.push('--relocation', JSON.stringify(config.terraneRelocation));
    let stdout;
    try {
      ({ stdout } = await exec(built.command, args, { signal: controller.signal, maxBuffer: 16 * 1024 * 1024 }));
    } catch (error) {
      if (id !== revision) return;
      if (config || artifact.file === file) throw error;
      // Only source coverage/freshness diagnostics from --view-json justify a new build.
      const diagnostic = error.stderr?.trim();
      const prefix = '<debugger>: error[S5001]: ';
      if (diagnostic !== `${prefix}${file} is not in this debug build` &&
          !diagnostic?.startsWith(`${prefix}source has changed or is unavailable in this debug build: `)) throw error;
      discardArtifact();
      return update(id, editor, item);
    }
    if (id !== revision) return;
    if (!config) artifact.file = file;
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
  const command = vscode.commands.registerCommand('terrane.showBuildView', () => {
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
    const item = vscode.debug.activeStackItem;
    schedule(editor, item?.session.type === 'terrane' && item.session.id !== endedSession ? item : undefined);
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
    vscode.window.onDidChangeActiveTextEditor(editor => {
      if (terrane(editor) && !vscode.debug.activeStackItem) schedule(editor, undefined, 150);
    }),
    vscode.workspace.onDidChangeTextDocument(event => {
      if (panel && event.document.isDirty && event.document === sourceEditor?.document) {
        invalidate(); message('Save this source before comparing it with a debug build.');
      }
    }),
    vscode.workspace.onDidSaveTextDocument(document => {
      if (!panel || document.languageId !== 'terrane' || document.uri.scheme !== 'file') return;
      discardArtifact();
      if (selection) schedule(sourceEditor, selection.item, 150);
    }),
    vscode.debug.onDidChangeActiveStackItem(item => {
      if (item?.session.type === 'terrane' && item.session.id !== endedSession) schedule(undefined, item);
      else if (!activeSession() && terrane(sourceEditor)) schedule(sourceEditor, undefined);
      else if (panel) { selection = undefined; invalidate(); message('Select a Terrane source line or pause in a mapped frame.'); }
    }),
    vscode.debug.onDidTerminateDebugSession(session => {
      if (session.type !== 'terrane') return;
      const current = activeSession();
      endedSession = session.id;
      if (panel && (!current || current.id === session.id) && terrane(sourceEditor)) {
        schedule(sourceEditor, undefined);
      }
    }),
    { dispose() { panel?.dispose(); reset(); } });
}

module.exports = { registerBuildView };
