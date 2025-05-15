// Library imports
import express from 'express';
const WebSocket = require('ws');
import { createServer } from 'http';
import { Buffer } from 'buffer';
import path from 'path';

// Local imports
import { openUrl } from './utils/BrowserHelper';
import * as manager from './InstallManager';
import { isLunaInstalled } from './utils/PathHelper';

// Type & Enum imports
import { WebsocketMessageTypes } from './enums/WebsocketMessageTypes';
import { WebsocketMessage } from './types/WebsocketMessage';
import { Release } from './types/Release';
import { Options } from './types/Options';

// Asset imports
import { publicAssets } from './public-bundle';

/*
* Release channel URL
* This url links to a json file with all release channels. Can be used to allow users
* the installing of different release streams, like stable, development, beta, etc.
*/
const releaseChannelUrl = 'https://raw.githubusercontent.com/Tr3yWay996/TidaLuna-Installer_reloaded/refs/heads/main/releases.json';
let releases: Release[] = [];

const app = express();
const server = createServer(app);
const wss = new WebSocket.Server({ server });

let options: Options = {
    overwritePath: undefined,
    downloadUrl: undefined,
    action: undefined,
}

server.listen(65530, () => {
    console.log('TidaLuna Installer is running on port 65530! Open http://localhost:65530 in your browser!');
    openUrl('http://localhost:65530').catch((err) => {
        console.error('Failed to open URL:', err);
    });
});

// Express middleware to handle Sending encoded files to the client
// Required because the files need to be packageed into the binary
app.use((req:express.Request, res:express.Response, next:express.NextFunction) => {
    if (req.method !== 'GET') return next(); // This endpoint only needs to handle GET requests
    let cleanPath = req.path.replace(/^\/+/, '');
    if (cleanPath === '') cleanPath = 'index.html'; // Default to index.html if no path is provided
    const b64 = publicAssets[cleanPath];
    if (!b64) return next();
    const data = Buffer.from(b64, 'base64');
    const ext = path.extname(cleanPath).toLowerCase();
    const mime = ext.startsWith('.') ? ext.slice(1) : ext;
    res.type(mime).send(data);
})

app.get('/state', async (req:express.Request, res:express.Response) => {
    let isRunning = await manager.getIsRunning();
    let options = await manager.getOptions();
    let currentStep = await manager.getCurrentStep();
    let currentStepIndex = await manager.getCurrentStepIndex();
    let steps = await manager.getSteps();

    res.json({
        isRunning: isRunning,
        options: options ? options : {},
        currentStep: currentStep ? currentStep : 'none',
        currentStepIndex: currentStepIndex,
        steps: steps,
    });
});

app.get('/releases', async (req:express.Request, res:express.Response) => {
    await fetchReleases();
    res.json(releases);
});

app.get('/start', async (req:express.Request, res:express.Response) => {
    res.status(200).send('Installation started!');
    await manager.generateInstallSteps();
    await manager.start();
});

app.get('/setOptions', async (req:express.Request, res:express.Response) => {
    if (!req.query || !req.query.action) {
        res.status(400).send('No options provided!');
        return;
    }

    const action = req.query.action;
    const overwritePath = req.query.overwritePath || undefined;
    const downloadUrl = action === 'install' ? req.query.downloadUrl : undefined;
    if (action === 'install' && !downloadUrl) {
        res.status(400).send('No download URL provided!');
        return;
    }
    if (action !== 'install' && action !== 'uninstall') {
        res.status(400).send('Invalid action provided!');
        return;
    }
    options = {
        action: action,
        // @ts-expect-error
        overwritePath: overwritePath,
        // @ts-expect-error
        downloadUrl: downloadUrl,
    };
    await manager.setOptions(options);
    res.status(200).json(options);
});

app.get('/isInstalled', async (req:express.Request, res:express.Response) => {
    const isInstalled = await isLunaInstalled();
    res.json({ isInstalled: isInstalled });
});

// Websocket setup
wss.on('connection', (ws: any) => {
    console.log('Frontend has connected to the websocket!');
    ws.on('close', () => {
        console.log('Frontend has disconnected from the websocket!');
    });
});

/*
* @description Sends a message to the frontend via the websocket connection.
* @param message The message to send to the frontend.
* @returns {Promise<void>} A promise that resolves when the message is sent.
*/
export async function sendMessageToFrontend(message: WebsocketMessage): Promise<void> {
    if (wss.clients.size === 0) return; // No clients connected, so we don't need to send a message
    const messageStr = JSON.stringify(message);
    wss.clients.forEach((client: WebSocket) => {
        client.send(messageStr);
    });
}

async function fetchReleases() {
    try {
        const response = await fetch(releaseChannelUrl);
        if (!response.ok) {
            throw new Error(`HTTP error! status: ${response.status}`);
        }
        releases = await response.json();
    } catch (error) {
        console.error('Error fetching releases:', error);
    }
}