import os from 'os';
import fs from 'fs';
import path from 'path';
import * as msg from '../utils/MessageHelper';
import { Steps } from '../enums/Steps';
import { Options } from '../types/Options';
import { getTidalDirectory } from '../utils/PathHelper';
import * as helpers from '../utils/FileHelper';

export async function execute(options: Options): Promise<boolean> {
    const tidalPath = options.overwritePath || await getTidalDirectory();
    
    if (!tidalPath || !fs.existsSync(tidalPath)) {
        msg.stepError(Steps.UNINSTALLING, 'Tidal is not installed', new Error('Invalid File Path'));
        return false;
    }

    msg.stepLog(Steps.UNINSTALLING, 'Uninstalling TidaLuna / Neptune...');
    const dir = path.join(tidalPath, 'app');

    if (!fs.existsSync(dir)) {
        msg.stepLog(Steps.UNINSTALLING, 'TidaLuna / Neptune is not installed, skipping uninstallation...');
        return true;
    }

    try {
        fs.rmSync(dir, { recursive: true, force: true });
        msg.stepLog(Steps.UNINSTALLING, 'TidaLuna / Neptune uninstalled successfully');
    } catch (error) {
        msg.stepError(Steps.UNINSTALLING, 'Error uninstalling TidaLuna / Neptune', error as Error);
        return false;
    }

    return true;
}