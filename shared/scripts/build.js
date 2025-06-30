const { parseArgs } = require("node:util");
const { spawnSync, execSync } = require("child_process");
const path = require("path");
const fs = require("fs");

const targets = ["web", "nodejs"];

const argsOptions = {
  target: {
    type: "string",
    short: "t",
  },
  multicore: {
    type: "boolean",
    short: "m",
  },
  release: {
    type: "boolean",
    short: "r",
  },
};
const {
  multicore,
  release,
  target: maybeTarget,
} = parseArgs({
  args: process.argv.slice(2),
  options: argsOptions,
}).values;

const mode = release ? "release" : "development";
const multicoreLabel = multicore ? "on" : "off";
const target = targets.includes(maybeTarget) ? maybeTarget : "web";

execSync("rm -rf dist");
execSync("rm -rf src/shared");

console.log(
  `Building \"shared\" in ${mode} mode for ${target} target. Multicore is ${multicoreLabel}.`
);

const features = [target];
let profile = "--release";

if (multicore) {
  features.push("multicore");
}
if (!release) {
  features.push("dev");
  profile = "--dev";
}

const outDir = `${__dirname}/../src/shared`;

execSync(`rm -rf ${outDir}}`);
const { status } = spawnSync(
  "wasm-pack",
  [
    "build",
    `${__dirname}/../lib`,
    profile,
    `--target`,
    target,
    `--out-dir`,
    outDir,
    `--`,
    ["--features", features.join(",")].flat(),
    multicore ? [`-Z`, `build-std=panic_abort,std`] : [],
  ].flat(),
  {
    stdio: "inherit",
    ...(multicore && {
      env: {
        ...process.env,
        RUSTFLAGS: "-C target-feature=+atomics,+bulk-memory,+mutable-globals",
      },
    }),
  }
);

if (status !== 0) {
  process.exit(status);
}

execSync("rm -rf dist && mkdir dist && mkdir dist/shared");

// Remove the .gitignore so we can publish generated files
execSync(`rm -rf ${outDir}.gitignore`);

/** 
remove String.raw
**/
const filePath = path.join(outDir, 'shared.js')

try {
    // Read the file content synchronously
    const data = fs.readFileSync(filePath, 'utf8');

    // Regular expression to match the require statements using String.raw
    const regex = /require\(String.raw`(.+?)`\)/g;
    const wasmBuffer = fs.readFileSync(path.join(outDir, 'shared_bg.wasm'));
   
    const base64String = wasmBuffer.toString('base64');

    // Replace found instances with standard require statements
    const modifiedData = data.replace(regex, "require('$1')");
    const modifiedData2 = modifiedData.replace(/const bytes = require\('fs'\)\.readFileSync\(path\);/g,`const bytes = Buffer.from("${base64String}", 'base64');`);


    // Write the modified content back to the file or a new file synchronously
    fs.writeFileSync(filePath, modifiedData2, 'utf8');

    console.log('The file has been saved with String.raw removed!');
} catch (err) {
    console.error(`can't modify file ${filePath}!`);
    console.error(err);
    process.exit(1)
}

// Manually copy wasms to dist
execSync(`cp -r ${outDir}/*.wasm ${__dirname}/../dist/shared`);