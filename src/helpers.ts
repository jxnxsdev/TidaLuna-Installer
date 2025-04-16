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

export const getTidalDirectory = () => {
    switch (os.platform()) {
        case "win32": {
            const tidalDir = path.join(process.env.LOCALAPPDATA, "TIDAL");
            const appDirs = fs
                .readdirSync(tidalDir)
                .filter((subDir) => subDir.startsWith("app-"));
            const latestAppDir = appDirs.sort().pop();
            if (!latestAppDir) {
                return "";
                console.error("TIDAL app directory not found");
            }
            return path.join(tidalDir, latestAppDir, "resources");
        }
        case "darwin":
            return "/Applications/TIDAL.app/Contents/Resources";
        default:
            console.error("Unsupported platform");
            return "";
    }
};

export const extractAll = async (zipPath: any, extractPath: any) => {
    try {
        if (!fs.existsSync(extractPath)) {
            fs.mkdirSync(extractPath, { recursive: true });
        }

        const zip = new AdmZip(zipPath);
        zip.extractAllTo(extractPath, true);

        console.log(`Extracted files to ${extractPath}`);
    } catch (error) {
        console.error("Failed to extract TidaLuna ZIP file:", error);
        throw error;
    }
};