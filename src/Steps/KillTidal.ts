import { exec } from 'child_process';
import { platform } from 'os';
import { Options } from '../types/Options';
import * as msg from '../utils/MessageHelper';
import { Steps } from '../enums/Steps';

export async function execute(options: Options): Promise<boolean> {
    const os = await platform();

    if (os === 'win32') {
        exec('taskkill /IM TIDAL.exe /F', (error, stdout, stderr) => {
            if (error) {
                msg.stepError(Steps.KILLING_TIDAL, 'Error killing Tidal process', error);
                return false;
            }
            msg.stepLog(Steps.KILLING_TIDAL, stdout)
            msg.stepLog(Steps.KILLING_TIDAL, 'Tidal process killed successfully');
            return true;
        });
    } else if (os === 'darwin') {
        exec('pkill -f TIDAL', (error, stdout, stderr) => {
            if (error) {
                msg.stepError(Steps.KILLING_TIDAL, 'Error killing Tidal process', error);
                return false;
            }
            msg.stepLog(Steps.KILLING_TIDAL, stdout)
            msg.stepLog(Steps.KILLING_TIDAL, 'Tidal process killed successfully');
            return true;
        });
    } else {
        msg.stepError(Steps.KILLING_TIDAL, 'Unsupported Operating System', new Error('Unsupported OS'));
        return false;
    }
}