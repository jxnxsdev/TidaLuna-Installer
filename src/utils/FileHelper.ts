import fs from 'fs';
import os from 'os';
import path from 'path';
import AdmZip from "adm-zip";

export const moveDir = (source: any, destination: any) => {
    fs.renameSync(source, destination);
};

export const removeFile = (filePath: any) => {
    fs.unlinkSync(filePath);
};

export const removeDir = (dirPath: any) => {
    fs.rmSync(dirPath, { recursive: true, force: true });
};