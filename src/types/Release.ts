export type Release = {
    name: string;       // Name of the release channel, for example "stable", "beta", "dev"
    githubUrl: string;  // Url to the github repo with the source of the injexted code
    id: string;         // Unique ID, used inside the code for version selection. Must be unique for each release channel, otherwise app go boom
    versions: ReleaseVersion[]; // Array of versions for this release channel
}

export type ReleaseVersion = {
    version: string;    // Version of the release (should be self explanatory)
    download: string;   // Direct download URL to the zip file containing the injected code
}

export type ReleaseSource = {
    url: string; // URL to the source
    type: 'github' | 'direct'; // Type of the source, either github (load releases) or direct (returns a json file)
}