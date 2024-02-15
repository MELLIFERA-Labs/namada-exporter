const { parseArgs } = require("node:util");
const { spawnSync, execSync } = require("child_process");
const fs = require('fs');
const path = require('path');

const argsOptions = {
  multicore: {
    type: "boolean",
    short: "m",
  },
  release: {
    type: "boolean",
    short: "r",
  },
};
const { multicore, release } = parseArgs({
  args: process.argv.slice(2),
  options: argsOptions,
}).values;

const mode = release ? "release" : "development";
const multicoreLabel = multicore ? "on" : "off";

execSync("rm -rf dist");

console.log(
  `Building \"shared\" in ${mode} mode. Multicore is ${multicoreLabel}.`
);

const features = [];
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
    `nodejs`,
    `--out-dir`,
    outDir,
    `--`,
    features.length > 0 ? ["--features", features.join(",")].flat() : [],
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

/** 
remove String.raw
**/
const filePath = path.join(outDir, 'shared.js')

try {
    // Read the file content synchronously
    const data = fs.readFileSync(filePath, 'utf8');

    // Regular expression to match the require statements using String.raw
    const regex = /require\(String.raw`(.+?)`\)/g;

    // Replace found instances with standard require statements
    const modifiedData = data.replace(regex, "require('$1')");

    // Write the modified content back to the file or a new file synchronously
    fs.writeFileSync(filePath, modifiedData, 'utf8');

    console.log('The file has been saved with String.raw removed!');
} catch (err) {
    console.error(`can't modify file ${filePath}!`);
    console.error(err);
    process.exit(1)
}

// Remove the .gitignore so we can publish generated files
// execSync(`rm -rf ${outDir}.gitignore`);

// Manually copy wasms to dist
execSync(`cp -r ${outDir}/*.wasm ${__dirname}/../dist/shared`);
