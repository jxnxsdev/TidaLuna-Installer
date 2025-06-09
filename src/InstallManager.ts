import { Steps } from './enums/Steps';
import { WebsocketMessageTypes } from './enums/WebsocketMessageTypes';
import { Options } from './types/Options';
import { sendMessageToFrontend } from '.';
import * as msg from './utils/MessageHelper';

import * as SetupStep from './Steps/Setup';
import * as KillTidalStep from './Steps/KillTidal';
import * as DownloadLunaStep from './Steps/DownloadLuna';
import * as ExtractLunaStep from './Steps/ExtractLuna';
import * as CopyAsarInstallStep from './Steps/CopyingAsarInstall';
import * as InsertLunaStep from './Steps/InsertingLuna';
import * as UninstallStep from './Steps/Uninstalling';
import * as CopyAsarUninstallStep from './Steps/CopyingAsarUninstall';
import * as SignTidalStep from './Steps/SignTidal';

let currentStep: Steps | undefined;
let steps: Steps[] = [];
let currentStepIndex = 0;
let options: Options | undefined;
let isRunning = false;

/**
 * Set the options for the (un)installation process.
 */
export async function setOptions(newOptions: Options): Promise<void> {
    if (isRunning) {
        msg.globalError('Installation process is already running.', new Error('InstallManager::setOptions'));
        return;
    }
    options = newOptions;
    msg.globalLog('Options set successfully.');
}

/**
 * Generate the sequence of install/uninstall steps.
 */
export async function generateInstallSteps(): Promise<void> {
    if (isRunning) {
        msg.globalError('Installation process is already running.', new Error('InstallManager::generateInstallSteps'));
        return;
    }

    if (!options) {
        msg.globalError('Options are not set.', new Error('InstallManager::generateInstallSteps'));
        return;
    }

    switch (options.action) {
        case 'install':
            steps = [
                Steps.SETUP,
                Steps.KILLING_TIDAL,
                Steps.UNINSTALLING,
                Steps.DOWNLOADING_LUNA,
                Steps.EXTRACTING_LUNA,
                Steps.COPYING_ASAR_INSTALL,
                Steps.INSERTING_LUNA,
                Steps.SIGNING_TIDAL
            ];
            break;
        case 'uninstall':
            steps = [
                Steps.KILLING_TIDAL,
                Steps.UNINSTALLING,
                Steps.COPYING_ASAR_UNINSTALL
            ];
            break;
        default:
            msg.globalError('Invalid action.', new Error('InstallManager::generateInstallSteps:: Invalid action.'));
            return;
    }

    currentStepIndex = 0;
    currentStep = steps[currentStepIndex];
    msg.globalLog('Install steps generated successfully.');
}

/**
 * Start the (un)installation process.
 */
export async function start(): Promise<void> {
    if (isRunning) {
        msg.globalError('Installation process is already running.', new Error('InstallManager::start'));
        return;
    }

    if (!steps.length || !options) {
        msg.globalError('Install steps are not generated or options missing.', new Error('InstallManager::start'));
        return;
    }

    isRunning = true;
    msg.globalLog('Installation process started.');

    await sendMessageToFrontend({
        type: WebsocketMessageTypes.INSTALLATION_START,
        data: {
            steps,
            currentStep,
            currentStepIndex,
            action: options.action
        },
    });

    await executeCurrentStep();
}

/**
 * Execute the current step and continue the chain if successful.
 */
async function executeCurrentStep(): Promise<void> {
    const step = steps[currentStepIndex];
    if (!step || !options) {
        msg.installError('Invalid step or missing options.');
        isRunning = false;
        return;
    }

    msg.nextStep(step);

    const stepMap: Record<Steps, { execute: (opts: Options) => Promise<boolean> }> = {
        [Steps.SETUP]: SetupStep,
        [Steps.KILLING_TIDAL]: KillTidalStep,
        [Steps.DOWNLOADING_LUNA]: DownloadLunaStep,
        [Steps.EXTRACTING_LUNA]: ExtractLunaStep,
        [Steps.COPYING_ASAR_INSTALL]: CopyAsarInstallStep,
        [Steps.INSERTING_LUNA]: InsertLunaStep,
        [Steps.UNINSTALLING]: UninstallStep,
        [Steps.COPYING_ASAR_UNINSTALL]: CopyAsarUninstallStep,
        [Steps.SIGNING_TIDAL]: SignTidalStep
    };

    try {
        const result = await stepMap[step]?.execute(options);
        
        if (!result) {
            msg.installError(`${step} step failed.`);
            isRunning = false;
            return;
        }

        currentStepIndex++;
        if (currentStepIndex >= steps.length) {
            msg.globalLog('Installation process completed successfully.');
            msg.installComplete();
            isRunning = false;
            return;
        }

        currentStep = steps[currentStepIndex];

        await new Promise(r => setTimeout(r, Math.random() * 1000 + 500));
        await executeCurrentStep();
    } catch (error) {
        console.log(error);
        msg.installError(`Error executing step ${step}: ${error.message}`);
        isRunning = false;
    }
}

export async function getIsRunning(): Promise<boolean> {
    return isRunning;
}

export async function getCurrentStep(): Promise<Steps | undefined> {
    return currentStep;
}

export async function getSteps(): Promise<Steps[]> {
    return steps;
}

export async function getCurrentStepIndex(): Promise<number> {
    return currentStepIndex;
}

export async function getOptions(): Promise<Options | undefined> {
    return options;
}