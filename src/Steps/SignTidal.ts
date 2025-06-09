import { exec } from 'child_process';
import { platform } from 'os';
import { Options } from '../types/Options';
import * as msg from '../utils/MessageHelper';
import { Steps } from '../enums/Steps';

export async function execute(options: Options): Promise<boolean> {
    const os = await platform();

    if (os === 'win32') {
        msg.stepLog(Steps.SIGNING_TIDAL, 'No need to sign tidal on Windows, skipping...');
    } else if (os === 'darwin') {
        exec('codesign --force --deep --sign - /Applications/TIDAL.app', (error, stdout, stderr) => {
            if (error) {
                msg.stepError(Steps.KILLING_TIDAL, 'Error signing Tidal on macOS', error);
                return false;
            }
            msg.stepLog(Steps.KILLING_TIDAL, stdout);
            msg.stepLog(Steps.KILLING_TIDAL, 'Tidal signed successfully on macOS');
            return true;
        });
    } else if (os === 'linux') {
        msg.stepLog(Steps.SIGNING_TIDAL, 'No need to sign tidal on Linux, skipping...');
        return true;
    } else {
        msg.stepError(Steps.SIGNING_TIDAL, 'Unsupported Operating System', new Error('Unsupported OS'));
        return false;
    }

    return true;
}