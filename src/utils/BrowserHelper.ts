import { exec } from 'child_process';
import * as os from 'os';

export async function openUrl(url: string): Promise<void> {
    return new Promise((resolve, reject) => {
        let command: string;
        if (os.platform() === 'win32') {
            command = `start ${url}`;
        } else if (os.platform() === 'darwin') {
            command = `open ${url}`;
        } else {
            command = `xdg-open ${url}`;
        }

        exec(command, (error) => {
            if (error) {
                reject(error);
            } else {
                resolve();
            }
        });
    });
}