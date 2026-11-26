'use strict';
const assert = require('node:assert/strict');
const Module = require('node:module');
const path = require('node:path');
const test = require('node:test');

function loadExtension(t, vscode, replacements = {}) {
  const entry = require.resolve('../extension');
  const load = Module._load;
  Module._load = function(request, parent, isMain) {
    if (parent?.filename === entry) {
      if (request === 'vscode') return vscode;
      if (Object.hasOwn(replacements, request)) return replacements[request];
    }
    return load.call(this, request, parent, isMain);
  };
  t.after(() => { Module._load = load; delete require.cache[entry]; });
  return require(entry);
}

test('server path resolves against the single workspace folder', t => {
  const folder = { uri: { fsPath: '/workspace/terrane' } };
  const workspace = { workspaceFolders: [folder] };
  const scopes = [];
  let configured = 'target/release/terrane-language-server';
  workspace.getConfiguration = (_, scope) => {
    scopes.push(scope);
    return { get: () => configured };
  };
  const { serverPath } = loadExtension(t, { workspace });
  const context = { asAbsolutePath: relative => path.join('/extension', relative) };
  assert.equal(serverPath(context), '/workspace/terrane/target/release/terrane-language-server');
  workspace.workspaceFolders.push({ uri: { fsPath: '/another' } });
  assert.throws(() => serverPath(context), /absolute terrane\.server\.path/);
  configured = '/custom/server';
  assert.equal(serverPath(context), '/custom/server');
  configured = '';
  assert.equal(serverPath(context), '/extension/server/terrane-language-server');
  assert.deepEqual(scopes, [folder.uri, undefined, undefined, undefined]);
});

test('Rust-only activation preserves markers and lazy explicit debugging', async t => {
  const started = [];
  let openDocument, watching = true;
  const disposable = { dispose() {} };
  const vscode = {
    workspace: {
      textDocuments: [{ languageId: 'rust' }],
      onDidOpenTextDocument(fn) { openDocument = fn; return { dispose() { watching = false; } }; },
      onDidGrantWorkspaceTrust: () => disposable,
      getConfiguration: () => ({ get: () => process.execPath }),
    },
    window: { createOutputChannel() { assert.fail('Unexpected output channel'); } },
    debug: {
      registerDebugConfigurationProvider(_, provider) { started.push('debug configuration'); assert.ok(provider); return disposable; },
      registerDebugAdapterDescriptorFactory(_, factory) { started.push('debug adapter'); assert.ok(factory); return disposable; },
    },
    Disposable: class { constructor(dispose) { this.dispose = dispose; } },
  };
  const { activate, deactivate } = loadExtension(t, vscode, {
    './security-wrap': { registerSecurityWrap() { started.push('wrap'); return disposable; } },
    './security-region-decorations': { registerSecurityRegions() { started.push('markers'); } },
    './build-view': { registerBuildView() { started.push('build view'); } },
    './rust-block-decorations': { createRustBlockDecorations() { started.push('Rust blocks'); return disposable; } },
    'vscode-languageclient/node': { LanguageClient: class {
      start() { started.push('language client'); }
      stop() {}
    } },
  });
  await activate({ subscriptions: [] });
  openDocument({ languageId: 'rust' });
  assert.deepEqual(started, ['debug configuration', 'debug adapter', 'wrap', 'markers']);
  assert.equal(watching, true);
  openDocument({ languageId: 'terrane' });
  assert.deepEqual(started.slice(4), ['build view', 'Rust blocks', 'language client']);
  assert.equal(watching, false);
  await deactivate();
});
