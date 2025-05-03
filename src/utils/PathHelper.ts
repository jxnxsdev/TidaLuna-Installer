import fs from 'fs';
import os from 'os';
import path from 'path';


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