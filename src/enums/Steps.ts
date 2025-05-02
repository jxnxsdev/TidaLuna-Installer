export enum Steps {
    SETUP = 'SETUP',                                        // App Setup Step, e.g. creating folders, copying files, etc.
    DOWNLOADING_LUNA = 'DOWNLOADING_LUNA',                  // Downloading Luna from github
    EXTRACTING_LUNA = 'EXTRACTING_LUNA',                    // Extracting Luna from the downloaded zip file into the temp folder
    COPYING_ASAR_INSTALL = 'COPYING_ASAR_INSTALL',          // Copying tidal's asar file to original.asar file
    INSERTING_LUNA = 'INSERTING_LUNA',                      // Copy lunas files into the unpacked asar directory
    UNINSTALLING_LUNA = 'UNINSTALLING_LUNA',                // Deletes luna files from the unpacked asar directory
    UNINSTALLING_NEPTUNE = 'UNINSTALLING_NEPTUNE',          // Deletes neptune files from the unpacked asar directory
    COPYING_ASAR_UNINSTALL = 'COPYING_ASAR_UNINSTALL',      // Copying tidal's asar file to original.asar file
    KILLING_TIDAL = 'KILLING_TIDAL',                        // Killing tidals app and player processes
    CHECK_IF_INSTALLED = 'CHECK_IF_INSTALLED'               // Check if the app is already installed
}