'use strict';

const assert = require('node:assert/strict');
const fs = require('node:fs/promises');
const path = require('node:path');
const { before, after, test } = require('node:test');
const { Registry, INITIAL } = require('vscode-textmate');
const oniguruma = require('vscode-oniguruma');

let registry;
let grammar;
before(async () => {
  const wasm = await fs.readFile(require.resolve('vscode-oniguruma/release/onig.wasm'));
  await oniguruma.loadWASM(wasm.buffer.slice(wasm.byteOffset, wasm.byteOffset + wasm.byteLength));
  const raw = JSON.parse(await fs.readFile(path.join(__dirname, '../syntaxes/terrane.tmLanguage.json'), 'utf8'));
  raw.repository['rust-block'].patterns = [];
  registry = new Registry({
    onigLib: Promise.resolve({
      createOnigScanner: patterns => new oniguruma.OnigScanner(patterns),
      createOnigString: text => new oniguruma.OnigString(text),
    }),
    loadGrammar: async scope => scope === 'source.terrane' ? raw : null,
  });
  grammar = await registry.loadGrammar('source.terrane');
});
after(() => registry?.dispose());

function tokenize(text) {
  let state = INITIAL;
  return text.split('\n').map(line => {
    const result = grammar.tokenizeLine(line, state);
    state = result.ruleStack;
    return { line, tokens: result.tokens };
  });
}
function scopesAt(row, text) {
  const start = row.line.indexOf(text);
  assert(start >= 0, `Missing ${text}`);
  return row.tokens.find(token => token.startIndex <= start && token.endIndex > start).scopes;
}
const has = (row, text, scope) => assert(scopesAt(row, text).includes(scope), `${row.line}: ${text} must have ${scope}`);
const lacks = (row, text, scope) => assert(!scopesAt(row, text).includes(scope), `${row.line}: ${text} must not have ${scope}`);

test('joined identifier runs are not operators but postfix updates remain operators', () => {
  for (const name of ['copy--of', 'copy++able', 'a---b', 'a-++b', 'a++--b']) {
    const [row] = tokenize(name);
    assert(!row.tokens.some(token => token.scopes.includes('keyword.operator.terrane')), name);
  }
  for (const update of ['x++', 'count--', '(count)++']) {
    const [row] = tokenize(update);
    has(row, update.endsWith('++') ? '++' : '--', 'keyword.operator.terrane');
  }
});

test('contextual words retain identifier highlighting outside their syntactic roles', () => {
  for (const word of ['enum', 'owning', 'consuming', 'associated']) {
    const [row] = tokenize(`${word} = 3`);
    lacks(row, word, 'keyword.control.terrane');
  }
  for (const [text, word] of [['enum status', 'enum'], ['owning interface resource', 'owning'],
    ['consuming async function close;', 'consuming'], ['associated type Item', 'associated']]) {
    has(tokenize(text)[0], word, 'keyword.control.terrane');
  }
  for (const word of ['true', 'false', 'none']) has(tokenize(`value = ${word}`)[0], word, 'constant.language.terrane');
});

test('block text starts after complete expression prefixes and dedentation ends it', () => {
  for (const header of ['>>', 'value = >>', 'return >>', 'x = a + >>', "call; 'a', >>",
    "call; 'a/>b', >>", 'value = [ >>', 'value = { >>', 'value = /* note */ >>']) {
    const rows = tokenize(`${header}\n  rust\n    opaque = rust\nreturn none`);
    has(rows[1], 'rust', 'string.unquoted.block.terrane');
    has(rows[2], 'rust', 'string.unquoted.block.terrane');
    lacks(rows[1], 'rust', 'meta.rust-block.terrane');
    lacks(rows[3], 'return', 'string.unquoted.block.terrane');
  }
});

test('pipe and colon tokens do not introduce block-text bodies', () => {
  for (const prefix of ['|', ':', '::']) {
    const rows = tokenize(`value = a ${prefix} >>\n  body\nreturn none`);
    lacks(rows[1], 'body', 'string.unquoted.block.terrane');
    lacks(rows[1], 'body', 'meta.embedded.block.rust');
  }
});

test('tail text follows expression-start delimiters and operator tokens', () => {
  for (const line of ['>tail', '  >tail', 'return >tail', 'value = >tail',
    'value = a + >tail', 'value = a+ >tail', 'value = a == >tail',
    'call; >tail', "call; 'first', >tail", 'value = (>tail', 'value = [>tail',
    'value = {>tail', 'value = a + /* note */ >tail', 'return/* note */>tail']) {
    const rows = tokenize(`${line}\nreturn none`);
    has(rows[0], '>tail', 'string.unquoted.tail.terrane');
    lacks(rows[1], 'return', 'string.unquoted.tail.terrane');
  }
});

test('tail text does not start after pipe/colon or inside opaque text and identifiers', () => {
  for (const line of ['value = a | >tail', 'value = a : >tail', 'value = a :: >tail',
    'value = a+>tail', '# value = a + >tail', '// value = a + >tail',
    "value = 'a + >tail'", '/* value = a + >tail */']) {
    lacks(tokenize(line)[0], '>tail', 'string.unquoted.tail.terrane');
  }
});

test('tail text may immediately follow a distinct operator token', () => {
  for (const line of ['value = a +>tail', 'value = 3 +>tail', 'value = 3+>tail',
    'value = 3.5+>tail', 'value = 0xff+>tail', "value = 'b' <>tail",
    "value = 'b' ->tail", 'value = a!>tail', 'value = a&>tail',
    'value = a <<>tail', 'value = a >=>tail', 'value = a >> >tail']) {
    const rows = tokenize(`${line}\nreturn none`);
    has(rows[0], '>tail', 'string.unquoted.tail.terrane');
    lacks(rows[1], 'return', 'string.unquoted.tail.terrane');
  }
});

test('adjacency cannot split joined names, update tokens or shift operands into text', () => {
  for (const line of ['value = a+>tail', 'value = a+++>tail', 'value = a-++>tail',
    'value = a<<>tail', 'value = 3++>tail', 'value = a ++>tail', 'value = 3-->tail',
    'value = a -->tail', 'value = a >>tail', 'value = a >tail']) {
    lacks(tokenize(line)[0], '>tail', 'string.unquoted.tail.terrane');
  }
});

test('many inline comments retain native headers and ordinary expression scopes', () => {
  const comments = '/* note ** still comment */ '.repeat(60);
  const rows = tokenize(`value int = ${comments}rust\n  42\nreturn none`);
  has(rows[0], 'rust', 'meta.rust-block.terrane');
  has(rows[1], '42', 'meta.embedded.block.rust');
  lacks(rows[2], 'return', 'meta.embedded.block.rust');
  const ordinary = tokenize(`value = ${comments}false\n  body`);
  has(ordinary[0], 'false', 'constant.language.terrane');
  lacks(ordinary[1], 'body', 'meta.embedded.block.rust');
  const text = tokenize(`value = ${comments}>>\n  text body\nreturn none`);
  has(text[1], 'text', 'string.unquoted.block.terrane');
});

test('quoted arguments are preserved in block-text and Rust initializer headers', () => {
  has(tokenize("call; 'first', >>\n  body")[0], 'first', 'string.quoted.single.terrane');
  const rows = tokenize("@[option; '--value'] value string = rust /* note */\n  String::new()\nreturn none");
  has(rows[0], '--value', 'string.quoted.single.terrane');
  has(rows[0], 'rust', 'keyword.control.terrane');
  has(rows[0], 'note', 'comment.block.terrane');
  has(rows[1], 'String', 'meta.embedded.block.rust');
  lacks(rows[2], 'return', 'meta.embedded.block.rust');
});

test('double strings tokenize code interpolation, nested quotes, calls, selectors and brace escapes', () => {
  const row = tokenize(`value = "hello {{literal}} {render(user.name, "inner {count}")}"`)[0];
  has(row, 'hello', 'string.quoted.double.terrane');
  has(row, '{{', 'constant.character.escape.terrane');
  has(row, 'render', 'meta.interpolation.terrane');
  has(row, 'user', 'meta.interpolation.terrane');
  has(row, 'name', 'meta.interpolation.terrane');
  has(row, 'inner', 'string.quoted.double.terrane');
  has(row, 'count', 'meta.interpolation.terrane');
});
test('interpolation selector marker and key have separate scopes', () => {
  const row = tokenize('value = "x {(render; user) !: plain}"')[0];
  has(row, '!: ', 'keyword.operator.terrane');
  has(row, 'plain', 'variable.other.interpolation-key.terrane');
  lacks(tokenize('value = "x {!: plain}"')[0], 'plain', 'variable.other.interpolation-key.terrane');
});

test('raw single strings only escape quote and backslash', () => {
  const row = tokenize(String.raw`value = 'raw \n \' \\ \q'`)[0];
  has(row, String.raw`\n`, 'string.quoted.single.terrane');
  lacks(row, String.raw`\n`, 'constant.character.escape.terrane');
  has(row, String.raw`\'`, 'constant.character.escape.terrane');
  has(row, String.raw`\\`, 'constant.character.escape.terrane');
  lacks(row, String.raw`\q`, 'constant.character.escape.terrane');
});

test('double escapes, interpolation-adjacent braces, and malformed boundaries recover', () => {
  const row = tokenize(String.raw`value = "\n \q \" {item}}} {{tail}}"`)[0];
  has(row, String.raw`\n`, 'constant.character.escape.terrane');
  lacks(row, String.raw`\q`, 'constant.character.escape.terrane');
  has(row, String.raw`\q`, 'invalid.illegal.escape.terrane');
  has(row, String.raw`\"`, 'constant.character.escape.terrane');
  has(row, '{item}', 'meta.interpolation.terrane');
  has(row, '{{tail}}', 'string.quoted.double.terrane');
  const close = row.line.indexOf('}}}');
  const scopesAtOffset = offset => row.tokens.find(token => token.startIndex <= offset && token.endIndex > offset).scopes;
  assert(scopesAtOffset(close).includes('punctuation.section.interpolation.end.terrane'));
  assert(scopesAtOffset(close + 1).includes('constant.character.escape.terrane'));
  assert(!scopesAtOffset(close + 1).includes('meta.interpolation.terrane'));
  const evenBackslashes = tokenize(String.raw`value = "slash\\" + outside`)[0];
  has(evenBackslashes, '+', 'keyword.operator.terrane');
  lacks(evenBackslashes, 'outside', 'string.quoted.double.terrane');
  const oddBackslashes = tokenize(String.raw`value = "slash\" still string" + outside`)[0];
  has(oddBackslashes, 'still string', 'string.quoted.double.terrane');
  has(oddBackslashes, '+', 'keyword.operator.terrane');
  const malformed = tokenize('value = "{call("inner")}"')[0];
  has(malformed, 'inner', 'string.quoted.double.terrane');
  const following = tokenize('value = "{unfinished\nreturn none');
  lacks(following[1], 'return', 'meta.interpolation.terrane');
});

test('byte escapes and unicode escapes stay inside string scopes', () => {
  const row = tokenize(String.raw`value = b'\n\r\t\0\xAF'`)[0];
  for (const escape of [String.raw`\n`, String.raw`\r`, String.raw`\t`, String.raw`\0`, String.raw`\xAF`]) {
    has(row, escape, 'constant.character.escape.terrane');
  }
  const unicode = tokenize(String.raw`value = "\u{1F600}"`)[0];
  has(unicode, String.raw`\u{1F600}`, 'constant.character.escape.terrane');
  lacks(unicode, '{1F600}', 'meta.interpolation.terrane');
});

test('unknown double escapes and unmatched closing braces are invalid, not raw text', () => {
  const double = tokenize(String.raw`value = "\q \x41 lone} {{literal}} {item}"`)[0];
  has(double, String.raw`\q`, 'invalid.illegal.escape.terrane');
  has(double, String.raw`\x`, 'invalid.illegal.escape.terrane');
  has(double, '}', 'invalid.illegal.brace.terrane');
  lacks(double, '{{', 'invalid.illegal.brace.terrane');
  has(double, '{item}', 'meta.interpolation.terrane');
  const raw = tokenize(String.raw`value = '\q \x41 }'`)[0];
  lacks(raw, String.raw`\q`, 'invalid.illegal.escape.terrane');
  lacks(raw, '}', 'invalid.illegal.brace.terrane');
});
test('interpolation key marker is not scoped in ordinary code', () => {
  lacks(tokenize('value = user !: plain')[0], '!: ', 'variable.other.interpolation-key.terrane');
});


test('unfinished Terrane text ends at the physical line boundary', () => {
  const rows = tokenize("value = 'unfinished\nrust\n  42\nreturn none");
  has(rows[0], 'unfinished', 'string.quoted.single.terrane');
  lacks(rows[1], 'rust', 'string.quoted.single.terrane');
  has(rows[1], 'rust', 'meta.rust-block.terrane');
  has(rows[2], '42', 'meta.embedded.block.rust');
});

test('comments and tail text cannot introduce block text or native regions', () => {
  for (const header of ['# value = >>', '// value = >>', '> value = >>', 'return >value = >>',
    'value = >text = >>', '# value = rust', '// value = rust', '> value = rust', 'value = >text = rust',
    '  >value = rust', '  return >value = >>', '  >value = >>']) {
    const rows = tokenize(`${header}\n    body`);
    lacks(rows[1], 'body', 'string.unquoted.block.terrane');
    lacks(rows[1], 'body', 'meta.embedded.block.rust');
  }
});
