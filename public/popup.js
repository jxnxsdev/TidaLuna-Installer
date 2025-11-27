/**
 * @fileoverview Popup modal component for displaying messages and actions from the backend
 */

/**
 * @typedef {Object} PopupButton
 * @property {string} label - Label of the button
 * @property {'open_url' | 'send_websocket_message' | 'send_api_request'} action - Action to perform when clicked
 * @property {string} value - Value associated with the action (URL, message, or API endpoint)
 * @property {'default' | 'primary' | 'danger'} color - Button color style
 */

/**
 * @typedef {Object} Popup
 * @property {string} title - Title of the popup
 * @property {string} message - Message/body of the popup (supports HTML markup)
 * @property {'info' | 'warning' | 'error' | 'success'} type - Type of popup, affects the icon shown
 * @property {PopupButton[]} [buttons] - Optional array of buttons to show in the popup
 */

/**
 * Shows a popup modal with the provided content and actions
 * @param {Popup} popupData - The popup data containing title, message, type, and optional buttons
 */
export function showPopup(popupData) {
  const popup = createPopupElement(popupData)
  document.body.appendChild(popup)

  setTimeout(() => {
    popup.classList.add("show")
  }, 10)
}

/**
 * Creates the popup DOM element
 * @param {Popup} popupData - The popup data
 * @returns {HTMLElement} The popup element
 * @private
 */
function createPopupElement(popupData) {
  const overlay = document.createElement("div")
  overlay.className = "popup-overlay"

  const popup = document.createElement("div")
  popup.className = `popup popup-${popupData.type}`

  const icon = getIconForType(popupData.type)

  const header = document.createElement("div")
  header.className = "popup-header"
  header.innerHTML = `
    ${icon}
    <h2 class="popup-title">${escapeHtml(popupData.title)}</h2>
    <button class="popup-close-btn" aria-label="Close popup">×</button>
  `

  const content = document.createElement("div")
  content.className = "popup-content"
  content.innerHTML = popupData.message

  const footer = document.createElement("div")
  footer.className = "popup-footer"

  const closeButton = document.createElement("button")
  closeButton.className = "popup-btn popup-btn-default"
  closeButton.textContent = "Close"
  footer.appendChild(closeButton)

  if (popupData.buttons && popupData.buttons.length > 0) {
    popupData.buttons.forEach((buttonData) => {
      const button = document.createElement("button")
      button.className = `popup-btn popup-btn-${buttonData.color}`
      button.textContent = buttonData.label

      button.addEventListener("click", () => {
        handleButtonAction(buttonData)
        closePopup(overlay)
      })

      footer.appendChild(button)
    })
  }

  popup.appendChild(header)
  popup.appendChild(content)
  popup.appendChild(footer)
  overlay.appendChild(popup)

  overlay.addEventListener("click", (e) => {
    if (e.target === overlay) {
      closePopup(overlay)
    }
  })

  const closeBtn = header.querySelector(".popup-close-btn")
  closeBtn.addEventListener("click", () => closePopup(overlay))

  const closeButtonInFooter = footer.querySelector(".popup-btn-default")
  closeButtonInFooter.addEventListener("click", () => closePopup(overlay))

  return overlay
}

/**
 * Handles the action triggered by a popup button
 * @param {PopupButton} buttonData - The button data containing action and value
 * @private
 */
function handleButtonAction(buttonData) {
  switch (buttonData.action) {
    case "open_url":
      window.open(buttonData.value, "_blank")
      break

    case "send_websocket_message":
      if (window.socket && window.socket.readyState === WebSocket.OPEN) {
        window.socket.send(buttonData.value)
      }
      break

    case "send_api_request":
      fetch(buttonData.value).catch((error) => {
        console.error("API request failed:", error)
      })
      break

    default:
      console.warn("Unknown button action:", buttonData.action)
  }
}

/**
 * Gets the SVG icon for the popup type
 * @param {'info' | 'warning' | 'error' | 'success'} type - The popup type
 * @returns {string} SVG icon HTML string
 * @private
 */
function getIconForType(type) {
  const icons = {
    info: '<svg class="popup-icon popup-icon-info" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="12" cy="12" r="10"></circle><line x1="12" y1="16" x2="12" y2="12"></line><line x1="12" y1="8" x2="12.01" y2="8"></line></svg>',
    warning:
      '<svg class="popup-icon popup-icon-warning" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M10.29 3.86L1.82 18a2 2 0 0 0 1.71 3.05h16.94a2 2 0 0 0 1.71-3.05L13.71 3.86a2 2 0 0 0-3.42 0z"></path><line x1="12" y1="9" x2="12" y2="13"></line><line x1="12" y1="17" x2="12.01" y2="17"></line></svg>',
    error:
      '<svg class="popup-icon popup-icon-error" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="12" cy="12" r="10"></circle><line x1="15" y1="9" x2="9" y2="15"></line><line x1="9" y1="9" x2="15" y2="15"></line></svg>',
    success:
      '<svg class="popup-icon popup-icon-success" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polyline points="20 6 9 17 4 12"></polyline></svg>',
  }

  return icons[type] || icons.info
}

/**
 * Closes a popup by removing it from the DOM with animation
 * @param {HTMLElement} overlay - The popup overlay element
 * @private
 */
function closePopup(overlay) {
  overlay.classList.remove("show")
  setTimeout(() => {
    overlay.remove()
  }, 300)
}

/**
 * Escapes HTML special characters to prevent XSS
 * @param {string} text - The text to escape
 * @returns {string} The escaped text
 * @private
 */
function escapeHtml(text) {
  const div = document.createElement("div")
  div.textContent = text
  return div.innerHTML
}
