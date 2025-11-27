/**
 * @fileoverview WebSocket message handlers
 */

import { addLog } from "./logger.js"
import { addStep, markStepsAsComplete, markCurrentStepAsError } from "./steps.js"
import { checkIfInstalled } from "./state.js"
import { setInstallationState } from "./app-state.js"
import { showPopup } from "./popup.js"

/**
 * Handles installation log messages
 * @param {Object} data - The message data
 * @param {string} data.message - The log message
 * @param {boolean} data.isError - Whether the log is an error
 * @param {string} [data.error] - Error details if applicable
 */
export function handleInstallLog(data) {
  if (data.isError) {
    addLog(`Error: ${data.message}${data.error ? ` - ${data.error}` : ""}`)
  } else {
    addLog(data.message)
  }
}

/**
 * Handles step-specific log messages
 * @param {Object} data - The message data
 * @param {string} data.step - The step identifier
 * @param {string} data.message - The log message
 * @param {boolean} data.isError - Whether the log is an error
 * @param {string} [data.error] - Error details if applicable
 */
export function handleStepLog(data) {
  const stepLogs = document.getElementById(`step-logs-${data.step}`)

  if (stepLogs) {
    const logEntry = document.createElement("div")
    logEntry.className = data.isError ? "step-log error" : "step-log"
    logEntry.textContent = data.isError ? `Error: ${data.message}${data.error ? ` - ${data.error}` : ""}` : data.message

    stepLogs.appendChild(logEntry)
    stepLogs.scrollTop = stepLogs.scrollHeight
  }
}

/**
 * Handles step update messages marking a new step as current
 * @param {string} step - The step identifier
 */
export function handleStepUpdate(step) {
  const steps = document.querySelectorAll(".step")
  steps.forEach((stepElement) => {
    stepElement.classList.remove("current")
    stepElement.classList.add("success")
  })

  addStep(step, true)
}

/**
 * Handles installation completion
 */
export function handleInstallationComplete() {
  addLog("Installation completed successfully!")

  setInstallationState(false)

  const backBtn = document.getElementById("back-btn")
  backBtn.disabled = false
  backBtn.classList.remove("disabled")

  markStepsAsComplete()

  const stepsContainer = document.getElementById("steps-container")
  const completionMessage = document.createElement("div")
  completionMessage.className = "completion-message"
  completionMessage.textContent = "✅ Installation Complete! You can now go back to the setup screen."
  stepsContainer.appendChild(completionMessage)

  checkIfInstalled()
}

/**
 * Handles installation start notification
 * @param {Object} data - The message data
 * @param {string} data.action - The action type (install/uninstall)
 * @param {Array} [data.steps] - The steps to perform
 */
export function handleInstallationStart(data) {
  addLog(`Starting ${data.action}...`)

  const stepsContainer = document.getElementById("steps-container")
  stepsContainer.innerHTML = ""

  if (data.steps && data.steps.length > 0) {
    addStep(data.steps[0], true)
  }
}

/**
 * Handles installation errors
 * @param {string} message - The error message
 */
export function handleInstallationError(message) {
  addLog(`Installation failed: ${message}`)
  setInstallationState(false)

  const backBtn = document.getElementById("back-btn")
  backBtn.disabled = false
  backBtn.classList.remove("disabled")

  markCurrentStepAsError()
}

/**
 * Handles popup messages from backend
 * @param {Object} data - The popup data
 * @param {string} data.title - Title of the popup
 * @param {string} data.message - Message/body of the popup
 * @param {'info' | 'warning' | 'error' | 'success'} data.type - Type of popup
 * @param {Array} [data.buttons] - Optional array of popup buttons
 */
export function handlePopupMessage(data) {
  showPopup(data)
}
