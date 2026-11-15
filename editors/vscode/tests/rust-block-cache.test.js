'use strict';

const assert = require('node:assert/strict');
const { before, after, test } = require('node:test');
const { createRustBlockScanner } = require('../rust-blocks');

let scanner;
before(async () => { scanner = await createRustBlockScanner(); });
after(() => scanner?.dispose());

test('cached sections track insertion, deletion, indentation and header edits', () => {
  const document = {};
  const update = (text, expected) => assert.deepEqual(scanner.findBlocks(text.split('\n'), document), expected);
  update('rust\n  42\nreturn none', [{ start: 0, end: 1 }]);
  update('# inserted\nrust\n  42\nreturn none', [{ start: 1, end: 2 }]);
  update('# inserted\nrust\n42\nreturn none', [{ start: 1, end: 1 }]);
  update('# inserted\nunsafe rust\n  42\nreturn none', [{ start: 1, end: 2 }]);
  update('return none', []);
  update('', []);
  update('return rust\n  43', [{ start: 0, end: 1 }]);
});

test('changed comment state propagates through unchanged Rust-looking lines', () => {
  const document = {};
  for (const [text, expected] of [
    ['# comment\nrust\n  42\n# end\nrust\n  43', [{ start: 1, end: 2 }, { start: 4, end: 5 }]],
    ['/* comment\nrust\n  42\n# end\nrust\n  43', []],
    ['/* comment\nrust\n  42\n*/\nrust\n  43', [{ start: 4, end: 5 }]],
    ['# comment\nrust\n  42\n# end\nrust\n  43', [{ start: 1, end: 2 }, { start: 4, end: 5 }]],
  ]) assert.deepEqual(scanner.findBlocks(text.split('\n'), document), expected);
});

test('document caches remain independent across interleaved edits', () => {
  const first = {};
  const second = {};
  assert.deepEqual(scanner.findBlocks(['rust', '  42'], first), [{ start: 0, end: 1 }]);
  assert.deepEqual(scanner.findBlocks(['/*', 'rust', '  42'], second), []);
  assert.deepEqual(scanner.findBlocks(['rust', '  43'], first), [{ start: 0, end: 1 }]);
  assert.deepEqual(scanner.findBlocks(['#', 'rust', '  42'], second), [{ start: 1, end: 2 }]);
});

test('incremental results match fresh tokenization after composed multiline edits', () => {
  const document = {};
  let lines = ['function main;', '  rust', '    42', '  return none', '/*', 'rust', '*/'];
  const replacements = ['rust', '  unsafe rust', '/*', '*/', "value = 'unfinished", '  return >>', '    body', ''];
  for (let step = 0; step < 64; step++) {
    const at = (step * 7) % (lines.length + 1);
    const count = step % 3 === 0 ? 0 : Math.min(2, lines.length - at);
    lines.splice(at, count, replacements[step % replacements.length]);
    assert.deepEqual(scanner.findBlocks(lines, document), scanner.findBlocks(lines), `edit ${step}`);
  }
});
