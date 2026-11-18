'use strict';

const assert = require('node:assert/strict');
const fs = require('node:fs');
const os = require('node:os');
const path = require('node:path');
const { test } = require('node:test');
const { registerDebugger } = require('../debugger');

const posix = { skip: process.platform === 'win32' ? 'Executable fixtures use POSIX shebangs and process groups' : false };
const launch = { type: 'terrane', request: 'launch', name: 'Debug', program: '/tmp/app.trn' };
const artifact = { program: '/tmp/native executable', terraneProvenance: '/tmp/provenance.json' };
const jsonCompiler = `process.stdout.write(JSON.stringify(${JSON.stringify(artifact)}));`;

function fixture(t, script) {
  const root = fs.mkdtempSync(path.join(os.tmpdir(), 'terrane-debugger-'));
  const compiler = path.join(root, 'compiler');
  fs.writeFileSync(compiler, `#!${process.execPath}\n${script}`, { mode: 0o755 });
  t.after(() => fs.rmSync(root, { recursive: true, force: true }));
  return { root, compiler };
}

function cancellationToken() {
  const callbacks = new Set();
  return {
    isCancellationRequested: false,
    disposed: 0,
    get listeners() { return callbacks.size; },
    onCancellationRequested(callback) {
      callbacks.add(callback);
      return { dispose: () => { if (callbacks.delete(callback)) this.disposed++; } };
    },
    cancel() {
      this.isCancellationRequested = true;
      for (const callback of callbacks) callback();
    },
  };
}

function client({ trusted = true, compiler = '/missing/terrane', languageId = 'terrane', folderPath } = {}) {
  let provider;
  let factory;
  let output = '';
  const errors = [];
  const subscriptions = [];
  const progressToken = cancellationToken();
  const disposable = { dispose() {} };
  const folder = folderPath ? { uri: { fsPath: folderPath } } : undefined;
  const vscode = {
    workspace: {
      isTrusted: trusted,
      getConfiguration: () => ({ get: () => compiler }),
      saveAll() { throw new Error('The extension must not override debug.saveBeforeStart'); },
    },
    window: {
      activeTextEditor: languageId ? { document: { languageId } } : undefined,
      createOutputChannel: () => ({
        ...disposable, clear() { output = ''; }, appendLine(text) { output += `${text}\n`; },
        append(text) { output += text; }, show() {},
      }),
      showErrorMessage: message => errors.push(message),
      withProgress: (_, work) => work({}, progressToken),
    },
    ProgressLocation: { Notification: 15 },
    debug: {
      registerDebugConfigurationProvider: (_, value) => { provider = value; return disposable; },
      registerDebugAdapterDescriptorFactory: (_, value) => { factory = value; return disposable; },
    },
    DebugAdapterExecutable: class {
      constructor(command, args) { this.command = command; this.args = args; }
    },
  };
  registerDebugger(vscode, { subscriptions });
  return {
    provider, factory, errors, folder, progressToken, get output() { return output; },
    resolve: (config = launch, token) => provider.resolveDebugConfigurationWithSubstitutedVariables(folder, config, token),
    dispose() { for (const subscription of subscriptions) subscription.dispose(); },
  };
}

function waitForFile(file) {
  return new Promise((resolve, reject) => {
    const timer = setTimeout(() => { watcher.close(); reject(new Error(`Timed out waiting for ${file}`)); }, 5000);
    const check = () => {
      if (!fs.existsSync(file)) return;
      clearTimeout(timer);
      watcher.close();
      resolve(fs.readFileSync(file, 'utf8'));
    };
    const watcher = fs.watch(path.dirname(file), check);
    check();
  });
}

test('untrusted workspaces cannot build or start the adapter', async () => {
  const editor = client({ trusted: false });
  assert.equal(await editor.resolve(), undefined);
  assert.match(editor.errors[0], /Trust this workspace/);
  assert.throws(() => editor.factory.createDebugAdapterDescriptor({}), /trust/);
});

test('default launches reject non-Terrane or absent editors, but explicit manifests work', () => {
  for (const languageId of ['plaintext', undefined]) {
    const editor = client({ languageId: languageId ?? '' });
    assert.equal(editor.provider.resolveDebugConfiguration(undefined, {}), undefined);
    assert.equal(editor.provider.resolveDebugConfiguration(undefined, { ...launch, program: '${file}' }), undefined);
    assert.match(editor.errors[0], /Open a Terrane source/);
    const manifest = { ...launch, program: 'terrane.toml' };
    assert.equal(editor.provider.resolveDebugConfiguration(undefined, manifest), manifest);
  }
  assert.equal(client().provider.resolveDebugConfiguration(undefined, {}).program, '${file}');
});

test('successful child build replaces source with artifacts without forcing editor saves', posix, async t => {
  const f = fixture(t, jsonCompiler);
  const editor = client({ compiler: f.compiler });
  const token = cancellationToken();
  const result = await editor.resolve({ ...launch, args: ['spaced π argument'], stopOnEntry: true }, token);
  assert.equal(result.program, artifact.program);
  assert.equal(result.terraneProvenance, artifact.terraneProvenance);
  assert.deepEqual(result.args, ['spaced π argument']);
  assert.equal(result.stopOnEntry, true);
  assert.equal(result.disableASLR, false);
  const adapter = editor.factory.createDebugAdapterDescriptor({ configuration: result });
  assert.equal(adapter.command, f.compiler);
  assert.equal(editor.progressToken.listeners, 0);
  assert.equal(editor.progressToken.disposed, 1);
  assert.equal(token.listeners, 0);
});

test('compiler startup failure aborts launch and disposes subscriptions', async () => {
  const editor = client();
  assert.equal(await editor.resolve(), undefined);
  assert.match(editor.errors[0], /ENOENT/);
  assert.equal(editor.progressToken.listeners, 0);
});

test('malformed JSON, relative artifacts, and unsuccessful compilers abort launch', posix, async t => {
  for (const [script, message] of [
    ['process.stdout.write("not json");', /update the Terrane compiler/],
    ['process.stdout.write(JSON.stringify({program:"relative",terraneProvenance:"/tmp/p"}));', /absolute debug artifact paths/],
    ['process.stdout.write(JSON.stringify({program:"/tmp/p",terraneProvenance:"relative"}));', /absolute debug artifact paths/],
    ['process.stderr.write("fixture compile error"); process.exit(7);', /status 7/],
  ]) {
    const f = fixture(t, script);
    const editor = client({ compiler: f.compiler });
    assert.equal(await editor.resolve(), undefined);
    assert.match(editor.errors[0], message);
    assert.equal(editor.progressToken.listeners, 0);
    if (script.includes('stderr')) assert.match(editor.output, /fixture compile error/);
  }
});

test('relative compiler paths use one workspace base for build and adapter', posix, async t => {
  const f = fixture(t, `require('fs').writeFileSync('invocation.json', JSON.stringify({cwd:process.cwd(),args:process.argv.slice(2)})); ${jsonCompiler}`);
  const editor = client({ compiler: './compiler', folderPath: f.root });
  const result = await editor.resolve({ ...launch, program: 'src/app.trn' });
  const invocation = JSON.parse(fs.readFileSync(path.join(f.root, 'invocation.json')));
  assert.deepEqual(invocation, { cwd: f.root, args: ['debug', '--build-only', path.join(f.root, 'src/app.trn')] });
  assert.equal(result.stopOnEntry, false);
  assert.equal(editor.factory.createDebugAdapterDescriptor({ workspaceFolder: editor.folder, configuration: result }).command, f.compiler);
});

test('folderless relative source, compiler, and debuggee cwd use the source directory', posix, async t => {
  const f = fixture(t, `require('fs').writeFileSync('invocation.json', JSON.stringify({cwd:process.cwd(),args:process.argv.slice(2)})); ${jsonCompiler}`);
  const sourceDir = path.join(f.root, 'src');
  fs.mkdirSync(sourceDir);
  const editor = client({ compiler: '../compiler' });
  const source = path.join(sourceDir, 'app.trn');
  const result = await editor.resolve({ ...launch, program: path.relative(process.cwd(), source), cwd: 'run' });
  const invocation = JSON.parse(fs.readFileSync(path.join(sourceDir, 'invocation.json')));
  assert.deepEqual(invocation, { cwd: sourceDir, args: ['debug', '--build-only', source] });
  assert.equal(result.cwd, path.join(sourceDir, 'run'));
  assert.equal(editor.factory.createDebugAdapterDescriptor({ configuration: result }).command, f.compiler);
});

test('pre-cancelled requests and disposal prevent a new build', async () => {
  const editor = client();
  const token = cancellationToken();
  token.cancel();
  assert.equal(await editor.resolve(launch, token), undefined);
  editor.dispose();
  assert.equal(await editor.resolve(), undefined);
  assert.deepEqual(editor.errors, []);
});

for (const stop of ['request cancellation', 'progress cancellation', 'extension disposal']) {
  test(`${stop} terminates the compiler and its descendant`, posix, async t => {
    const f = fixture(t, `
      const fs = require('fs'), path = require('path');
      const root = __dirname;
      process.on('SIGTERM', () => { fs.writeFileSync(path.join(root,'parent-stopped'), 'yes'); process.exit(0); });
      const script = "process.on('SIGTERM',()=>{require('fs').writeFileSync(process.argv[1],'yes');process.exit(0)});process.send('ready');setInterval(()=>{},1000)";
      const child = require('child_process').spawn(process.execPath, ['-e', script, path.join(root,'child-stopped')], {stdio:['ignore','inherit','inherit','ipc']});
      child.once('message', () => fs.writeFileSync(path.join(root,'ready'), JSON.stringify({parent:process.pid,child:child.pid})));
      setInterval(()=>{},1000);
    `);
    const editor = client({ compiler: f.compiler, folderPath: f.root });
    const token = cancellationToken();
    const ready = waitForFile(path.join(f.root, 'ready'));
    const pending = editor.resolve(launch, token);
    const pids = JSON.parse(await ready);
    t.after(() => { try { process.kill(-pids.parent); } catch {} });
    const parentStopped = waitForFile(path.join(f.root, 'parent-stopped'));
    const childStopped = waitForFile(path.join(f.root, 'child-stopped'));
    if (stop === 'extension disposal') editor.dispose();
    else if (stop === 'progress cancellation') editor.progressToken.cancel();
    else token.cancel();
    assert.equal(await pending, undefined);
    assert.equal(await parentStopped, 'yes');
    assert.equal(await childStopped, 'yes');
    assert.equal(token.listeners, 0);
    assert.equal(editor.progressToken.listeners, 0);
    assert.deepEqual(editor.errors, []);
  });
}

test('a vanished process group still falls back to stopping the compiler child', posix, async t => {
  const f = fixture(t, `require('fs').writeFileSync(__dirname+'/ready',String(process.pid)); setInterval(()=>{},1000);`);
  const editor = client({ compiler: f.compiler });
  const ready = waitForFile(path.join(f.root, 'ready'));
  const pending = editor.resolve();
  const pid = Number(await ready);
  const kill = process.kill.bind(process);
  t.after(() => { try { kill(-pid); } catch {} });
  t.mock.method(process, 'kill', (target, signal) => {
    if (target === -pid) throw Object.assign(new Error('Group no longer exists'), { code: 'ESRCH' });
    return kill(target, signal);
  });
  editor.progressToken.cancel();
  assert.equal(await pending, undefined);
  assert.equal(editor.progressToken.listeners, 0);
  assert.deepEqual(editor.errors, []);
});

test('non-string args and cwd are rejected before building', async () => {
  const editor = client();
  assert.equal(await editor.resolve({ ...launch, args: ['valid', 5] }), undefined);
  assert.match(editor.errors[0], /array of strings/);
  assert.equal(await editor.resolve({ ...launch, cwd: 5 }), undefined);
  assert.match(editor.errors[1], /directory path/);
});
