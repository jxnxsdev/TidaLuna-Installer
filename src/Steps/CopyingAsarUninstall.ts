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
    
    if (!tidalPath || !await fs.existsSync(tidalPath)) {
        msg.stepError(Steps.COPYING_ASAR_UNINSTALL, 'Tidal is not installed', new Error('Invalid File Path'));
        return false;
    }
    
    const asarFiles = await fs.readdirSync(tidalPath).filter(file => file.endsWith('.asar'));
    if (asarFiles.length === 0) {
        msg.stepError(Steps.COPYING_ASAR_UNINSTALL, 'Tidal is not installed or your installation is corrupt!', new Error('Asar file missing'));
        return false;
    }

    msg.stepLog(Steps.COPYING_ASAR_UNINSTALL, 'Copying original.asar to app.asar');
    let originalAsarPath = path.join(tidalPath, 'original.asar');
    let appAsarPath = path.join(tidalPath, 'app.asar');
    const originalAsarExists = await fs.existsSync(originalAsarPath);
    if (!originalAsarExists) {
        msg.stepError(Steps.COPYING_ASAR_UNINSTALL, 'original.asar not found. Your installation is corrupt! Please reinstall tidal!', new Error('original.asar not found'));
        return false;
    }
    if (await fs.existsSync(appAsarPath)) {
        await fs.unlinkSync(appAsarPath);
    }

    await fs.copyFileSync(originalAsarPath, appAsarPath);
    msg.stepLog(Steps.COPYING_ASAR_UNINSTALL, 'Copying original.asar to app.asar completed successfully');
    return true;
}