// Library imports
import express from 'express';
import ws from 'ws';
import { createServer } from 'http';
import { Buffer } from 'buffer';
import path from 'path';

// Local imports
import { openUrl } from './utils/BrowserHelper';

// Type & Enum imports
import { WebsocketMessageTypes } from './enums/WebsocketMessageTypes';

// Asset imports
import { publicAssets } from './public-bundle';
import { WebsocketMessage } from './types/WebsocketMessage';

const app = express();
const server = createServer(app);
const wss = new ws.Server({ server });

server.listen(3013, () => {
    console.log('TidaLuna Installer is running on port 3013! Open http://localhost:3013 in your browser!');
    openUrl('http://localhost:3013').catch((err) => {
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

// Websocket setup
wss.on('connection', (ws) => {
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
    wss.clients.forEach((client) => {
        if (client.readyState === ws.OPEN) {
            client.send(messageStr);
        }
    });
}