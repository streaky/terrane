'use strict';

const fs = require('fs/promises');
const path = require('path');
const { Registry, INITIAL } = require('vscode-textmate');
const oniguruma = require('vscode-oniguruma');

async function createRustBlockScanner() {
  const wasm = await fs.readFile(require.resolve('vscode-oniguruma/release/onig.wasm'));
  await oniguruma.loadWASM(wasm.buffer.slice(wasm.byteOffset, wasm.byteOffset + wasm.byteLength));
  const grammarPath = path.join(__dirname, 'syntaxes', 'terrane.tmLanguage.json');
  const rawGrammar = JSON.parse(await fs.readFile(grammarPath, 'utf8'));
  // Only the containing regions matter. Explicitly omit native tokenization:
  // an unresolved source.rust include would make TextMate discard the whole rule.
  rawGrammar.repository['rust-block'].patterns = [];
  const registry = new Registry({
    onigLib: Promise.resolve({
      createOnigScanner: patterns => new oniguruma.OnigScanner(patterns),
      createOnigString: text => new oniguruma.OnigString(text),
    }),
    loadGrammar: async scope => scope === 'source.terrane' ? rawGrammar : null,
  });
  const grammar = await registry.loadGrammar('source.terrane');
  const documents = new WeakMap();

  return {
    findBlocks(lines, document) {
      const text = Array.from(lines);
      const previous = document ? documents.get(document) : undefined;
      let first = 0;
      let suffix = 0;
      if (previous) {
        while (first < text.length && first < previous.lines.length
          && text[first] === previous.lines[first].text) first++;
        if (first === text.length && first === previous.lines.length) return previous.blocks;
        while (suffix < text.length - first && suffix < previous.lines.length - first
          && text[text.length - suffix - 1] === previous.lines[previous.lines.length - suffix - 1].text) suffix++;
      }

      let states = previous ? previous.lines.slice(0, first) : [];
      let state = first ? states[first - 1].state : INITIAL;
      for (let line = first; line < text.length; line++) {
        const oldLine = previous ? line + previous.lines.length - text.length : -1;
        if (previous && line >= text.length - suffix) {
          const oldState = oldLine ? previous.lines[oldLine - 1].state : INITIAL;
          if (state.equals(oldState)) {
            // The remaining text and incoming grammar state agree. Reuse the tail,
            // including after line insertions/deletions, without retokenizing it.
            states = states.concat(previous.lines.slice(oldLine));
            break;
          }
        }
        const result = grammar.tokenizeLine(text[line], state);
        state = result.ruleStack;
        states.push({
          text: text[line],
          state,
          inRust: result.tokens.some(token => token.scopes.includes('meta.rust-block.terrane')),
          inBody: result.tokens.some(token => token.scopes.includes('meta.embedded.block.rust')),
        });
      }

      const blocks = [];
      let block;
      for (let lineNumber = 0; lineNumber < states.length; lineNumber++) {
        const { text: line, inRust, inBody } = states[lineNumber];
        if (block && (!inRust || !inBody)) {
          blocks.push(block);
          block = undefined;
        }
        if (inRust) {
          if (!block) block = { start: lineNumber, end: lineNumber };
          // Keep internal blank lines, but do not shade separators after the body.
          if (line.trim()) block.end = lineNumber;
        }
      }
      if (block) blocks.push(block);
      if (document) documents.set(document, { lines: states, blocks });
      return blocks;
    },
    dispose() {
      registry.dispose();
    },
  };
}

module.exports = { createRustBlockScanner };
