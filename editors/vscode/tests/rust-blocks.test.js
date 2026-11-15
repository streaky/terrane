'use strict';

const assert = require('node:assert/strict');
const { before, after, test } = require('node:test');
const { createRustBlockScanner } = require('../rust-blocks');

let scanner;
before(async () => { scanner = await createRustBlockScanner(); });
after(() => { scanner?.dispose(); });

const blocks = text => scanner.findBlocks(text.split('\n'));

test('includes each supported Rust header and its body, not the next Terrane line', () => {
  for (const header of ['rust', 'unsafe rust', 'return rust', 'return unsafe rust',
    'value int = rust', 'value int = unsafe rust', 'rust # native', 'rust // native']) {
    assert.deepEqual(blocks(`function main;\n  ${header}\n    42\n  print; 42`), [{ start: 1, end: 2 }]);
  }
});

test('does not merge adjacent Rust sections at the same indentation', () => {
  assert.deepEqual(blocks('  rust\n    first();\n  unsafe rust\n    second();'), [
    { start: 0, end: 1 }, { start: 2, end: 3 },
  ]);
});

test('keeps interior blank lines but excludes trailing blank separators', () => {
  assert.deepEqual(blocks('  rust\n    first();\n\n    second();\n\n  \n  return'), [
    { start: 0, end: 3 },
  ]);
});

test('uses structural tab indentation and handles a body ending at EOF', () => {
  assert.deepEqual(blocks('\tif true\n\t\trust\n\t\t\t42\n\t\treturn\n\trust\n\t\t43'), [
    { start: 1, end: 2 }, { start: 4, end: 5 },
  ]);
});

test('does not decorate apparent Rust headers inside Terrane comments or text', () => {
  for (const text of [
    '# value int = rust\n  42',
    '  // value int = rust\n    42',
    '/*\n  rust\n    42\n*/',
    "value = 'native\n  rust\n    42'",
    'value = >>\n  rust\n    42',
    'return >>\n  rust\n    42',
    '>value int = rust\n  42',
    'value = >value int = rust\n  42',
  ]) assert.deepEqual(blocks(text), [], text);
});

test('Rust comments, raw strings and nested indentation cannot start extra sections', () => {
  assert.deepEqual(blocks('rust\n  /*\n    rust\n  */\n  let text = r#"\n    unsafe rust\n  "#;\nreturn'), [
    { start: 0, end: 6 },
  ]);
});

test('dedentation ends the region even while typing an unfinished native string', () => {
  assert.deepEqual(blocks('  rust\n    let text = "unfinished\n  return none'), [{ start: 0, end: 1 }]);
});

test('rescanning after edits does not retain deleted regions or previous indentation state', () => {
  assert.deepEqual(blocks('rust\n  42'), [{ start: 0, end: 1 }]);
  assert.deepEqual(blocks('print; 42'), []);
  assert.deepEqual(blocks('function main;\n  rust\n    42'), [{ start: 1, end: 2 }]);
});
