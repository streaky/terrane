'use strict';

const assert = require('node:assert/strict');
const { test } = require('node:test');
const { isValidId, scanMarkers } = require('../security-markers');

const scan = (lines, languageId = 'rust') => scanMarkers(lines, languageId);
const noMarkers = { regions: [], issues: [] };

function assertIssues(result, expected) {
  assert.deepEqual(result.issues.map(({ line }) => line), expected.map(([line]) => line));
  for (const [index, [, message]] of expected.entries()) assert.match(result.issues[index].message, message);
}

test('validates exactly the checker identity grammar', () => {
  for (const id of ['a', 'a1', 'a-', 'a--.b9-c', 'filesystem.beneath-open']) assert.equal(isValidId(id), true, id);
  for (const id of ['', 'A', '9a', '-a', 'a-b.9', 'a.-b', 'a..b', 'a.', '.a', 'a_b', 'a.β', 'a/b',
    ' a', 'a ', 'a\n', 'a\u2028', undefined]) assert.equal(isValidId(id), false, String(id));
});

test('pairs exact comments in opening order, including blank, adjacent, and nested lines', () => {
  const lines = [
    '\t//# SECURITY-BEGIN [fs.outer] \r',
    '    //# SECURITY-BEGIN [inner]',
    '',
    '    //# SECURITY-END [inner]\r',
    '\t//# SECURITY-END [fs.outer]\r',
    '//# SECURITY-BEGIN [next]',
    '//# SECURITY-END [next]',
  ];
  const expected = { regions: [{ start: 0, end: 4 }, { start: 1, end: 3 }, { start: 5, end: 6 }], issues: [] };
  for (const language of ['rust', 'terrane']) assert.deepEqual(scan(lines, language), expected);
});

test('distinguishes structural errors and does not decorate bad pairs', () => {
  const result = scan([
    '//# SECURITY-END [orphan]',
    'let inline = true; //# SECURITY-BEGIN [inline]',
    '//# SECURITY-BEGIN [unfinished',
    '//# SECURITY-BEGIN [extra]]',
    '//# SECURITY-BEGIN [bad id]',
    '//# SECURITY-BEGIN [valid] trailing text',
    '//# SECURITY-BEGIN [outer]',
    '//# SECURITY-BEGIN [inner]',
    '//# SECURITY-END [outer]',
  ]);
  assert.deepEqual(result.regions, []);
  assertIssues(result, [
    [0, /without begin/i], [1, /own line/i], [2, /malformed/i], [3, /identity/i], [4, /identity/i],
    [5, /malformed/i], [6, /without end/i], [7, /without end/i], [8, /crossing|mismatched/i],
  ]);
});

test('warns about marker-like comments without accepting them as regions or warning on prose', () => {
  const result = scan([
    '// Security-sensitive: validate untrusted input.',
    '/// Security-relevant helper.',
    '//! This text mentions SECURITY-END [doc].',
    '// security begins [here] after initialization.',
    '// SECURITY-BEGIN[legacy]',
    '/// SECURITY-BEGIN [doc]',
    '//! SECURITY-END [doc]',
    '//# security-begin [case]',
    '//#SECURITY-BEGIN [spacing]',
    '// # SECURITY-END[spacing]',
    '//# SECURITYBEGIN [spacing]',
    '//# SEUCRITY-BEGIN [transposed]',
    '//# SECURITY-BEGN [missing-letter]',
    '//# SECURITY-BEGIN',
    '# security-review: done',
    '# SECURITY-BEGIN [wrong-comment]',
    '//# SECURITY-BEGIN [real]',
    '//# SECURITY-END [real]',
  ], 'terrane');
  assert.deepEqual(result.regions, [{ start: 16, end: 17 }]);
  assertIssues(result, [4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 15].map(line => [line, /malformed/i]));
  assert.deepEqual(scan(['# SECURITY-BEGIN [not-a-rust-comment]']), noMarkers);
  assert.deepEqual(scan([]), noMarkers);
});

test('excludes Rust literals and nested block comments without swallowing lifetimes or labels', () => {
  const result = scan([
    "fn f<'a>(value: &'a str) { 'label: loop { break 'label; }",
    'let a = r##"first',
    '//# SECURITY-BEGIN [hidden]',
    '"# is not the end',
    '//# SECURITY-BEGN [hidden]',
    '"##;',
    'let b = br"//# SECURITY-END [hidden]";',
    'let c = cr#"',
    '//# security-begin [hidden]',
    '"#;',
    'let d = c"escaped \\" and',
    '// # SECURITY-END [hidden]',
    '";',
    String.raw`let e = (b'/', '\u{1f600}', '\x22', '💡');`,
    '/* outer /* nested */',
    '//# SECURITY-BEGIN [hidden]',
    '// SECURITY-BEGIN [hidden]',
    '*/',
    '//# SECURITY-BEGIN [real]',
    '}',
    '//# SECURITY-END [real]',
  ]);
  assert.deepEqual(result, { regions: [{ start: 18, end: 20 }], issues: [] });
});

test('excludes Terrane single, interpolated, inline-tail and indented text', () => {
  const result = scan([
    'text = >text " /* //# SECURITY-BEGIN [hidden]',
    'block = >>',
    '',
    '    //# SECURITY-BEGIN [hidden]',
    '    //# SECURITY-BEGN [hidden]',
    'literal = \'first',
    '//# SECURITY-END [hidden]',
    "last'",
    'text = "escaped {{ {value.concat; "nested {value',
    '# SECURITY-BEGIN [hidden]',
    '// # SECURITY-BEGIN [hidden]',
    '/* } */',
    '}"}',
    '//# SECURITY-BEGIN [hidden]',
    'last"',
    '/* outer /* first terminator */',
    '//# SECURITY-BEGIN [real]',
    "value = '*/'",
    '//# SECURITY-END [real]',
  ], 'terrane');
  assert.deepEqual(result, { regions: [{ start: 16, end: 18 }], issues: [] });
});

test('uses checker native block headers and resumes at dedentation outside skipped tokens', () => {
  for (const header of ['rust', 'unsafe rust', 'return rust', 'return unsafe rust',
    'value int = rust', 'value int = unsafe rust', 'rust # native', 'rust // native',
    'return /* note */ unsafe rust']) {
    const result = scan([
      `  ${header}`,
      '    let raw = r#"',
      '//# SECURITY-BEGIN [hidden-at-dedent]',
      '//# SECURITY-BEGN [hidden-at-dedent]',
      '"#;',
      '    /* outer /* nested */',
      '    //# SECURITY-BEGIN [hidden-in-native-block-comment]',
      '    */',
      '  //# SECURITY-BEGIN [real]',
      '  //# SECURITY-END [real]',
    ], 'terrane');
    assert.deepEqual(result, { regions: [{ start: 8, end: 9 }], issues: [] }, header);
  }
});

test('does not enter native mode for ordinary rust identifiers or comparisons', () => {
  for (const header of ['value = options.rust', 'value == rust', 'value <= rust', 'rust + other']) {
    assert.deepEqual(scan([
      header,
      "    'literal",
      '    //# SECURITY-BEGIN [hidden]',
      "    end'",
      '//# SECURITY-BEGIN [real]',
      '//# SECURITY-END [real]',
    ], 'terrane'), { regions: [{ start: 4, end: 5 }], issues: [] }, header);
  }
});

test('unfinished skipped constructs do not throw or emit marker warnings for their contents', () => {
  for (const [language, opening] of [['rust', '/*'], ['rust', 'let raw = br#"'],
    ['rust', 'let s = "'], ['terrane', 'text = "{value'], ['terrane', "text = '"]]) {
    assert.deepEqual(scan([opening, '//# SECURITY-BEGIN [hidden]', '// # SECURITY-BEGN [hidden]'], language), noMarkers);
  }
});
