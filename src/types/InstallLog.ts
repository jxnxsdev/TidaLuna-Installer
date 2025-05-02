import { Steps } from '../enums/Steps';

/**
 * @description This type is used to log the installation process of the app.
 * @param step The step of the installation process.
 * @param message The log-message of the installation process.
 * @param error The error-message of the installation process.
 * @param isError If the step encountered an error or not.
 */
export type InstallLog = {
    step: Steps;
    message: string;
    error?: string;
    isError: boolean;
}