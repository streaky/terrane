'use strict';

const path = require('path');
const { spawn } = require('child_process');

function registerDebugger(vscode, context) {
  let output;
  const debuggerOutput = () => output ??= vscode.window.createOutputChannel('Terrane Debugger');
  const commandFor = (folder, base) => {
    const configured = vscode.workspace.getConfiguration('terrane', folder?.uri)
      .get('debugger.path', 'terrane').trim() || 'terrane';
    return path.isAbsolute(configured) || !configured.includes('/') && !configured.includes('\\')
      ? configured : path.resolve(base, configured);
  };
  const initial = () => ({
    type: 'terrane', request: 'launch', name: 'Debug Terrane',
    program: '${file}', args: [], stopOnEntry: false,
  });
  const activeTerrane = () => vscode.window.activeTextEditor?.document?.languageId === 'terrane';
  const activeBuilds = new Set();
  let disposed = false;
  const disposeBuilds = { dispose() {
    disposed = true;
    for (const cancel of activeBuilds) cancel();
    output?.dispose();
  } };

  const provider = {
    provideDebugConfigurations: () => [initial()],
    resolveDebugConfiguration(folder, config) {
      const empty = !config.type && !config.request && !config.name;
      if (empty || config.program === '${file}') {
        if (!activeTerrane()) {
          vscode.window.showErrorMessage('Open a Terrane source file, or set program to a Terrane package manifest.');
          return undefined;
        }
        if (empty) return initial();
      }
      return config;
    },
    async resolveDebugConfigurationWithSubstitutedVariables(folder, config, token) {
      if (!vscode.workspace.isTrusted) {
        vscode.window.showErrorMessage('Trust this workspace before building or debugging Terrane programs.');
        return undefined;
      }
      if (config.request !== 'launch' || typeof config.program !== 'string' || !config.program.trim()) {
        vscode.window.showErrorMessage('Terrane debugging requires a launch configuration with a source file or manifest in program.');
        return undefined;
      }
      if (config.args !== undefined && (!Array.isArray(config.args) || config.args.some(arg => typeof arg !== 'string'))) {
        vscode.window.showErrorMessage('Terrane debugger args must be an array of strings.');
        return undefined;
      }
      if (config.cwd !== undefined && typeof config.cwd !== 'string') {
        vscode.window.showErrorMessage('Terrane debugger cwd must be a directory path.');
        return undefined;
      }
      if (token?.isCancellationRequested || disposed) return undefined;
      const source = path.resolve(folder?.uri.fsPath || process.cwd(), config.program);
      const cwd = folder?.uri.fsPath || path.dirname(source);
      const command = commandFor(folder, cwd);
      const configuredCwd = config.cwd === undefined ? undefined : path.resolve(cwd, config.cwd);
      const output = debuggerOutput();
      output.clear();
      output.appendLine(`Building debug program: ${source}`);
      try {
        const artifact = await vscode.window.withProgress({
          location: vscode.ProgressLocation.Notification,
          title: 'Building Terrane debug program',
          cancellable: true,
        }, (_, progressToken) => buildProgram(command, source, cwd, output, [token, progressToken], activeBuilds, () => disposed));
        if (token?.isCancellationRequested || disposed) return undefined;
        return {
          ...config, ...artifact,
          ...(configuredCwd === undefined ? {} : { cwd: configuredCwd }),
          // Retain the resolved compiler when program becomes a native artifact, especially folderless.
          __terraneCompiler: command,
          stopOnEntry: config.stopOnEntry ?? false, disableASLR: false,
        };
      } catch (error) {
        if (error.cancelled) return undefined;
        output.show(true);
        vscode.window.showErrorMessage(`Terrane debug build failed: ${error.message}`);
        return undefined;
      }
    },
  };
  context.subscriptions.push(
    vscode.debug.registerDebugConfigurationProvider('terrane', provider),
    vscode.debug.registerDebugAdapterDescriptorFactory('terrane', {
      createDebugAdapterDescriptor(session) {
        if (!vscode.workspace.isTrusted) throw new Error('Terrane debugging requires workspace trust.');
        return new vscode.DebugAdapterExecutable(session.configuration.__terraneCompiler, ['debug-adapter', '--stdio']);
      },
    }),
    disposeBuilds,
  );
  return async (folder, source, token) => {
    if (!vscode.workspace.isTrusted) throw new Error('Trust this workspace before building.');
    const cwd = folder?.uri.fsPath || path.dirname(source);
    const command = commandFor(folder, cwd);
    const artifact = await vscode.window.withProgress({
      location: vscode.ProgressLocation.Notification,
      title: 'Building Terrane source view',
      cancellable: true,
    }, (_, progressToken) => buildProgram(command, source, cwd, debuggerOutput(),
      [token, progressToken], activeBuilds, () => disposed,
      ['--view-build', source, ...(folder ? ['--workspace', cwd] : [])]));
    return { ...artifact, command };
  };
}

function buildProgram(command, source, cwd, output, tokens, activeBuilds, isDisposed, args = ['--build-only', source]) {
  return new Promise((resolve, reject) => {
    const cancelledError = () => Object.assign(new Error('Build cancelled'), { cancelled: true });
    if (isDisposed() || tokens.some(token => token?.isCancellationRequested)) {
      reject(cancelledError());
      return;
    }
    const child = spawn(command, ['debug', ...args], {
      cwd, detached: process.platform !== 'win32',
    });
    let stdout = '';
    let cancelled = false;
    let settled = false;
    const subscriptions = [];
    const finish = (error, value) => {
      if (settled) return;
      settled = true;
      activeBuilds.delete(cancel);
      for (const subscription of subscriptions) subscription.dispose();
      if (error) reject(error);
      else resolve(value);
    };
    const cancel = () => {
      if (settled) return;
      cancelled = true;
      if (child.pid && process.platform !== 'win32') {
        try { process.kill(-child.pid); } catch {
          child.kill();
        }
      } else child.kill();
    };
    child.stdout.setEncoding('utf8');
    child.stderr.setEncoding('utf8');
    child.stdout.on('data', chunk => { stdout += chunk; });
    child.stderr.on('data', chunk => output.append(chunk));
    child.on('error', error => finish(cancelled ? cancelledError() : error));
    child.on('close', code => {
      if (cancelled) return finish(cancelledError());
      if (code !== 0) return finish(new Error(`compiler exited with status ${code}; see Terrane Debugger output`));
      try {
        const artifact = JSON.parse(stdout);
        if (typeof artifact.program !== 'string' || !path.isAbsolute(artifact.program)
            || typeof artifact.terraneProvenance !== 'string' || !path.isAbsolute(artifact.terraneProvenance)) {
          throw new Error('compiler did not return absolute debug artifact paths');
        }
        finish(undefined, { program: artifact.program, terraneProvenance: artifact.terraneProvenance });
      } catch (error) {
        finish(new Error(`invalid debug build result (${error.message}); update the Terrane compiler`));
      }
    });
    activeBuilds.add(cancel);
    for (const token of tokens.filter(Boolean)) {
      subscriptions.push(token.onCancellationRequested(cancel));
    }
    if (isDisposed() || tokens.some(token => token?.isCancellationRequested)) cancel();
  });
}

module.exports = { registerDebugger };
