export enum WebsocketMessageTypes {
    INSTALL_LOG = 'install_log', // Log message from the installation process
    STEP_LOG = 'step_log', // Log message for a specific step in the installation process
    STEP_UPDATE = 'step_update', // Update message for the current step of the installation process
    INSTALLATION_COMPLETE = 'installation_complete', // Message indicating the installation is complete
    INSTALLATION_SRART = 'installation_start', // Message indicating the installation has started
}