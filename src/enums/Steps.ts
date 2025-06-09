export enum Steps {
    SETUP = 'SETUP',                                        // App Setup Step, e.g. creating folders, copying files, etc.
    DOWNLOADING_LUNA = 'DOWNLOADING_LUNA',                  // Downloading Luna from github
    EXTRACTING_LUNA = 'EXTRACTING_LUNA',                    // Extracting Luna from the downloaded zip file into the temp folder
    COPYING_ASAR_INSTALL = 'COPYING_ASAR_INSTALL',          // Copying tidal's asar file to original.asar file
    INSERTING_LUNA = 'INSERTING_LUNA',                      // Copy lunas files into the tidal directory
    UNINSTALLING = 'UNINSTALLING',                          // Deletes luna / neptune files from the tidal directory
    COPYING_ASAR_UNINSTALL = 'COPYING_ASAR_UNINSTALL',      // Copying tidal's asar file to original.asar file
    KILLING_TIDAL = 'KILLING_TIDAL',                        // Killing tidals app and player processes
    SIGNING_TIDAL = 'SIGNING_TIDAL',                        // Signing the tidal app on macOS
}