const axios = require('axios');
const semver = require('semver');
const fs = require('fs');

const GITHUB_REPO = 'Inrixia/TidaLuna';

async function getReleases() {
    const res = await axios.get(`https://api.github.com/repos/${GITHUB_REPO}/releases`);
    return res.data;
}

function classifyReleases(releases) {
    const channels = {};
    const semverReleases = [];

    for (const rel of releases) {
        const tag = rel.tag_name;
        const zipUrl = rel.assets.find(a => a.name === 'luna.zip')?.browser_download_url;
        if (!zipUrl) continue;

        if (semver.valid(tag)) {
            semverReleases.push({ version: tag, url: zipUrl, githubUrl: rel.html_url });
            continue;
        }

        const match = tag.match(/^([a-zA-Z]+)-?(\d+\.\d+\.\d+)?$/);
        if (match) {
            const channel = match[1];
            if (!channels[channel]) channels[channel] = [];
            channels[channel].push({ version: tag, url: zipUrl, githubUrl: rel.html_url });
        } else {
            if (!channels[tag]) channels[tag] = [];
            channels[tag].push({ version: tag, url: zipUrl, githubUrl: rel.html_url });
        }
    }

    if (semverReleases.length > 0) {
        const latest = semverReleases.sort((a, b) => semver.rcompare(a.version, b.version))[0];
        channels['semantic'] = [latest];
    }

    return channels;
}

async function main() {
    const releases = await getReleases();
    const channels = classifyReleases(releases);

    const output = [];
    let idCounter = 1;

    for (const [channel, items] of Object.entries(channels)) {
        for (const item of items) {
            output.push({
                name: channel === 'semantic' ? item.version : channel,
                download: item.url,
                githubUrl: item.githubUrl,
                version: item.version,
                id: idCounter++,
            });
        }
    }

    const newJson = JSON.stringify(output, null, 4);
    const oldJson = fs.existsSync('releases.json') ? fs.readFileSync('releases.json', 'utf8') : '';
    if (newJson.trim() !== oldJson.trim()) {
        fs.writeFileSync('releases.json', newJson);
        console.log('RELEASES_CHANGED=true');
        fs.writeFileSync('changed.txt', 'yes');
    } else {
        fs.writeFileSync('changed.txt', '');
    }
}

main().catch(err => {
    console.error(err);
    process.exit(1);
});
