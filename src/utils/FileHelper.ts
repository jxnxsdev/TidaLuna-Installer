import fs from 'fs';
import { getAppdataPath } from './PathHelper';

export const removeFile = (filePath: any) => {
    fs.unlinkSync(filePath);
};

export const removeDir = (dirPath: any) => {
    fs.rmSync(dirPath, { recursive: true, force: true });
};

export async function createAppDataDir() {
    const appDataPath = await getAppdataPath();
    if (!fs.existsSync(appDataPath)) {
        fs.mkdirSync(appDataPath, { recursive: true });
    }
}