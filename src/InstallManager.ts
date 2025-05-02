import { Steps } from './enums/Steps';
import { WebsocketMessageTypes } from './enums/WebsocketMessageTypes';
import { StepLog } from './types/StepLog';
import { Options } from './types/Options';
import { sendMessageToFrontend } from '.';
import * as msg from './utils/MessageHelper';

let currentStep: Steps = null;
let steps: Steps[] = null;
let currentStepIndex: number = null;
let options: Options = null;
let isRunning = false;

/**
* @description Set's the options used for the (un)installation process.
* @param newOptions The options to set.
* @returns {Promise<void>} A promise that resolves when the options are set.
*/
export async function setOptions(newOptions: Options): Promise<void> {
    if (isRunning) {
        msg.globalError('Installation process is already running.', new Error('InstallManager::setOptions:: Installation process is already running.'));
        return;
    }
    options = newOptions;
    msg.globalLog('Options set successfully.');
}

/**
* @description Generates the install steps required for the installation process.
* @returns {Promise<void>} A promise that resolves when the install steps are generated.
*/
export async function generateInstallSteps(): Promise<void> {
    if (isRunning) {
        msg.globalError('Installation process is already running.', new Error('InstallManager::generateInstallSteps:: Installation process is already running.'));
        return;
    }
    if (!options) {
        msg.globalError('Options are not set.', new Error('InstallManager::generateInstallSteps:: Options are not set.'));
        return;
    }

    steps = options.action === 'install' ? [
        Steps.SETUP,
        Steps.KILLING_TIDAL,
        Steps.CHECK_IF_INSTALLED,
        Steps.UNINSTALLING_LUNA,
        Steps.UNINSTALLING_NEPTUNE,
        Steps.DOWNLOADING_LUNA,
        Steps.EXTRACTING_LUNA,
        Steps.COPYING_ASAR_INSTALL,
        Steps.INSERTING_LUNA
    ] : options.action === 'uninstall' ? [
        Steps.KILLING_TIDAL,
        Steps.CHECK_IF_INSTALLED,
        Steps.UNINSTALLING_LUNA,
        Steps.UNINSTALLING_NEPTUNE,
        Steps.COPYING_ASAR_UNINSTALL
    ] : null;

    if (!steps) {
        msg.globalError('Invalid action.', new Error('InstallManager::generateInstallSteps:: Invalid action.'));
    }
}
