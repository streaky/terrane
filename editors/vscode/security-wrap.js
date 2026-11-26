'use strict';

const { isValidId } = require('./security-markers');

function registerSecurityWrap(vscode) {
  const { window } = vscode;
  return vscode.commands.registerCommand('terrane.wrapSecurityRegion', async () => {
    const editor = window.activeTextEditor;
    const document = editor?.document;
    if (!document || (document.languageId !== 'rust' && document.languageId !== 'terrane')) {
      void window.showWarningMessage('Open a Rust or Terrane document to wrap a security region.');
      return;
    }
    if (editor.selections.length !== 1 || editor.selection.isEmpty) {
      void window.showWarningMessage('Select one non-empty region to wrap with security markers.');
      return;
    }

    const selection = editor.selection;
    const version = document.version;
    const languageId = document.languageId;
    const id = await window.showInputBox({
      prompt: 'Security contract ID',
      ignoreFocusOut: true,
      placeHolder: 'filesystem.beneath-open',
      validateInput: value => isValidId(value) ? undefined : 'Enter a valid security contract ID.',
    });
    if (id === undefined) return;
    if (!isValidId(id)) {
      void window.showWarningMessage('Invalid security contract ID.');
      return;
    }
    if (window.activeTextEditor !== editor || editor.document !== document ||
        document.isClosed || document.version !== version || document.languageId !== languageId ||
        editor.selections.length !== 1 || !selection.anchor.isEqual(editor.selection.anchor) ||
        !selection.active.isEqual(editor.selection.active)) {
      void window.showWarningMessage('The document or selection changed. Select the region and try again.');
      return;
    }

    const first = document.lineAt(selection.start.line);
    // An exclusive endpoint at column zero belongs to the next, unselected line.
    const last = document.lineAt(selection.end.line - (selection.end.character === 0 ? 1 : 0));
    const indent = /^[\t ]*/.exec(first.text)[0];
    const eol = document.eol === vscode.EndOfLine.CRLF ? '\r\n' : '\n';
    const begin = `${indent}//# SECURITY-BEGIN [${id}]`;
    const end = `${indent}//# SECURITY-END [${id}]`;
    const hasFollowingLine = last.lineNumber + 1 < document.lineCount;
    const edited = await editor.edit(edit => {
      edit.insert(first.range.start, begin + eol);
      if (hasFollowingLine) {
        edit.insert(document.lineAt(last.lineNumber + 1).range.start, end + eol);
      } else {
        // Keep an unterminated final line unterminated after wrapping it.
        edit.insert(last.range.end, eol + end);
      }
    });
    if (!edited) void window.showWarningMessage('Could not apply the security markers.');
  });
}

module.exports = { registerSecurityWrap };
