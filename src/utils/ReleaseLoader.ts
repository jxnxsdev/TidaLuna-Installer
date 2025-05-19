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
 * Extracts a release channel name from a version tag.
 * - "1.0.0" => "1.0.0"
 * - "1.0.0-alpha" or "alpha-1.0.0" => "alpha-1.0.0"
 * - "alpha" => "alpha"
 */
function extractReleaseChannel(tag: string): string {
    const cleanTag = tag.replace(/^v/, ''); // Remove leading 'v'

    // Exact semver: "1.0.0"
    if (semver.valid(cleanTag)) return cleanTag;

    // Semver with pre-release: "1.0.0-alpha"
    if (semver.valid(cleanTag.split(/[-]/).pop() || '')) return cleanTag;

    // Possibly "alpha-1.0.0" -> capture as-is
    if (cleanTag.includes('-') && semver.valid(cleanTag.split('-').pop() || '')) return cleanTag;

    // Fallback to full tag (e.g. "alpha")
    return cleanTag;
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

    const githubReleases = await response.json();

    const grouped: Map<string, ReleaseVersion[]> = new Map();

    for (const ghRelease of githubReleases) {
        const tagName: string = ghRelease.tag_name;
        const downloadUrl: string | undefined = ghRelease.assets?.[0]?.browser_download_url;

        if (!downloadUrl) continue;

        const channel = extractReleaseChannel(tagName);

        if (!grouped.has(channel)) {
            grouped.set(channel, []);
        }

        grouped.get(channel)!.push({
            version: tagName,
            download: downloadUrl
        });
    }

    const releases: Release[] = [];
    for (const [channel, versions] of grouped) {
        releases.push({
            name: channel,
            githubUrl: `https://github.com/${source.url}`,
            id: uuidv4(),
            versions
        });
    }

    return releases;
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