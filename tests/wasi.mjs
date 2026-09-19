import assert from 'node:assert/strict';
import { mkdirSync, readFileSync, writeFileSync } from 'node:fs';
import { resolve } from 'node:path';
import { spawnSync } from 'node:child_process';

const artifact = resolve(process.argv[2] ?? 'target/wasm32-wasip1/release/adamantium-json.wasm');
const bytes = readFileSync(artifact);
assert.deepEqual([...bytes.subarray(0, 8)], [0, 97, 115, 109, 1, 0, 0, 0]);
assert.ok(WebAssembly.validate(bytes));
const module = await WebAssembly.compile(bytes);
assert.ok(WebAssembly.Module.exports(module).some(value => value.name === '_start'));
mkdirSync('target/wasi-json-test', { recursive: true });

function run(args, code = 0) {
  const result = spawnSync(process.execPath, [
    'scripts/run-wasi.mjs', artifact, resolve('target/wasi-json-test'), ...args,
  ], { encoding: 'utf8' });
  assert.ifError(result.error);
  assert.equal(result.status, code, `${args}: ${result.stderr}`);
  return result.stdout;
}

assert.equal(run(['compact', '{"value": 1}']).trim(), '{"value":1}');
assert.equal(run(['type-of', '[1,2]']).trim(), 'array');
assert.equal(run(['is-valid', '{bad}']).trim(), 'false');
assert.match(run(['parse', '{bad}'], 1), /^$/);
writeFileSync('target/wasi-json-test/input.json', '{"ok":true}');
assert.equal(run(['read', '/workspace/input.json']), `{
  "ok": true
}`);
run(['write', '/workspace/output.json', '[1, 2]']);
assert.equal(readFileSync('target/wasi-json-test/output.json', 'utf8'), `[
  1,
  2
]\n`);
console.log('WASM validation and WASI JSON tests passed.');
