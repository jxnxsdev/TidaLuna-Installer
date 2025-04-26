// bundle-public.js
const fs = require('fs');
const path = require('path');

const publicDir = path.join(__dirname, 'public');
const outputFile = path.join(__dirname, 'src', 'public-bundle.ts');

function walkDir(dir, fileCallback) {
  fs.readdirSync(dir).forEach(file => {
    const fullPath = path.join(dir, file);
    if (fs.statSync(fullPath).isDirectory()) {
      walkDir(fullPath, fileCallback);
    } else {
      fileCallback(fullPath);
    }
  });
}

const assets = {};

walkDir(publicDir, (filePath) => {
  const relPath = path.relative(publicDir, filePath).replace(/\\/g, '/');
  const content = fs.readFileSync(filePath);
  assets[relPath] = content.toString('base64'); // base64 encode
});

const tsOutput = `
// This file is auto-generated
export const publicAssets: Record<string, string> = ${JSON.stringify(assets, null, 2)};
`;

fs.writeFileSync(outputFile, tsOutput);

console.log(`✅ public-bundle.ts generated with ${Object.keys(assets).length} assets.`);
