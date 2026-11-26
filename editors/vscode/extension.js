'use strict';

const fs = require('fs');
const path = require('path');
const vscode = require('vscode');

function startDecorations(context) {
  let disposed = false;
  let decorationDisposable;
  let outputChannel;
  function reportFailure(error) {
    const message = error instanceof Error ? error.stack || error.message : String(error);
    try {
      if (disposed) {
        console.error(`Rust block decorations unavailable: ${message}`);
        return;
      }
      if (!outputChannel) {
        outputChannel = vscode.window.createOutputChannel('Terrane');
        context.subscriptions.push(outputChannel);
      }
      outputChannel.appendLine(`Rust block decorations unavailable: ${message}`);
    } catch (reportingError) {
      console.error(`Rust block decorations unavailable: ${message}`, reportingError);
    }
  }
  const lifecycle = new vscode.Disposable(() => {
    disposed = true;
    decorationDisposable?.dispose();
  });
  context.subscriptions.push(lifecycle);
  try {
    const { createRustBlockDecorations } = require('./rust-block-decorations');
    Promise.resolve(createRustBlockDecorations())
      .then(disposable => {
        if (disposed) disposable.dispose();
        else decorationDisposable = disposable;
      })
      .catch(reportFailure);
  } catch (error) {
    reportFailure(error);
  }
}

let client;

function serverPath(context) {
  const folders = vscode.workspace.workspaceFolders;
  const folder = folders?.length === 1 ? folders[0] : undefined;
  const configured = vscode.workspace
    .getConfiguration('terrane', folder?.uri)
    .get('server.path', '')
    .trim();
  if (configured) {
    if (path.isAbsolute(configured)) return path.normalize(configured);
    if (folder) return path.resolve(folder.uri.fsPath, configured);
    throw new Error('Use an absolute terrane.server.path outside a single-folder workspace.');
  }
  const executable = process.platform === 'win32'
    ? 'terrane-language-server.exe'
    : 'terrane-language-server';
  return context.asAbsolutePath(path.join('server', executable));
}

async function activate(context) {
  const build = require('./debugger').registerDebugger(vscode, context);
  const start = () => startTerraneServices(context, build);
  context.subscriptions.push(require('./security-wrap').registerSecurityWrap(vscode));
  require('./security-region-decorations').registerSecurityRegions(vscode, context);
  if (vscode.workspace.textDocuments.some(doc => doc.languageId === 'terrane')) {
    await start();
  } else {
    const listener = vscode.workspace.onDidOpenTextDocument(doc => {
      if (doc.languageId !== 'terrane') return;
      listener.dispose();
      start().catch(error => vscode.window.showErrorMessage(`Terrane initialization failed: ${error.message}`));
    });
    context.subscriptions.push(listener);
  }
}

async function startTerraneServices(context, build) {
  require('./build-view').registerBuildView(vscode, context, build);
  startDecorations(context);
  const command = serverPath(context);
  if (!fs.existsSync(command)) {
    const message = `Terrane language server not found at ${command}. Configure terrane.server.path or package the server binary.`;
    vscode.window.showErrorMessage(message);
    throw new Error(message);
  }
  const { LanguageClient } = require('vscode-languageclient/node');
  client = new LanguageClient(
    'terraneLanguageServer',
    'Terrane Language Server',
    { command },
    {
      initializationOptions: () => ({
        allowConsumerExecution: vscode.workspace.isTrusted,
      }),
      documentSelector: [
        { scheme: 'file', language: 'terrane' },
        { scheme: 'untitled', language: 'terrane' },
      ],
      synchronize: {
        configurationSection: 'terrane',
      },
    },
  );
  await client.start();
  context.subscriptions.push(client);
  context.subscriptions.push(vscode.workspace.onDidGrantWorkspaceTrust(async () => {
    await client.restart();
  }));
}

async function deactivate() {
  if (client) {
    await client.stop();
    client = undefined;
  }
}

module.exports = { activate, deactivate, serverPath };
