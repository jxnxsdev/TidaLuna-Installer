/**
 * @fileoverview Screen navigation utilities
 */

import { getInstallationState } from "./app-state.js"

/**
 * Shows the setup screen and hides the progress screen
 */
export function showSetupScreen() {
  const setupScreen = document.getElementById("setup-screen")
  const progressScreen = document.getElementById("progress-screen")
  setupScreen.classList.add("active")
  progressScreen.classList.remove("active")
}

/**
 * Shows the progress screen and hides the setup screen
 */
export function showProgressScreen() {
  const setupScreen = document.getElementById("setup-screen")
  const progressScreen = document.getElementById("progress-screen")
  setupScreen.classList.remove("active")
  progressScreen.classList.add("active")
  updateBackButtonState()
}

/**
 * Clears all steps from the display
 */
export function clearSteps() {
  const stepsContainer = document.getElementById("steps-container")
  stepsContainer.innerHTML = ""
}

/**
 * Updates the back button state based on installation progress
 */
function updateBackButtonState() {
  const backBtn = document.getElementById("back-btn")
  const isInstalling = getInstallationState()
  backBtn.disabled = isInstalling

  if (isInstalling) {
    backBtn.classList.add("disabled")
  } else {
    backBtn.classList.remove("disabled")
  }
}
