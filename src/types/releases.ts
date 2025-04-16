export type Release = {
    name: string;       // Name of the release channel, for example "stable", "beta", "dev"
    download: string;   // Direct download URL to the zip file containing the injected code
    githubUrl: string;  // Url to the github repo with the source of the injexted code
    version: string;    // Version of the release (should be self explanatory)
    id: string;         // Unique ID, used inside the code for version selection. Must be unique for each release channel, otherwise app go boom
}