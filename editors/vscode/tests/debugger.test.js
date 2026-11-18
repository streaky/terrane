'use strict';

const assert = require('node:assert/strict');
const { test } = require('node:test');
const { registerDebugger } = require('../debugger');

function client({ trusted = true, saved = true } = {}) {
  let provider;
  let factory;
  let saves = 0;
  const errors = [];
  const disposable = { dispose() {} };
  const vscode = {
    workspace: {
      isTrusted: trusted,
      getConfiguration: () => ({ get: () => '/missing/terrane' }),
      saveAll: async () => { saves++; return saved; },
    },
    window: {
      createOutputChannel: () => ({ ...disposable, clear() {}, appendLine() {}, append() {}, show() {} }),
      showErrorMessage: message => errors.push(message),
      withProgress: (_, work) => work({}, { onCancellationRequested: () => disposable }),
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
  registerDebugger(vscode, { subscriptions: [] });
  return { provider, factory, errors, get saves() { return saves; } };
}

const launch = { type: 'terrane', request: 'launch', name: 'Debug', program: '/tmp/app.trn' };

test('untrusted workspaces cannot save, build, or start the adapter', async () => {
  const editor = client({ trusted: false });
  assert.equal(await editor.provider.resolveDebugConfigurationWithSubstitutedVariables(undefined, launch), undefined);
  assert.equal(editor.saves, 0);
  assert.match(editor.errors[0], /Trust this workspace/);
  assert.throws(() => editor.factory.createDebugAdapterDescriptor({}), /trust/);
});

test('failed saves and cancelled requests cannot launch stale source', async () => {
  const editor = client({ saved: false });
  assert.equal(await editor.provider.resolveDebugConfigurationWithSubstitutedVariables(undefined, launch), undefined);
  const cancelled = client();
  assert.equal(await cancelled.provider.resolveDebugConfigurationWithSubstitutedVariables(undefined, launch, {
    isCancellationRequested: true,
  }), undefined);
});

test('compiler startup failure aborts launch with an actionable error', async () => {
  const editor = client();
  assert.equal(await editor.provider.resolveDebugConfigurationWithSubstitutedVariables(undefined, launch), undefined);
  assert.match(editor.errors[0], /Terrane debug build failed:.*ENOENT/);
});

test('non-string arguments are rejected before building', async () => {
  const editor = client();
  assert.equal(await editor.provider.resolveDebugConfigurationWithSubstitutedVariables(undefined, {
    ...launch, args: ['valid', 5],
  }), undefined);
  assert.equal(editor.saves, 0);
  assert.match(editor.errors[0], /array of strings/);
});
