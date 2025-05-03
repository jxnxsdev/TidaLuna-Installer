import { Options } from "../types/Options";
import * as msg from "../utils/MessageHelper";
import { Steps } from "../enums/Steps";
import fs from "fs";
import os from "os";
import path from "path";
import { pipeline } from "stream";
import { promisify } from "util";

const pipelineAsync = promisify(pipeline);

export async function execute(options: Options): Promise<boolean> {
    msg.stepLog(Steps.DOWNLOADING_LUNA, "Finding Temporary Directory");
    const tempDir = path.join(os.tmpdir(), "TidaLunaInstaller");
    const zipAth = path.join(tempDir, "Luna.zip");

    msg.stepLog(Steps.DOWNLOADING_LUNA, "Downloading Luna");
    if (!options.downloadUrl) {
        msg.stepError(Steps.DOWNLOADING_LUNA, "Download URL is not set", new Error("Download URL is not set"));
        return false;
    }
    const downloadUrl = options.downloadUrl;
    const res = await fetch(downloadUrl);
    if (!res.ok || !res.body) {
        msg.stepError(Steps.DOWNLOADING_LUNA, "Error downloading Luna! Please check your network connection!", new Error("Error downloading Luna"));
        return false;
    }

    const filestream = await fs.createWriteStream(zipAth);
    await pipelineAsync(res.body, filestream);

    msg.stepLog(Steps.DOWNLOADING_LUNA, "Luna downloaded successfully");
    return true;
}