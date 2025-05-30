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