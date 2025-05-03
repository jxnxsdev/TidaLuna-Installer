/**
 * @description This type is used to log the installation process of the app globally.
 * @param message The log-message of the installation process.
 * @param error The error-message of the installation process.
 * @param isError If the step encountered an error or not.
 */
export type InstallLog = {
    message: string;
    error?: string;
    isError: boolean;
}