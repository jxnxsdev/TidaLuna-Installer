import { Steps } from "../enums/Steps"

export type InstallInfo = {
    steps: Steps[];
    currentStep: Steps;
    currentStepIndex: number;
    action: "uninstall" | "install";
}