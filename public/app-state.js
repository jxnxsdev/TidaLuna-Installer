/**
 * @fileoverview Application state management
 */

const state = {
  isInstalling: false,
  isInstalled: false,
  selectedRelease: null,
  selectedVersion: null,
  currentSteps: [],
}

/**
 * Gets the current installation state
 * @returns {boolean} True if installation is in progress
 */
export function getInstallationState() {
  return state.isInstalling
}

/**
 * Sets the installation state
 * @param {boolean} value - The installation state
 */
export function setInstallationState(value) {
  state.isInstalling = value
}

/**
 * Gets the installed state
 * @returns {boolean} True if TidaLuna is installed
 */
export function getInstalledState() {
  return state.isInstalled
}

/**
 * Sets the installed state
 * @param {boolean} value - The installed state
 */
export function setInstalledState(value) {
  state.isInstalled = value
}

/**
 * Gets the selected release
 * @returns {Object|null} The selected release object
 */
export function getSelectedRelease() {
  return state.selectedRelease
}

/**
 * Sets the selected release
 * @param {Object} release - The release object
 */
export function setSelectedRelease(release) {
  state.selectedRelease = release
}

/**
 * Gets the selected version
 * @returns {Object|null} The selected version object
 */
export function getSelectedVersion() {
  return state.selectedVersion
}

/**
 * Sets the selected version
 * @param {Object} version - The version object
 */
export function setSelectedVersion(version) {
  state.selectedVersion = version
}

/**
 * Gets the current steps array
 * @returns {Array} The current steps
 */
export function getCurrentSteps() {
  return state.currentSteps
}

/**
 * Adds a step to the current steps array
 * @param {string} step - The step to add
 */
export function addCurrentStep(step) {
  if (!state.currentSteps.includes(step)) {
    state.currentSteps.push(step)
  }
}

/**
 * Clears all current steps
 */
export function clearCurrentSteps() {
  state.currentSteps = []
}
