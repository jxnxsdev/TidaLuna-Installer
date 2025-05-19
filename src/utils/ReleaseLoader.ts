import { Release, ReleaseVersion, ReleaseSource } from "../types/Release";
import semver from "semver";
import { v4 as uuidv4 } from "uuid";

let releases: Release[] = [];
let sources: ReleaseSource[] = [];
let releaseSourcesUrl: string = 'https://raw.githubusercontent.com/jxnxsdev/TidaLuna-Installer/main/resources/sources.json';
let releasesLoaded: boolean = false;

/**
 * Loads releases from sources whuch supply a direct download link to a json file
 * @param source - The source to load releases from
 * @returns - An array of releases
 */
async function processDirectReleaseSource(source: ReleaseSource): Promise<Release[]> {
    const response = await fetch(source.url);
    if (!response.ok) {
        console.error(`Failed to fetch release data from ${source.url}: ${response.statusText}`);
    }
    const data = await response.json();
    for (const release of data.releases) {
        release.id = await uuidv4();
    }
    return data.releases;
}


/**
 * Extracts the release channel name from a tag.
 * For example: "1.2.3-alpha", "alpha-1.2.3", "dev" → "alpha", "alpha", "dev"
 */
function extractChannelName(tag: string): string {
    const clean = tag.replace(/^v/, '');

    // Try to parse as semver and get prerelease label
    const parsed = semver.parse(clean);
    if (parsed && parsed.prerelease.length > 0) {
        const first = String(parsed.prerelease[0]);
        return first.split(/[-.]/)[0]; // e.g., "alpha-hotfix" → "alpha"
    }

    // Try matching "alpha-1.2.3" format
    const match = clean.match(/^([a-zA-Z]+)[-_]\d/);
    if (match) return match[1];

    // Valid semver with no prerelease → stable
    if (semver.valid(clean)) return "stable";

    // Fallback: whole tag
    return clean;
}


/**
 * Loads releases from GitHub and groups them into release channels
 * @param source - The source to load releases from
 * @returns - An array of releases
 */
export async function processGithubReleaseSource(source: ReleaseSource): Promise<Release[]> {
    const releaseUrl = `https://api.github.com/repos/${source.url}/releases`;
    const response = await fetch(releaseUrl);
    if (!response.ok) {
        console.error(`Failed to fetch release data from ${releaseUrl}: ${response.statusText}`);
        return [];
    }

    const data = await response.json();

    const grouped: Map<string, Release> = new Map();

    for (const release of data) {
        const tag = release.tag_name;
        const channelName = extractChannelName(tag);

        const version: ReleaseVersion = {
            version: tag,
            download: `https://github.com/${source.url}/releases/download/${tag}/luna.zip`
        };

        if (!grouped.has(channelName)) {
            grouped.set(channelName, {
                name: channelName,
                githubUrl: `https://github.com/${source.url}`,
                id: uuidv4(),
                versions: [version]
            });
        } else {
            grouped.get(channelName)!.versions.push(version);
        }
    }

    return Array.from(grouped.values());
}


/**
 * Loads releases from the sources defined in the sources.json file
 * 
 */
async function loadReleaseSources() {
    const response = await fetch(releaseSourcesUrl);
    if (!response.ok) {
        throw new Error(`Failed to fetch release sources: ${response.statusText}`);
        return;
    }
    sources = await response.json();
}

/**
 * Loads releases from the sources defined in the sources.json file
 * @returns - An array of releases
 */
export async function loadReleases() {
    if (releasesLoaded) return releases;
    await loadReleaseSources();
    for (const source of sources) {
        if (source.type === 'github') {
            const githubReleases = await processGithubReleaseSource(source);
            releases.push(...githubReleases);
        } else if (source.type === 'direct') {
            const directReleases = await processDirectReleaseSource(source);
            releases.push(...directReleases);
        }
    }
    releasesLoaded = true;
    return releases;
}