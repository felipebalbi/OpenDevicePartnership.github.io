#!/usr/bin/env node
/**
 * postbuild.js
 *
 * Runs after `cargo leptos build --release` to slim down the
 * deployed asset tree under `target/site/`:
 *
 *   1. Minify our lazy-loaded D3 wrapper (`repo_graph.js`) with
 *      terser. cargo-leptos minifies the wasm-bindgen shim under
 *      /pkg/ via swc (js-minify=true) but does NOT touch arbitrary
 *      JS copied verbatim from the `assets-dir` (public/), so we
 *      handle that here.
 *
 *   2. Delete the wasm-bindgen TypeScript declarations
 *      (`*.d.ts`) shipped under /pkg/. They are useful at compile
 *      time, not at runtime; serving them is pure dead weight.
 *
 * Must run BEFORE `./target/release/odp --prerender` so the
 * prerendered HTML doesn't reference the now-deleted .d.ts blobs
 * (it doesn't today, but keeps the ordering honest).
 */
const fs = require('node:fs');
const path = require('node:path');
const { minify } = require('terser');

const root = path.resolve(__dirname, '..');
const siteDir = path.join(root, 'target', 'site');
const pkgDir = path.join(siteDir, 'pkg');

if (!fs.existsSync(siteDir)) {
    console.error(`postbuild: missing ${siteDir} -- run \`cargo leptos build --release\` first`);
    process.exit(1);
}

async function minifyRepoGraph() {
    const file = path.join(siteDir, 'repo_graph.js');
    if (!fs.existsSync(file)) {
        console.warn(`postbuild: skipping ${file} (not present)`);
        return;
    }
    const before = fs.statSync(file).size;
    const src = fs.readFileSync(file, 'utf8');
    const out = await minify(src, {
        ecma: 2020,
        compress: { passes: 2, drop_console: false },
        mangle: true,
        format: { comments: false },
    });
    if (out.error) throw out.error;
    fs.writeFileSync(file, out.code);
    const after = Buffer.byteLength(out.code, 'utf8');
    console.log(`postbuild: minified repo_graph.js ${before} -> ${after} bytes`);
}

function removeDeclarations() {
    if (!fs.existsSync(pkgDir)) return;
    let removed = 0;
    let bytes = 0;
    for (const entry of fs.readdirSync(pkgDir)) {
        if (!entry.endsWith('.ts')) continue;
        const f = path.join(pkgDir, entry);
        bytes += fs.statSync(f).size;
        fs.unlinkSync(f);
        removed += 1;
    }
    console.log(`postbuild: removed ${removed} TypeScript declaration file(s) (${bytes} bytes)`);
}

(async () => {
    await minifyRepoGraph();
    removeDeclarations();
})().catch((err) => {
    console.error('postbuild failed:', err);
    process.exit(1);
});
