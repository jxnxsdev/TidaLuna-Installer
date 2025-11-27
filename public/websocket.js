/**
 * @fileoverview WebSocket connection and message handling
 */

import { WebsocketMessageTypes } from "./constants.js"
import {
  handleInstallLog,
  handleStepLog,
  handleStepUpdate,
  handleInstallationComplete,
  handleInstallationStart,
  handleInstallationError,
  handlePopupMessage,
} from "./message-handlers.js"
import { addLog } from "./logger.js"

let socket = null

/**
 * Initializes WebSocket connection to the server
 */
export function initWebSocket() {
  const protocol = window.location.protocol === "https:" ? "wss:" : "ws:"
  const wsUrl = `${protocol}//${window.location.host}`

  socket = new WebSocket(wsUrl)

  socket.onopen = () => {
    addLog("Connected to server")
  }

  socket.onclose = () => {
    addLog("Disconnected from server")
    setTimeout(initWebSocket, 3000)
  }

  socket.onerror = (error) => {
    addLog("WebSocket error: " + error.message)
  }

  socket.onmessage = (event) => {
    handleWebSocketMessage(event.data)
  }
}

/**
 * Retrieves the current WebSocket instance
 * @returns {WebSocket} The active WebSocket connection
 */
export function getSocket() {
  return socket
}

/**
 * Handles incoming WebSocket messages
 * @param {string} data - The message data as JSON string
 */
function handleWebSocketMessage(data) {
  try {
    const message = JSON.parse(data)

    switch (message.type) {
      case WebsocketMessageTypes.INSTALL_LOG:
        handleInstallLog(message.data)
        break
      case WebsocketMessageTypes.STEP_LOG:
        handleStepLog(message.data)
        break
      case WebsocketMessageTypes.STEP_UPDATE:
        handleStepUpdate(message.data)
        break
      case WebsocketMessageTypes.INSTALLATION_COMPLETE:
        handleInstallationComplete()
        break
      case WebsocketMessageTypes.INSTALLATION_START:
        handleInstallationStart(message.data)
        break
      case WebsocketMessageTypes.INSTALLATION_ERROR:
        handleInstallationError(message.data)
        break
      case WebsocketMessageTypes.POPUP_MESSAGE:
        handlePopupMessage(message.data)
        break
      default:
        console.log("Unknown message type:", message.type)
    }
  } catch (error) {
    console.error("Error parsing WebSocket message:", error)
  }
}
