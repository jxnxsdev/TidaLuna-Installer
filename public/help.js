/**
 * @fileoverview Help system for displaying documentation and support links
 */

import { showPopup } from "./popup.js"

/**
 * Initializes the help button and its event listener
 */
export function initHelp() {
  const helpBtn = document.getElementById("help-btn")
  if (helpBtn) {
    helpBtn.addEventListener("click", showHelpPopup)
  }
}

/**
 * Displays the help popup with links to documentation and support
 */
function showHelpPopup() {
  showPopup({
    title: "TidaLuna Help",
    message: `
      <p>Need help with TidaLuna? Check out these resources:</p>
      <p><strong>Wiki:</strong> Learn how to install TidaLuna</p>
      <p><strong>Discord Server:</strong> Get support from the community and developers.</p>
    `,
    type: "info",
    buttons: [
      {
        label: "Wiki",
        action: "open_url",
        value: "https://luna-wiki.jxnxsdev.de",
        color: "primary",
      },
      {
        label: "Discord",
        action: "open_url",
        value: "https://discord.gg/jK3uHrJGx4",
        color: "primary",
      },
    ],
  })
}
