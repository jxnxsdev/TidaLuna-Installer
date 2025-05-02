import os from 'os';
import fs from 'fs';
import path from 'path';
import * as msg from '../utils/MessageHelper';
import { Steps } from '../enums/Steps';
import { Options } from '../types/Options';
import { getTidalDirectory } from '../utils/PathHelper';

/**
 * @description This function creates a temporary directory for the installer.
 * @returns {Promise<boolean>} A promise that resolves to true if the step was successful, false otherwise.
 */
export async function execute(options: Options): Promise<boolean> {
    msg.stepLog(Steps.SETUP, 'Getting System Temporary Directory');
    const installerDir = path.join(os.tmpdir(), 'TidaLunaInstaller');

    msg.stepLog(Steps.SETUP, 'Creating Temporary Directory');
    fs.mkdirSync(installerDir, { recursive: true });
    msg.stepLog(Steps.SETUP, 'Temporary Directory Created');

    msg.stepLog(Steps.SETUP, 'Checking if Tidal is installed');
    const tidalPath = options.overwritePath || await getTidalDirectory();

    if (!tidalPath || !fs.existsSync(tidalPath)) {
        msg.stepError(Steps.SETUP, 'Tidal is not installed', new Error('Tidal is not installed'));
        return false;
    }

    const asarFiles = fs.readdirSync(tidalPath).filter(file => file.endsWith('.asar'));
    if (asarFiles.length === 0) {
        msg.stepError(Steps.SETUP, 'Tidal is not installed', new Error('Tidal is not installed'));
        return false;
    }

    msg.stepLog(Steps.SETUP, 'Tidal is installed');
    return true;
}