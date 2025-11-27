/**
 * @fileoverview Main entry point for the TidaLuna Installer application
 */

import { initTheme } from "./theme.js"
import { initWebSocket } from "./websocket.js"
import { initEventListeners } from "./events.js"
import { initParticles } from "./particles.js"
import { checkCurrentState, checkIfInstalled, fetchReleases } from "./state.js"
import { initEasterEgg } from "./easter-egg.js"
import { initHelp } from "./help.js"

/**
 * Initializes the application on DOM content loaded
 */
function initializeApp() {
  initTheme()
  initWebSocket()
  initEventListeners()
  initParticles()
  checkCurrentState()
  fetchReleases()
  checkIfInstalled()
  initEasterEgg()
  initHelp()
}

document.addEventListener("DOMContentLoaded", initializeApp)
