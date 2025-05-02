// Library imports
import express from 'express';
import ws from 'ws';
import { createServer } from 'http';
import { Buffer } from 'buffer';
import path from 'path';

// Local imports
import { openUrl } from './utils/BrowserHelper';

// Type & Enum imports

// Asset imports
import { publicAssets } from './public-bundle';

const app = express();
const server = createServer(app);
const wss = new ws.Server({ server });

server.listen(3013, () => {
    console.log('TidaLuna Installer is running on port 3013! Open http://localhost:3013 in your browser!');
    openUrl('http://localhost:3013').catch((err) => {
        console.error('Failed to open URL:', err);
    });
});


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