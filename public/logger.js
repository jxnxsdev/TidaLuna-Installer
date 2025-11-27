/**
 * @fileoverview Logging utilities
 */

/**
 * Gets the DOM element for the global log
 * @returns {HTMLElement} The global log container
 */
function getGlobalLogElement() {
  return document.getElementById("global-log")
}

/**
 * Gets the DOM element for the progress screen global log
 * @returns {HTMLElement} The progress screen log container
 */
function getProgressLogElement() {
  return document.getElementById("progress-global-log")
}

/**
 * Adds a timestamped log entry to both log displays
 * @param {string} message - The message to log
 */
export function addLog(message) {
  const logEntry = document.createElement("div")
  logEntry.className = "log-entry"
  logEntry.textContent = `[${new Date().toLocaleTimeString()}] ${message}`

  const globalLog = getGlobalLogElement()
  const progressLog = getProgressLogElement()

  globalLog.appendChild(logEntry)
  progressLog.appendChild(logEntry.cloneNode(true))

  globalLog.scrollTop = globalLog.scrollHeight
  progressLog.scrollTop = progressLog.scrollHeight
}
