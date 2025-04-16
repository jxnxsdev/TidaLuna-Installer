import { Release } from "./types/releases"
import * as helpers from "./helpers"
import { installResponse, uninstallResponse } from "./types/responses"
import path from "path"
import fs from "fs"
import os from "os"
import { pipeline } from "stream";
import { promisify } from "util";

/*
    * Release channel URL
    * This url links to a json file with all release channels. Can be used to allow users
    * the installing of different release streams, like stable, development, beta, etc.
*/
const releaseChannelUrl = 'https://raw.githubusercontent.com/jxnxsdev/TidaLuna-Installer/main/releases.json';

const pipelineAsync = promisify(pipeline);

export class TidaLunaInstaller {
    private releases: Release[] = []

    constructor() {
        this.loadReleases()
    }

    private async loadReleases() {
        try {
            const response = await fetch(releaseChannelUrl)
            if (!response.ok) {
                throw new Error(`Failed to fetch releases: ${response.statusText}`)
            }
            const data = await response.json()
            this.releases = data as Release[]
        } catch (error) {
            console.error('Error loading releases:', error)
        }
    }

    public getReleases() {
        return this.releases
    }

    public async install(id: string): Promise<installResponse> {
        try {
            const release = this.releases.find((r) => r.id === id)
            if (!release) {
                return {
                    success: false,
                    message: "Release channel not found"
                }
            }

            const tempDir = os.tmpdir();
            const zipPath = path.join(tempDir, 'tidaluna.zip')
            const extractPath = path.join(tempDir, "tidaluna-unzipped");

            const res = await fetch(release.download);
            if (!res.ok || !res.body) {
                return {
                    success: false,
                    message: "Failed to download release"
                }
            }

            const fileStream = await fs.createWriteStream(zipPath);
            await pipelineAsync(res.body, fileStream);

            await helpers.extractAll(zipPath, extractPath);
            await helpers.removeFile(zipPath);

            const tidalDir = await helpers.getTidalDirectory();

            const destinationAppPath = path.join(tidalDir, "app");
            await helpers.moveDir(extractPath, destinationAppPath);

            const originalAsarPath = path.join(tidalDir, "original.asar");
            const originalAsarExist = await fs.existsSync(originalAsarPath);
            if (!originalAsarExist) {
                await helpers.moveDir(path.join(tidalDir, "app.asar"), originalAsarPath);
            }

            return {
                success: true,
                message: "TidaLuna installed successfully"
            }
        } catch {
            return {
                success: false,
                message: "Failed to install TidaLuna"
            }
        }
    }

    public async uninstall(): Promise<uninstallResponse> {
        try {
            const tidalDir = await helpers.getTidalDirectory();
            if (!tidalDir) {
                return {
                    success: false,
                    message: "Tidal was not found on your system"
                }
            }

            await helpers.removeDir(path.join(tidalDir, "app"));

            const originalAsar = path.join(tidalDir, "original.asar");
            const originalInPlace = await fs.existsSync(originalAsar)
            
            if (!originalInPlace) {
                return {
                    success: false,
                    message: "Original asar file not found. TidaLuna is (probably) already uninstalled. If it isnt, please reinstall Tidal manually to restore the original asar file."
                }
            }

            await helpers.moveDir(originalAsar, path.join(tidalDir, "app.asar"));

            return {
                success: true,
                message: "TidaLuna uninstalled successfully"
            }
        } catch {
            return {
                success: false,
                message: "Failed to uninstall TidaLuna"
            }
        }
    }
}