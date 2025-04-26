const { mkdirSync, copyFileSync, readFileSync, writeFileSync } = require("fs");
mkdirSync("./build", { recursive: true });
mkdirSync("./compiled", { recursive: true });
const binPath = process.argv[2];
if (binPath === undefined) throw new Error("No bin path provided");
copyFileSync(process.execPath, binPath);