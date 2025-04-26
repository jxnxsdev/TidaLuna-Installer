import express from 'express';
import { exec } from 'child_process';
import os from 'os';
import path from 'path';
import { TidaLunaInstaller } from './installer';
import { uninstallResponse, installResponse } from './types/responses';
import { publicAssets } from './public-bundle';
import { Buffer } from 'buffer';

const app = express();
let installer: TidaLunaInstaller | null = null;

app.use(express.json());
app.use(express.urlencoded({ extended: true }));


app.use((req, res, next) => {
  if (req.method !== 'GET') return next();

  let cleanPath = req.path.replace(/^\/+/, '');

  if (cleanPath === '') cleanPath = 'index.html';

  const b64 = publicAssets[cleanPath];
  if (!b64) return next();

  const data = Buffer.from(b64, 'base64');
  const ext  = path.extname(cleanPath).toLowerCase();
  const mime = ext.startsWith('.') ? ext.slice(1) : ext;

  res.type(mime).send(data);
});


app.get('/uninstall', async (req, res) => {
    const response: uninstallResponse = await installer?.uninstall();
    if (!response) {
        res.status(500).send('Failed to uninstall TidaLuna');
        return;
    }

    res.status(response.success ? 200 : 500).send(response.message);
    return;
});

app.get('/install', async (req, res) => {
    if (!req.query || !req.query.release) {
        res.status(400).send('Release channel not specified');
        return;
    }

    const releaseId = req.query.release as string;

    const releases = await installer?.getReleases();
    if (!releases) {
        res.status(500).send('Failed to load releases');
        return;
    }

    const release = await releases.find((r) => r.id === releaseId);
    if (!release) {
        res.status(404).send('Release channel not found');
        return;
    }

    const response: installResponse = await installer?.install(releaseId);

    if (!response) {
        res.status(500).send('Failed to install TidaLuna');
        return;
    }

    res.status(response.success ? 200 : 500).send(response.message);
    return;
});

app.get('/releases', async (req, res) => {
    const releases = await installer?.getReleases();
    if (!releases) {
        res.status(500).send('Failed to load releases');
        return;
    }

    res.status(200).json(releases);
    return;
});

app.listen(3000, async () => {
    console.log('Server is running on port 3000');

    installer = await new TidaLunaInstaller();

    // Open the installer inside the browser
    const url = 'http://localhost:3000';
    switch (os.platform()) {
        case 'win32':
            exec(`start ${url}`);
            break;
        case 'darwin':
            exec(`open ${url}`);
            break;
        case 'linux':
            exec(`xdg-open ${url}`);
            break;
        default:
            console.log(`Please open the URL manually: ${url}`);
    }
});