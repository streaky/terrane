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

  return {
    findBlocks(lines) {
      const blocks = [];
      let state = INITIAL;
      let block;
      let lineNumber = 0;
      for (const text of lines) {
        const result = grammar.tokenizeLine(text, state);
        state = result.ruleStack;
        const inRust = result.tokens.some(token => token.scopes.includes('meta.rust-block.terrane'));
        const inBody = result.tokens.some(token => token.scopes.includes('meta.embedded.block.rust'));
        if (block && (!inRust || !inBody)) {
          blocks.push(block);
          block = undefined;
        }
        if (inRust) {
          if (!block) block = { start: lineNumber, end: lineNumber };
          // Keep internal blank lines, but do not shade separators after the body.
          if (text.trim()) block.end = lineNumber;
        }
        lineNumber++;
      }
      if (block) blocks.push(block);
      return blocks;
    },
    dispose() {
      registry.dispose();
    },
  };
}

module.exports = { createRustBlockScanner };
