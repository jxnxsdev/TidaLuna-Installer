/**
 * @fileoverview Installation and uninstallation logic
 */

import { getSelectedRelease, getSelectedVersion, setInstallationState } from "./app-state.js"
import { clearSteps, showProgressScreen } from "./screens.js"
import { addLog } from "./logger.js"

/**
 * Starts the installation process
 * @param {Object} options - Installation options
 * @param {string} options.action - The action type (install)
 * @param {string} options.downloadUrl - The download URL for the version
 * @param {string} [options.overwritePath] - Optional custom install path
 */
export function startInstallation(options) {
  clearSteps()

  const queryParams = new URLSearchParams()
  for (const key in options) {
    queryParams.append(key, options[key])
  }

  fetch(`/setOptions?${queryParams.toString()}`)
    .then((response) => {
      if (!response.ok) {
        throw new Error("Failed to set options")
      }
      return fetch("/start")
    })
    .then((response) => {
      if (!response.ok) {
        throw new Error("Failed to start installation")
      }
      setInstallationState(true)
      showProgressScreen()
    })
    .catch((error) => {
      addLog("Error: " + error.message)
    })
}

/**
 * Starts the uninstallation process
 * @param {Object} options - Uninstallation options
 * @param {string} options.action - The action type (uninstall)
 */
export function startUninstallation(options) {
  clearSteps()

  const queryParams = new URLSearchParams()
  for (const key in options) {
    queryParams.append(key, options[key])
  }

  fetch(`/setOptions?${queryParams.toString()}`)
    .then((response) => {
      if (!response.ok) {
        throw new Error("Failed to set options")
      }
      return fetch("/start")
    })
    .then((response) => {
      if (!response.ok) {
        throw new Error("Failed to start uninstallation")
      }
      setInstallationState(true)
      showProgressScreen()
    })
    .catch((error) => {
      addLog("Error: " + error.message)
    })
}

/**
 * Initiates installation based on user selections
 */
export function initiateInstall() {
  const selectedRelease = getSelectedRelease()
  const selectedVersion = getSelectedVersion()
  const customPath = document.getElementById("custom-path")

  if (!selectedRelease || !selectedVersion) {
    addLog("Please select a release channel and version")
    return
  }

  const overwritePath = customPath.value.trim()

  const options = {
    action: "install",
    downloadUrl: selectedVersion.download,
    ...(overwritePath && { overwritePath }),
  }

  startInstallation(options)
}

/**
 * Initiates uninstallation
 */
export function initiateUninstall() {
  const options = {
    action: "uninstall",
  }

  startUninstallation(options)
}
