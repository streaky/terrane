'use strict';

const vscode = require('vscode');
const { createRustBlockScanner } = require('./rust-blocks');

function* documentLines(document) {
  for (let line = 0; line < document.lineCount; line++) {
    yield document.lineAt(line).text;
  }
}

async function createRustBlockDecorations() {
  const scanner = await createRustBlockScanner();
  const decoration = vscode.window.createTextEditorDecorationType({
    isWholeLine: true,
    backgroundColor: new vscode.ThemeColor('terrane.rustBlockBackground'),
    borderColor: new vscode.ThemeColor('terrane.rustBlockBorder'),
    borderStyle: 'solid',
    borderWidth: '0 0 0 2px',
    rangeBehavior: vscode.DecorationRangeBehavior.ClosedClosed,
  });
  const cache = new WeakMap();
  let timer;

  function refresh() {
    clearTimeout(timer);
    for (const editor of vscode.window.visibleTextEditors) {
      const document = editor.document;
      const enabled = document.languageId === 'terrane'
        && vscode.workspace.getConfiguration('terrane', document.uri).get('rustBlocks.decorate', true);
      if (!enabled) {
        editor.setDecorations(decoration, []);
        continue;
      }
      let entry = cache.get(document);
      if (!entry || entry.version !== document.version) {
        const ranges = scanner.findBlocks(documentLines(document)).map(block => new vscode.Range(
          block.start, 0, block.end, document.lineAt(block.end).text.length,
        ));
        entry = { version: document.version, ranges };
        cache.set(document, entry);
      }
      editor.setDecorations(decoration, entry.ranges);
    }
  }

  const subscriptions = [
    vscode.window.onDidChangeVisibleTextEditors(refresh),
    vscode.workspace.onDidOpenTextDocument(refresh),
    vscode.workspace.onDidChangeTextDocument(event => {
      if (vscode.window.visibleTextEditors.some(editor => editor.document === event.document)) {
        clearTimeout(timer);
        timer = setTimeout(refresh, 100);
      }
    }),
    vscode.workspace.onDidChangeConfiguration(event => {
      if (event.affectsConfiguration('terrane.rustBlocks.decorate')) refresh();
    }),
  ];
  refresh();
  return new vscode.Disposable(() => {
    clearTimeout(timer);
    for (const subscription of subscriptions) subscription.dispose();
    decoration.dispose();
    scanner.dispose();
  });
}

module.exports = { createRustBlockDecorations };
