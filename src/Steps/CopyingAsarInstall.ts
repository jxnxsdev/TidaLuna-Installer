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
        msg.stepError(Steps.COPYING_ASAR_INSTALL, 'Tidal is not installed', new Error('Invalid File Path'));
        return false;
    }
    
    const asarFiles = fs.readdirSync(tidalPath).filter(file => file.endsWith('.asar'));
    if (asarFiles.length === 0) {
        msg.stepError(Steps.COPYING_ASAR_INSTALL, 'Tidal is not installed or your installation is corrupt!', new Error('Asar file missing'));
        return false;
    }

    msg.stepLog(Steps.COPYING_ASAR_INSTALL, 'Copying app.asar to original.asar');
    let originalAsarPath = path.join(tidalPath, 'original.asar');
    const originalAsarExists = fs.existsSync(originalAsarPath);
    if (!originalAsarExists) {
        msg.stepLog(Steps.COPYING_ASAR_INSTALL, 'Creating original.asar backup');
        await fs.copyFileSync(originalAsarPath, path.join(tidalPath, 'app.asar'));
    }

    const appAsarPath = path.join(tidalPath, 'app.asar');
    if (fs.existsSync(appAsarPath)) {
        fs.unlinkSync(appAsarPath);
    }

    msg.stepLog(Steps.COPYING_ASAR_INSTALL, 'Copying app.asar to original.asar completed successfully');
    return true;
}