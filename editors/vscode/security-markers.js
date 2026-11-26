'use strict';

const beginPrefix = '//# SECURITY-BEGIN [';
const endPrefix = '//# SECURITY-END [';
const leadingSpace = /^\p{White_Space}*/u;
const trailingSpace = /\p{White_Space}+$/u;

// Mirrors valid-identity in tools/security-contracts/src/model.trn.
function isValidId(value) {
  return typeof value === 'string' && /^[a-z][a-z0-9-]*(?:\.[a-z][a-z0-9-]*)*(?![\s\S])/.test(value);
}

function isWord(source, index) {
  const code = source.charCodeAt(index);
  return code >= 128 || code === 95 || (code >= 48 && code <= 57)
    || (code >= 65 && code <= 90) || (code >= 97 && code <= 122);
}

function blockEnd(source, start, nested) {
  let depth = 1;
  for (let i = start + 2; i < source.length; i++) {
    if (source.startsWith('*/', i)) {
      if (--depth === 0) return i + 2;
      i++;
    } else if (nested && source.startsWith('/*', i)) {
      depth++;
      i++;
    }
  }
  return source.length;
}

function characterEnd(source, start) {
  let i = start + 1;
  if (source[i] !== '\\') {
    i += source.codePointAt(i) > 0xffff ? 2 : 1;
  } else {
    i++;
    if (source[i] === 'u') {
      if (source[++i] !== '{') return -1;
      while (i < source.length && source[i] !== '}' && source[i] !== '\n') i++;
      i++;
    } else {
      i += source[i] === 'x' ? 3 : 1;
    }
  }
  return source[i] === "'" ? i + 1 : -1;
}

// Use explicit frames for nested Terrane interpolation, so unfinished or deep
// editor input does not throw. An unfinished literal hides the remaining text.
function quotedEnd(source, start, interpolate) {
  const frames = [source[start]];
  let i = start + 1;
  while (i < source.length && frames.length) {
    const frame = frames[frames.length - 1];
    const ch = source[i];
    if (frame === '{') {
      if (ch === '}') {
        frames.pop();
      } else if (ch === '{' || ch === '"' || ch === "'") {
        frames.push(ch);
      } else if (source.startsWith('/*', i)) {
        i = blockEnd(source, i, false);
        continue;
      } else if (ch === '#' || source.startsWith('//', i)) {
        const newline = source.indexOf('\n', i);
        i = newline < 0 ? source.length : newline;
        continue;
      }
    } else if (ch === frame) {
      frames.pop();
    } else if (ch === '\\') {
      i++;
    } else if (interpolate && frame === '"' && ch === '{') {
      if (source[i + 1] === '{') i++;
      else frames.push('{');
    }
    i++;
  }
  return Math.min(i, source.length);
}

function nativeLiteralEnd(source, start) {
  const prefixed = source[start] !== 'r';
  let i = start + Number(prefixed);
  if (source[i] === 'r') {
    const hashes = ++i;
    while (source[i] === '#') i++;
    if (source[i] === '"') {
      const closing = source.indexOf('"' + source.slice(hashes, i), i + 1);
      return closing < 0 ? source.length : closing + i - hashes + 1;
    }
  } else if (prefixed && source[i] === '"') {
    return quotedEnd(source, i, false);
  } else if (prefixed && source[i] === "'") {
    const end = characterEnd(source, i);
    return end < 0 ? start + 1 : end;
  }
  return start + 1;
}

// Only warn about marker-shaped comments, not prose mentioning a marker. Allow
// missing/misplaced comment punctuation, case, separators, and typos in one word.
function oneTypo(word, expected) {
  if (Math.abs(word.length - expected.length) > 1) return false;
  let i = 0;
  while (i < word.length && word[i] === expected[i]) i++;
  if (i === word.length && i === expected.length) return true;
  return word.slice(i + (word.length >= expected.length ? 1 : 0))
      === expected.slice(i + (expected.length >= word.length ? 1 : 0))
    || (word.length === expected.length && word[i] === expected[i + 1]
      && word[i + 1] === expected[i] && word.slice(i + 2) === expected.slice(i + 2));
}

function looksLikeMarker(comment) {
  const match = /^(?:\/{2,}!?|#)\s*(#\s*)?([a-z]+)(?:([\s_-]+)([a-z]+))?(?=\s|[[\]:]|$)(\s*\[)?/i.exec(comment);
  if (!match) return false;
  const [, hash, first, separator, second, bracket] = match;
  let security = first.toUpperCase();
  let action = second?.toUpperCase();
  if (!action && security.startsWith('SECURITY')) {
    action = security.slice('SECURITY'.length);
    security = 'SECURITY';
  }
  if (!action) return false;
  const exact = security === 'SECURITY' && (action === 'BEGIN' || action === 'END');
  const joined = !separator || /[-_]/.test(separator);
  return (hash || bracket || comment.startsWith('#') || (exact && joined)) && (
    exact || ((hash || joined) && (
      (security === 'SECURITY' && (oneTypo(action, 'BEGIN') || oneTypo(action, 'END')))
      || ((action === 'BEGIN' || action === 'END') && oneTypo(security, 'SECURITY')))));
}

function hasMarkerCandidate(lines, terrane) {
  const commentStart = terrane ? /\/\/+!?|#/g : /\/\/+!?/g;
  return lines.some(line => {
    commentStart.lastIndex = 0;
    for (let match; (match = commentStart.exec(line));) {
      if (looksLikeMarker(line.slice(match.index))) return true;
    }
    return false;
  });
}

// Lexical states deliberately follow tools/security-contracts/src/markers.trn,
// not TextMate. Markers are only recognized in comment tokens in source code.
function scanMarkers(lines, languageId) {
  const regions = [];
  const issues = [];
  const terrane = languageId === 'terrane';
  if (!hasMarkerCandidate(lines, terrane)) return { regions, issues };
  const source = lines.join('\n');
  const starts = [];
  const stack = [];
  let line = 0;
  let lineStart = 0;
  let lineEnd = lines[0].length;
  let indent = 0;
  let textIndent = -1;
  let nativeIndent = -1;
  let native = !terrane;
  let expressionStart = true;
  // Empty, return, eligible unsafe, assignment, or other.
  let header = 'empty';
  let nativeTail = false;

  function issue(message, at = line) {
    issues.push({ line: at, message });
  }

  function marker(comment, index) {
    const begin = comment.startsWith(beginPrefix);
    const end = comment.startsWith(endPrefix);
    if (!begin && !end) {
      if (looksLikeMarker(comment)) issue('Malformed security marker; use //# SECURITY-BEGIN [id] or //# SECURITY-END [id]');
      return;
    }
    if (leadingSpace.exec(source.slice(lineStart, index))[0].length !== index - lineStart) {
      issue('Security markers must occupy their own line');
      return;
    }
    const cleaned = comment.replace(trailingSpace, '');
    if (!cleaned.endsWith(']')) {
      issue('Malformed security marker');
      return;
    }
    const id = cleaned.slice((begin ? beginPrefix : endPrefix).length, -1);
    if (!isValidId(id)) {
      issue('Invalid security marker identity');
    } else if (begin) {
      const entry = { id, start: line, end: -1 };
      starts.push(entry);
      stack.push(entry);
    } else if (!stack.length) {
      issue('Security end marker without begin');
    } else if (stack[stack.length - 1].id !== id) {
      issue('Crossing or mismatched security markers');
    } else {
      stack.pop().end = line;
    }
  }

  for (let index = 0; index < source.length;) {
    if (index === lineStart) {
      indent = 0;
      while (source[index + indent] === ' ' || source[index + indent] === '\t') indent++;
      const first = index + indent;
      const blank = first >= lineEnd || source[first] === '\r';
      if (!blank && indent <= textIndent) textIndent = -1;
      if (!blank && indent <= nativeIndent) nativeIndent = -1;
      native = !terrane || nativeIndent >= 0;
      expressionStart = true;
      header = 'empty';
      nativeTail = false;
    }
    const ch = source[index];
    let after = index + 1;
    if (textIndent >= 0 && ch !== '\n') {
      after = lineEnd;
    } else if (source.startsWith('/*', index)) {
      after = blockEnd(source, index, native);
    } else if (source.startsWith('//', index) || (terrane && !native && ch === '#')) {
      after = lineEnd;
      marker(source.slice(index, after), index);
    } else if (native && (ch === 'r' || ch === 'b' || ch === 'c') && !isWord(source, index - 1)) {
      after = nativeLiteralEnd(source, index);
    } else if (ch === '"' || ch === "'") {
      if (native && ch === "'") {
        const end = characterEnd(source, index);
        if (end >= 0) after = end;
      } else {
        after = quotedEnd(source, index, terrane && !native && ch === '"');
      }
      expressionStart = false;
      header = 'other';
      nativeTail = false;
    } else if (terrane && !native && ch === '>' && expressionStart) {
      nativeTail = false;
      after = lineEnd;
      if (source.slice(index, after).replace(trailingSpace, '') === '>>') textIndent = indent;
    } else if (isWord(source, index)) {
      while (isWord(source, after) || (terrane && source[after] === '-')) after++;
      const word = source.slice(index, after);
      nativeTail = terrane && !native && word === 'rust' && header !== 'other';
      if (word === 'unsafe' && (header === 'empty' || header === 'return' || header === 'assign')) header = 'unsafe';
      else if (word === 'return' && header === 'empty') header = 'return';
      else header = 'other';
      expressionStart = word === 'return' || word === 'throw';
    } else if (ch !== ' ' && ch !== '\t' && ch !== '\r' && ch !== '\n') {
      nativeTail = false;
      header = ch === '=' && source[index + 1] !== '=' && !/[=!<>]/.test(source[index - 1] || '') ? 'assign' : 'other';
      expressionStart = '=;,(['.includes(ch);
    }

    while (after > lineEnd && line + 1 < lines.length) {
      if (nativeTail) {
        nativeIndent = indent;
        nativeTail = false;
      }
      lineStart = lineEnd + 1;
      lineEnd = lineStart + lines[++line].length;
    }
    index = after;
  }

  for (const { start } of stack) issue('Security begin marker without end', start);
  for (const { start, end } of starts) {
    if (end >= 0) regions.push({ start, end });
  }
  issues.sort((a, b) => a.line - b.line);
  return { regions, issues };
}

module.exports = { scanMarkers, isValidId };
