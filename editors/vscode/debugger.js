'use strict';

const path = require('path');
const { spawn } = require('child_process');

function registerDebugger(vscode, context) {
  const output = vscode.window.createOutputChannel('Terrane Debugger');
  const commandFor = (folder) => vscode.workspace.getConfiguration('terrane', folder?.uri)
    .get('debugger.path', 'terrane').trim() || 'terrane';
  const initial = () => ({
    type: 'terrane', request: 'launch', name: 'Debug Terrane',
    program: '${file}', args: [], stopOnEntry: false,
  });

  const provider = {
    provideDebugConfigurations: () => [initial()],
    resolveDebugConfiguration(folder, config) {
      if (!config.type && !config.request && !config.name) {
        return initial();
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
      // A failed save must not launch an executable built from older on-disk text.
      if (!await vscode.workspace.saveAll(false) || token?.isCancellationRequested) {
        return undefined;
      }
      const cwd = folder?.uri.fsPath || path.dirname(config.program);
      const source = path.resolve(cwd, config.program);
      const command = commandFor(folder);
      output.clear();
      output.appendLine(`Building debug program: ${source}`);
      try {
        const artifact = await vscode.window.withProgress({
          location: vscode.ProgressLocation.Notification,
          title: 'Building Terrane debug program',
          cancellable: true,
        }, (_, progressToken) => buildProgram(command, source, cwd, output, [token, progressToken]));
        if (token?.isCancellationRequested) return undefined;
        return { ...config, ...artifact, stopOnEntry: config.stopOnEntry ?? false, disableASLR: false };
      } catch (error) {
        if (error.cancelled) return undefined;
        output.show(true);
        vscode.window.showErrorMessage(`Terrane debug build failed: ${error.message}`);
        return undefined;
      }
    },
  };
  context.subscriptions.push(
    output,
    vscode.debug.registerDebugConfigurationProvider('terrane', provider),
    vscode.debug.registerDebugAdapterDescriptorFactory('terrane', {
      createDebugAdapterDescriptor(session) {
        if (!vscode.workspace.isTrusted) throw new Error('Terrane debugging requires workspace trust.');
        return new vscode.DebugAdapterExecutable(commandFor(session.workspaceFolder), ['debug-adapter', '--stdio']);
      },
    }),
  );
}

function buildProgram(command, source, cwd, output, tokens) {
  return new Promise((resolve, reject) => {
    const child = spawn(command, ['debug', '--build-only', source], {
      cwd, detached: process.platform !== 'win32',
    });
    let stdout = '';
    let cancelled = false;
    const cancel = () => {
      cancelled = true;
      if (child.pid && process.platform !== 'win32') {
        try { process.kill(-child.pid); } catch (error) {
          if (error.code !== 'ESRCH') child.kill();
        }
      } else {
        child.kill();
      }
    };
    const subscriptions = tokens.filter(Boolean).map(token => token.onCancellationRequested(cancel));
    if (tokens.some(token => token?.isCancellationRequested)) cancel();
    child.stdout.setEncoding('utf8');
    child.stderr.setEncoding('utf8');
    child.stdout.on('data', chunk => { stdout += chunk; });
    child.stderr.on('data', chunk => output.append(chunk));
    child.on('error', reject);
    child.on('close', code => {
      for (const subscription of subscriptions) subscription.dispose();
      if (cancelled) return reject(Object.assign(new Error('Build cancelled'), { cancelled: true }));
      if (code !== 0) return reject(new Error(`compiler exited with status ${code}; see Terrane Debugger output`));
      try {
        const artifact = JSON.parse(stdout);
        if (typeof artifact.program !== 'string' || !path.isAbsolute(artifact.program)
            || typeof artifact.terraneProvenance !== 'string' || !path.isAbsolute(artifact.terraneProvenance)) {
          throw new Error('compiler did not return absolute debug artifact paths');
        }
        resolve({ program: artifact.program, terraneProvenance: artifact.terraneProvenance });
      } catch (error) {
        reject(new Error(`invalid debug build result (${error.message}); update the Terrane compiler`));
      }
    });
  });
}

module.exports = { registerDebugger };
