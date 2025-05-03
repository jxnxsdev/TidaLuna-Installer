import fs from 'fs';

export const removeFile = (filePath: any) => {
    fs.unlinkSync(filePath);
};

export const removeDir = (dirPath: any) => {
    fs.rmSync(dirPath, { recursive: true, force: true });
};