const { mkdirSync, copyFileSync, readFileSync, writeFileSync } = require("fs");
const { execSync } = require("child_process");
mkdirSync("./build", { recursive: true });
mkdirSync("./compiled", { recursive: true });
const binPath = process.argv[2];
const isLinux = process.argv[3] === "linux";


if (binPath === undefined) throw new Error("No bin path provided");

if (isLinux) {
    const downloadUrl = "https://nodejs.org/dist/v20.15.1/node-v20.15.1-linux-x64.tar.xz";
    const tarFile = "./node.tar.xz";
    execSync(`curl -L ${downloadUrl} -o ${tarFile}`);

    mkdirSync("./node", { recursive: true });

    execSync(`tar -xf ${tarFile} --strip-components=1 -C ./node`);
    execSync(`rm ${tarFile}`);
    copyFileSync("./node/bin/node", binPath);
} else {
    copyFileSync(process.execPath, binPath);
}