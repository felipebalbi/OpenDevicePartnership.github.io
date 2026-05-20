#!/usr/bin/env node
/**
 * build-assets.js
 *
 * Assembles the static `target/site/` deploy tree without
 * cargo-leptos. Runs AFTER `cargo build --release --bin odp` but
 * BEFORE `./target/release/odp --prerender`, so the prerender step
 * writes its HTML next to the assets they reference.
 *
 * Steps:
 *   1. Clean `target/site/`.
 *   2. Copy `public/**` into `target/site/` verbatim (favicon,
 *      images, fonts, _headers, _redirects, repo_graph.{js,css},
 *      interactive.js, ...).
 *   3. Minify `target/site/odp.css` from `style/main.css` via
 *      lightningcss.
 *   4. Minify the two hand-written JS files
 *      (`target/site/interactive.js` and `target/site/repo_graph.js`)
 *      with terser.
 */

const fs = require('node:fs');
const fsp = require('node:fs/promises');
const path = require('node:path');
const lightningcss = require('lightningcss');
const { minify } = require('terser');

const root = path.resolve(__dirname, '..');
const siteDir = path.join(root, 'target', 'site');
const publicDir = path.join(root, 'public');
const cssIn = path.join(root, 'style', 'main.css');
const cssOut = path.join(siteDir, 'odp.css');

async function cleanContents(p) {
    // Remove entries *inside* `p` but leave `p` itself in place.
    // Windows holds a directory lock when a process has it as cwd
    // (e.g. `python -m http.server` from `target/site/` during a
    // local preview), which makes `rm -rf target/site` fail with
    // EBUSY halfway through and leaves a half-cleaned tree behind.
    // Cleaning contents only is enough — copy + minify steps below
    // overwrite anything we still care about.
    if (!fs.existsSync(p)) {
        await fsp.mkdir(p, { recursive: true });
        return;
    }
    for (const entry of await fsp.readdir(p)) {
        await fsp.rm(path.join(p, entry), { recursive: true, force: true });
    }
}

async function copyDir(src, dst) {
    await fsp.mkdir(dst, { recursive: true });
    for (const entry of await fsp.readdir(src, { withFileTypes: true })) {
        const s = path.join(src, entry.name);
        const d = path.join(dst, entry.name);
        if (entry.isDirectory()) {
            await copyDir(s, d);
        } else if (entry.isFile()) {
            await fsp.copyFile(s, d);
        }
    }
}

async function minifyCss() {
    if (!fs.existsSync(cssIn)) {
        throw new Error(`missing ${cssIn} — run \`npm run build:css\` first`);
    }
    const code = fs.readFileSync(cssIn);
    const out = lightningcss.transform({
        filename: 'odp.css',
        code,
        minify: true,
        sourceMap: false,
    });
    fs.writeFileSync(cssOut, out.code);
    console.log(
        `build-assets: minified CSS  ${code.length} -> ${out.code.length} bytes (${cssOut})`
    );
}

async function minifyJsInPlace(file) {
    if (!fs.existsSync(file)) {
        console.warn(`build-assets: skipping ${file} (not present)`);
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
    console.log(`build-assets: minified JS  ${before} -> ${after} bytes (${path.basename(file)})`);
}

(async () => {
    await cleanContents(siteDir);
    await copyDir(publicDir, siteDir);
    await minifyCss();
    await minifyJsInPlace(path.join(siteDir, 'interactive.js'));
    await minifyJsInPlace(path.join(siteDir, 'repo_graph.js'));
})().catch((err) => {
    console.error('build-assets failed:', err);
    process.exit(1);
});
