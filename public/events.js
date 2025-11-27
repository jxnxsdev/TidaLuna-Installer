/**
 * @fileoverview Event listener setup
 */

import { toggleTheme } from "./theme.js"
import { initiateInstall, initiateUninstall } from "./installation.js"
import { showSetupScreen } from "./screens.js"
import { getInstallationState } from "./app-state.js"
import { addLog } from "./logger.js"

/**
 * Initializes all event listeners for the application
 */
export function initEventListeners() {
  const installBtn = document.getElementById("install-btn")
  const uninstallBtn = document.getElementById("uninstall-btn")
  const backBtn = document.getElementById("back-btn")
  const themeToggle = document.querySelector(".theme-toggle")
  const accordionHeader = document.querySelector(".accordion-header")
  const accordion = document.querySelector(".accordion")

  installBtn.addEventListener("click", initiateInstall)

  uninstallBtn.addEventListener("click", initiateUninstall)

  backBtn.addEventListener("click", () => {
    if (!getInstallationState()) {
      showSetupScreen()
    } else {
      addLog("Cannot go back during installation")
    }
  })

  themeToggle.addEventListener("click", toggleTheme)

  accordionHeader.addEventListener("click", () => {
    accordion.classList.toggle("active")
  })
}
