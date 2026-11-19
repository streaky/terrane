'use strict';

// Run in an isolated VS Code extension-test host, not Node's unit-test runner:
// TERRANE_DEBUG_SMOKE_COMPILER=/absolute/path/to/terrane code --user-data-dir <scratch>/data \
//   --extensions-dir <scratch>/extensions --extensionDevelopmentPath=<repo>/editors/vscode \
//   --extensionTestsPath=<repo>/editors/vscode/tests/debugger.integration.js
// Optional TERRANE_DEBUG_SMOKE_PAUSE writes a stopped-state JSON file and waits for
// <that-file>.continue so the actual window can be captured before stepping.
const assert = require('node:assert/strict');
const fs = require('node:fs');
const os = require('node:os');
const path = require('node:path');
const vscode = require('vscode');

exports.run = async () => {
  const root = fs.mkdtempSync(path.join(os.tmpdir(), 'terrane-vscode-smoke-'));
  const source = [
    'namespace vscode-debug-smoke', '', 'function main;', '  rust',
    '    println!("RUN_CWD={}", std::env::current_dir().unwrap().display());',
    '  small = 41', String.raw`  raw = 'literal \n stays raw'`,
    String.raw`  text = "hello \"quoted\" {small} {{braces}}"`,
    '  small = small + 1', '  print; text', '  print; raw', '  print; small', '',
  ].join('\n');
  const file = path.join(root, 'main.trn');
  const cwd = path.join(root, 'run');
  fs.mkdirSync(cwd);
  fs.writeFileSync(file, source);
  const uri = vscode.Uri.file(file);
  const configuration = vscode.workspace.getConfiguration('terrane');
  const debugConfiguration = vscode.workspace.getConfiguration('debug');
  const previousCompiler = configuration.inspect('debugger.path')?.globalValue;
  const previousSave = debugConfiguration.inspect('saveBeforeStart')?.globalValue;
  const messages = [];
  let session;
  let tracker;
  let document;
  let stoppedResolve;
  const stopped = () => new Promise(resolve => { stoppedResolve = resolve; });
  try {
    await configuration.update('debugger.path', process.env.TERRANE_DEBUG_SMOKE_COMPILER || 'terrane', vscode.ConfigurationTarget.Global);
    await debugConfiguration.update('saveBeforeStart', 'none', vscode.ConfigurationTarget.Global);
    await vscode.extensions.getExtension('terrane.terrane-language').activate();
    document = await vscode.workspace.openTextDocument(uri);
    await vscode.window.showTextDocument(document);
    const edit = new vscode.WorkspaceEdit();
    edit.insert(uri, document.positionAt(document.getText().length), '# intentionally unsaved\n');
    assert(await vscode.workspace.applyEdit(edit));
    vscode.debug.addBreakpoints([new vscode.SourceBreakpoint(new vscode.Location(uri, new vscode.Position(8, 0)))]);
    tracker = vscode.debug.registerDebugAdapterTrackerFactory('terrane', {
      createDebugAdapterTracker(value) {
        session = value;
        return { onDidSendMessage(message) {
          messages.push(message);
          if (message.type === 'event' && message.event === 'stopped') stoppedResolve(message);
        } };
      },
    });
    const firstStop = stopped();
    assert(await vscode.debug.startDebugging(undefined, {
      type: 'terrane', request: 'launch', name: 'Terrane Debug Smoke',
      program: file, cwd, args: ['spaced π argument'], stopOnEntry: false,
    }));
    const stop = await firstStop;
    assert(document.isDirty, 'debug.saveBeforeStart=none must not be overridden');
    assert.equal(fs.readFileSync(file, 'utf8'), source);
    const inspect = async () => {
      const frames = await session.customRequest('stackTrace', { threadId: stop.body.threadId });
      const frame = frames.stackFrames[0];
      const scopes = await session.customRequest('scopes', { frameId: frame.id });
      const variables = await session.customRequest('variables', { variablesReference: scopes.scopes[0].variablesReference });
      return { frame, variables: variables.variables };
    };
    const before = await inspect();
    assert.equal(before.frame.source.path, file);
    assert.equal(before.frame.line, 9);
    assert(before.variables.some(value => value.name === 'small' && value.value === '41'));
    assert(messages.some(message => message.event === 'terrane/fidelity' && message.body.mode === 'source'));
    assert(messages.some(message => message.command === 'setBreakpoints' && message.success && message.body.breakpoints.some(value => value.verified)));
    assert(messages.some(message => message.event === 'output' && message.body.output?.includes(`RUN_CWD=${cwd}`)));
    if (process.env.TERRANE_DEBUG_SMOKE_PAUSE) {
      const marker = process.env.TERRANE_DEBUG_SMOKE_PAUSE;
      await new Promise((resolve, reject) => {
        const timer = setTimeout(() => { watcher.close(); reject(new Error('Screenshot release timeout')); }, 180000);
        const finish = () => {
          if (!fs.existsSync(`${marker}.continue`)) return;
          clearTimeout(timer); watcher.close(); resolve();
        };
        const watcher = fs.watch(path.dirname(marker), finish);
        fs.writeFileSync(marker, JSON.stringify(before, null, 2));
        finish();
      });
    }
    const nextStop = stopped();
    await session.customRequest('next', { threadId: stop.body.threadId });
    await nextStop;
    const after = await inspect();
    assert.equal(after.frame.line, 10);
    assert(after.variables.some(value => value.name === 'small' && value.value === '42'));
    await vscode.debug.stopDebugging(session);
    session = undefined;
    console.log('Terrane debugger smoke passed: source breakpoint 9, next 10, small 41 -> 42, explicit cwd, unsaved buffer retained, Stop.');
  } finally {
    if (session) await vscode.debug.stopDebugging(session);
    tracker?.dispose();
    vscode.debug.removeBreakpoints(vscode.debug.breakpoints.filter(point => point instanceof vscode.SourceBreakpoint && point.location.uri.toString() === uri.toString()));
    if (document?.isDirty) {
      const edit = new vscode.WorkspaceEdit();
      edit.replace(uri, new vscode.Range(document.positionAt(0), document.positionAt(document.getText().length)), source);
      await vscode.workspace.applyEdit(edit);
      await document.save();
    }
    await configuration.update('debugger.path', previousCompiler, vscode.ConfigurationTarget.Global);
    await debugConfiguration.update('saveBeforeStart', previousSave, vscode.ConfigurationTarget.Global);
    fs.rmSync(root, { recursive: true, force: true });
  }
};
