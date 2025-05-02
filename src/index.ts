import express from 'express';
import ws from 'ws';
import { createServer } from 'http';

import { openUrl } from './utils/BrowserHelper';

const app = express();
const server = createServer(app);
const wss = new ws.Server({ server });

server.listen(3013, () => {
    console.log('TidaLuna Installer is running on port 3013! Open http://localhost:3013 in your browser!');
    openUrl('http://localhost:3013').catch((err) => {
        console.error('Failed to open URL:', err);
    });
});