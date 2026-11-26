'use strict';

const test = require('node:test');
const assert = require('node:assert/strict');
const { registerSecurityWrap } = require('../security-wrap');

const id = 'filesystem.beneath-open';
const begin = `//# SECURITY-BEGIN [${id}]`;
const end = `//# SECURITY-END [${id}]`;

function position(line, character) {
  return { line, character, isEqual: other => line === other.line && character === other.character };
}

function selection(anchor, active) {
  anchor = position(...anchor);
  active = position(...active);
  const reverse = anchor.line > active.line ||
    (anchor.line === active.line && anchor.character > active.character);
  return {
    anchor, active, isEmpty: anchor.isEqual(active),
    start: reverse ? active : anchor, end: reverse ? anchor : active,
  };
}

function harness(text, range, { languageId = 'rust', answer = id, editResult = true } = {}) {
  const eol = text.includes('\r\n') ? '\r\n' : '\n';
  const lines = text.split(eol);
  const document = {
    version: 1, languageId, isClosed: false, eol: eol === '\r\n' ? 2 : 1, lineCount: lines.length,
    lineAt(lineNumber) {
      const text = lines[lineNumber];
      return { lineNumber, text, range: { start: position(lineNumber, 0), end: position(lineNumber, text.length) } };
    },
  };
  let edits = 0;
  const editor = {
    document, selections: [range], get selection() { return this.selections[0]; },
    async edit(callback) {
      edits++;
      const inserts = [];
      callback({ insert({ line, character }, value) {
        const offset = lines.slice(0, line).reduce((offset, prefix) => offset + prefix.length + eol.length, character);
        inserts.push({ offset, value });
      } });
      if (editResult) {
        for (const { offset, value } of inserts.sort((a, b) => b.offset - a.offset)) {
          text = text.slice(0, offset) + value + text.slice(offset);
        }
      }
      return editResult;
    },
  };
  let command, promptCount = 0;
  const warnings = [];
  const disposable = { dispose() {} };
  const window = {
    activeTextEditor: editor,
    showInputBox() { promptCount++; return typeof answer === 'function' ? answer() : answer; },
    showWarningMessage(warning) { warnings.push(warning); },
  };
  const registration = registerSecurityWrap({
    window, EndOfLine: { LF: 1, CRLF: 2 },
    commands: { registerCommand(name, callback) {
      assert.equal(name, 'terrane.wrapSecurityRegion');
      command = callback;
      return disposable;
    } },
  });
  assert.equal(registration, disposable);
  return { window, editor, document, command, warnings,
    get text() { return text; }, get promptCount() { return promptCount; }, get edits() { return edits; } };
}

test('wraps complete lines for a reversed partial CRLF selection', async () => {
  const fixture = harness('outside\r\n\t  first 😀\r\n    second\r\nnext\r\n', selection([3, 0], [1, 5]), { languageId: 'terrane' });
  await fixture.command();
  assert.equal(fixture.text, `outside\r\n\t  ${begin}\r\n\t  first 😀\r\n    second\r\n\t  ${end}\r\nnext\r\n`);
  assert.equal(fixture.promptCount, 1);
  assert.equal(fixture.edits, 1);
  assert.deepEqual(fixture.warnings, []);
});

test('wraps one line without including the next line at column zero', async () => {
  const fixture = harness('  code\nother', selection([0, 0], [1, 0]));
  await fixture.command();
  assert.equal(fixture.text, `  ${begin}\n  code\n  ${end}\nother`);
});

test('wraps the final line without changing whether EOF has a newline', async () => {
  for (const trailing of ['', '\n', '\n\n']) {
    const fixture = harness(`  code  ${trailing}`, selection([0, 3], [0, 5]));
    await fixture.command();
    assert.equal(fixture.text, `  ${begin}\n  code  \n  ${end}${trailing}`);
  }
});

test('rejects empty or multiple selections and unsupported editors before prompting', async () => {
  for (const setup of [
    fixture => { fixture.editor.selections = [selection([0, 1], [0, 1])]; },
    fixture => { fixture.editor.selections.push(selection([0, 2], [0, 3])); },
    fixture => { fixture.document.languageId = 'plaintext'; },
    fixture => { fixture.window.activeTextEditor = undefined; },
  ]) {
    const fixture = harness('code', selection([0, 0], [0, 1]));
    setup(fixture);
    await fixture.command();
    assert.equal(fixture.promptCount, 0);
    assert.equal(fixture.edits, 0);
    assert.equal(fixture.warnings.length, 1);
  }
});

test('prompts once and does not edit on cancellation or invalid input', async () => {
  for (const answer of [() => undefined, '', 'Invalid', ` ${id}`, `${id}\n`, `${id}]`]) {
    const fixture = harness('code', selection([0, 0], [0, 1]), { answer });
    await fixture.command();
    assert.equal(fixture.promptCount, 1);
    assert.equal(fixture.edits, 0);
    assert.equal(fixture.warnings.length, typeof answer === 'function' ? 0 : 1);
  }
});

test('does not apply a pending prompt to a changed editor, document or selection', async () => {
  for (const change of [
    fixture => { fixture.window.activeTextEditor = { ...fixture.editor }; },
    fixture => { fixture.editor.document = { ...fixture.document }; },
    fixture => { fixture.document.isClosed = true; },
    fixture => { fixture.document.version++; },
    fixture => { fixture.document.languageId = 'terrane'; },
    fixture => { fixture.editor.selections = [selection([0, 1], [0, 0])]; },
    fixture => { fixture.editor.selections = [selection([0, 0], [0, 2])]; },
    fixture => { fixture.editor.selections.push(selection([0, 2], [0, 3])); },
  ]) {
    let answer;
    const fixture = harness('code', selection([0, 0], [0, 1]), {
      answer: () => new Promise(resolve => { answer = resolve; }),
    });
    const pending = fixture.command();
    change(fixture);
    answer(id);
    await pending;
    assert.equal(fixture.edits, 0);
    assert.match(fixture.warnings[0], /changed/);
  }
});

test('reports when the editor declines the edit', async () => {
  const fixture = harness('code', selection([0, 0], [0, 1]), { editResult: false });
  await fixture.command();
  assert.equal(fixture.text, 'code');
  assert.equal(fixture.edits, 1);
  assert.match(fixture.warnings[0], /Could not apply/);
});
