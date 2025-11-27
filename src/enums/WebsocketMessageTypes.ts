export enum WebsocketMessageTypes {
    INSTALL_LOG = 'install_log', // Log message from the installation process
    STEP_LOG = 'step_log', // Log message for a specific step in the installation process
    STEP_UPDATE = 'step_update', // Update message for the current step of the installation process
    INSTALLATION_COMPLETE = 'installation_complete', // Message indicating the installation is complete
    INSTALLATION_START = 'installation_start', // Message indicating the installation has started
    INSTALLATION_ERROR = 'installation_error', // Message indicating an error occurred during installation
    POPUP_MESSAGE = 'popup_message' // Message to show a popup to the user
}