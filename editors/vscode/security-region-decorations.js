'use strict';

const { scanMarkers } = require('./security-markers');

function registerSecurityRegions(vscode, context) {
  const supported = doc => doc.languageId === 'terrane' || doc.languageId === 'rust';
  const configuration = doc => vscode.workspace.getConfiguration('terrane', doc.uri);
  const scansMarkers = doc => supported(doc)
    && (doc.languageId !== 'rust' || configuration(doc).get('securityRegions.rust', true));
  const diagnostics = vscode.languages.createDiagnosticCollection('terrane-security-markers');
  const decoration = vscode.window.createTextEditorDecorationType({
    isWholeLine: true,
    backgroundColor: new vscode.ThemeColor('terrane.securityRegionBackground'),
    borderColor: new vscode.ThemeColor('terrane.securityRegionBorder'),
    borderStyle: 'solid', borderWidth: '0 2px 0 0',
    rangeBehavior: vscode.DecorationRangeBehavior.ClosedClosed,
  });
  const cache = new WeakMap();
  const timers = new Map();
  function update(doc) {
    if (!supported(doc)) return;
    if (!scansMarkers(doc)) {
      cancel(doc);
      if (cache.delete(doc)) diagnostics.delete(doc.uri);
      decorate(doc, []);
      return;
    }
    let entry = cache.get(doc);
    if (entry?.version !== doc.version) {
      const { regions, issues } = scanMarkers(doc.getText().split(/\r?\n/), doc.languageId);
      const ranges = regions.map(({ start, end }) => new vscode.Range(start, 0, end, doc.lineAt(end).text.length));
      diagnostics.set(doc.uri, issues.map(({ line, message }) => {
        const diag = new vscode.Diagnostic(doc.lineAt(line).range, message, vscode.DiagnosticSeverity.Warning);
        diag.source = 'Terrane security markers';
        return diag;
      }));
      entry = { version: doc.version, ranges };
      cache.set(doc, entry);
    }
    const enabled = configuration(doc).get('securityRegions.decorate', true);
    decorate(doc, enabled ? entry.ranges : []);
  }
  function decorate(doc, ranges) {
    for (const editor of vscode.window.visibleTextEditors) {
      if (editor.document === doc) editor.setDecorations(decoration, ranges);
    }
  }
  const updateVisible = () => { for (const editor of vscode.window.visibleTextEditors) update(editor.document); };
  function cancel(doc) { clearTimeout(timers.get(doc)); timers.delete(doc); }
  context.subscriptions.push(diagnostics, decoration,
    vscode.workspace.onDidOpenTextDocument(update),
    vscode.workspace.onDidCloseTextDocument(doc => { cancel(doc); cache.delete(doc); diagnostics.delete(doc.uri); }),
    vscode.workspace.onDidChangeTextDocument(({ document, contentChanges }) => {
      if (!contentChanges.length || !scansMarkers(document)) return;
      cancel(document);
      timers.set(document, setTimeout(() => { timers.delete(document); update(document); }, 120));
    }),
    vscode.window.onDidChangeVisibleTextEditors(updateVisible),
    vscode.workspace.onDidChangeConfiguration(event => {
      if (event.affectsConfiguration('terrane.securityRegions.rust')) {
        for (const doc of vscode.workspace.textDocuments) {
          if (doc.languageId === 'rust') update(doc);
        }
      }
      if (event.affectsConfiguration('terrane.securityRegions.decorate')) updateVisible();
    }),
    { dispose() { for (const timer of timers.values()) clearTimeout(timer); timers.clear(); } });
  for (const doc of vscode.workspace.textDocuments) update(doc);
}

module.exports = { registerSecurityRegions };
