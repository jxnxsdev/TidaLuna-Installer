/**
 * @fileoverview Server state management and fetching
 */

import { setInstallationState, setInstalledState } from "./app-state.js"
import { addStep, clearSteps } from "./steps.js"
import { addLog } from "./logger.js"
import { updateButtonStates } from "./ui.js"
import { renderReleaseCards } from "./releases.js"
import { showProgressScreen } from "./screens.js"

/**
 * Checks if TidaLuna is currently installed
 */
export function checkIfInstalled() {
  fetch("/isInstalled")
    .then((response) => response.json())
    .then((data) => {
      setInstalledState(data.isInstalled)
      updateButtonStates()
    })
    .catch((error) => {
      addLog("Error checking installation status: " + error.message)
    })
}

/**
 * Checks the current state from the server
 */
export function checkCurrentState() {
  fetch("/state")
    .then((response) => response.json())
    .then((data) => {
      if (data.isRunning) {
        setInstallationState(true)
        showProgressScreen()

        clearSteps()

        if (data.steps && data.steps.length > 0) {
          for (let i = 0; i <= data.currentStepIndex; i++) {
            if (i < data.steps.length) {
              addStep(data.steps[i], i === data.currentStepIndex)
            }
          }
        }
      }
    })
    .catch((error) => {
      addLog("Error fetching state: " + error.message)
    })
}

/**
 * Fetches available releases from the server
 */
export function fetchReleases() {
  fetch("/releases")
    .then((response) => response.json())
    .then((data) => {
      renderReleaseCards(data)
    })
    .catch((error) => {
      addLog("Error fetching releases: " + error.message)
    })
}
