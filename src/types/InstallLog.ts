import { Steps } from '../enums/Steps';

export type InstallLog = {
    step: Steps;
    message: string;
    error?: string;
    isError: boolean;
}