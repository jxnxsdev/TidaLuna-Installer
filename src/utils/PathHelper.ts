import fs from 'fs';
import os from 'os';
import path from 'path';
import * as msg from '../utils/MessageHelper';


export async function getTidalDirectory(): Promise<string> {
    switch (os.platform()) {
        case "win32": {
            const tidalDir = path.join(process.env.LOCALAPPDATA, "TIDAL");
            const appDirs = fs
                .readdirSync(tidalDir)
                .filter((subDir) => subDir.startsWith("app-"));
            const latestAppDir = appDirs.sort().pop();
            if (!latestAppDir) {
                return "";
            }
            return path.join(tidalDir, latestAppDir, "resources");
        }
        case "darwin":
            return "/Applications/TIDAL.app/Contents/Resources";
        case "linux":
            if(await fs.existsSync("/var/lib/flatpak/app/com.mastermindzh.tidal-hifi/current/active/files/lib/tidal-hifi/resources/")) {
                return "/var/lib/flatpak/app/com.mastermindzh.tidal-hifi/current/active/files/lib/tidal-hifi/resources/";
            } else if(await fs.existsSync("/opt/tidal-hifi/resources/")) {
                return "/opt/tidal-hifi/resources/"
            } else return "";
        default:
            return "";
    }
}

export async function isLunaInstalled(): Promise<boolean> {
    const tidalPath = await getTidalDirectory();
    if (!tidalPath) {
        return false;
    }
    const appDir = path.join(tidalPath, "app");
    return await fs.existsSync(appDir);
}

/*
* Get the application data path for TidaLunaInstaller based on the OS
* @returns {Promise<string>} The application data path
*/
export async function getAppdataPath(): Promise<string> {
    switch (os.platform()) {
        case "win32":
            return path.join(process.env.APPDATA, "TidaLunaInstaller");
        case "darwin":
            return path.join(os.homedir(), "Library", "Application Support", "TidaLunaInstaller");
        case "linux":
            return path.join(os.homedir(), ".config", "TidaLunaInstaller");
        default:
            return "";
    }
}