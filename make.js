import { execSync } from "child_process";
import { copyFileSync, mkdirSync, rmSync } from "fs";
import { platform } from "os";

const run = (cmd) => execSync(cmd, { stdio: "inherit" });

const binPath = platform() === "win32" ? ".\\build\\luna-installer.exe" : "./build/luna-installer";

// Clean build directory
rmSync("./build", { recursive: true, force: true });
rmSync("./dist", { recursive: true, force: true });

// Bundle the code
run("npx esbuild ./src/index.ts --bundle --minify --tree-shaking=true --platform=node --outfile=./dist/luna-installer.cjs");

// Copy node binary
mkdirSync("./build", { recursive: true });
copyFileSync(process.execPath, binPath);

// Create the blob
run("node --experimental-sea-config ./sea-config.json");

// Inject the blob
run(
	`npx postject ${binPath} NODE_SEA_BLOB ./dist/luna-installer.blob --sentinel-fuse NODE_SEA_FUSE_fce680ab2cc467b6e072b8b5df1996b2 --macho-segment-name NODE_SEA`
);
