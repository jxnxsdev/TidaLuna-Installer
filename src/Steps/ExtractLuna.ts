import { Options } from "../types/Options";
import * as msg from "../utils/MessageHelper";
import { Steps } from "../enums/Steps";
import fs from "fs";
import os from "os";
import path from "path";
import AdmZip from "adm-zip";

export async function execute(options: Options): Promise<boolean> {
    msg.stepLog(Steps.EXTRACTING_LUNA, "Finding Temporary Directory");
    const tempDir = path.join(os.tmpdir(), "TidaLunaInstaller");
    const zipPath = path.join(tempDir, "Luna.zip");
    const extractPath = path.join(tempDir, "LunaExtracted");

    msg.stepLog(Steps.EXTRACTING_LUNA, "Checking if extract path exists");
    if (!fs.existsSync(extractPath)) {
        msg.stepLog(Steps.EXTRACTING_LUNA, "Creating extract path");
        fs.mkdirSync(extractPath, { recursive: true });
    }

    msg.stepLog(Steps.EXTRACTING_LUNA, "Extracting Luna");
    const zip = new AdmZip(zipPath);
    try {
        zip.extractAllTo(extractPath, true);
    } catch (error) {
        msg.stepError(Steps.EXTRACTING_LUNA, "Error extracting Luna", error as Error);
        return false;
    }

    msg.stepLog(Steps.EXTRACTING_LUNA, "Luna extracted successfully");
    msg.stepLog(Steps.EXTRACTING_LUNA, "Cleaning up temporary files");
    try {
        fs.unlinkSync(zipPath);
        fs.rmdirSync(extractPath, { recursive: true });
    } catch (error) {
        msg.stepError(Steps.EXTRACTING_LUNA, "Error cleaning up temporary files", error as Error);
        return false;
    }
    msg.stepLog(Steps.EXTRACTING_LUNA, "Temporary files cleaned up successfully");
    return true;
}