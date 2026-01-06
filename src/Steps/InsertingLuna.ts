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
        msg.stepError(Steps.INSERTING_LUNA, 'Tidal is not installed', new Error('Invalid File Path'));
        return false;
    }

    const tempDir = path.join(os.tmpdir(), 'TidaLunaInstaller');
    const tempDirLuna = path.join(tempDir, 'LunaExtracted');
    const destinationPath = path.join(tidalPath, 'app');

    // copy from the temp directory to the destination path
    if (!fs.existsSync(tempDirLuna)) {
        msg.stepError(Steps.INSERTING_LUNA, 'Temporary directory does not exist', new Error('Invalid File Path'));
        return false;
    }

    await fs.cpSync(tempDirLuna, destinationPath, { recursive: true });
    
    msg.stepLog(Steps.INSERTING_LUNA, 'Luna files copied successfully');
    msg.stepLog(Steps.INSERTING_LUNA, 'Cleaning up temporary files');
    try {
        if (fs.existsSync(tempDir)) {
            fs.rmSync(tempDir, { recursive: true, force: true });
        }

        if (fs.existsSync(tempDirLuna)) {
            fs.rmSync(tempDirLuna, { recursive: true, force: true });
        }
    } catch (error) {
        msg.stepError(Steps.INSERTING_LUNA, 'Error cleaning up temporary files', error as Error);
        return false;
    }
    msg.stepLog(Steps.INSERTING_LUNA, 'Temporary files cleaned up successfully');

    return true;
}