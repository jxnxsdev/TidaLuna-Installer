/**
 * @fileoverview UI state management and updates
 */

import { getInstalledState } from "./app-state.js"

/**
 * Updates button states based on installation status
 */
export function updateButtonStates() {
  const installBtn = document.getElementById("install-btn")
  const uninstallBtn = document.getElementById("uninstall-btn")
  const isInstalled = getInstalledState()

  if (isInstalled) {
    installBtn.textContent = "Reinstall"
    uninstallBtn.disabled = false
  } else {
    installBtn.textContent = "Install"
    uninstallBtn.disabled = true
  }
}
