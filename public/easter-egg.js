/**
 * @fileoverview Easter egg functionality - WTF mode
 */

import { addLog } from "./logger.js"

/**
 * Initializes the WTF mode easter egg triggered by typing 'wtf'
 */
export function initEasterEgg() {
  let keys = []
  const wtfSequence = "wtf"

  window.addEventListener("keydown", (e) => {
    if (/^[a-z]$/i.test(e.key)) {
      keys.push(e.key.toLowerCase())

      if (keys.length > 3) {
        keys = keys.slice(-3)
      }

      if (keys.join("") === wtfSequence) {
        executeWtfMode()
        keys = []
      }
    }
  })
}

/**
 * Executes the WTF mode effect
 */
function executeWtfMode() {
  const fonts = [
    "Arial",
    "Verdana",
    "Helvetica",
    "Times New Roman",
    "Courier New",
    "Georgia",
    "Trebuchet MS",
    "Comic Sans MS",
    "Impact",
    "Lucida Console",
    "Tahoma",
    "Palatino Linotype",
    "Segoe UI",
    "Garamond",
    "Monaco",
  ]

  const randomColor = () =>
    "#" +
    Math.floor(Math.random() * 16777215)
      .toString(16)
      .padStart(6, "0")

  const randomFont = () => fonts[Math.floor(Math.random() * fonts.length)]

  const randomOffset = () => Math.floor(Math.random() * 61) - 30

  const randomAngle = () => Math.floor(Math.random() * 360)

  const randomGradient = () => {
    const color1 = randomColor()
    const color2 = randomColor()
    const angle = randomAngle()
    return `linear-gradient(${angle}deg, ${color1}, ${color2})`
  }

  document.querySelectorAll("*").forEach((el) => {
    el.style.backgroundImage = randomGradient()
    el.style.backgroundSize = "cover"
    el.style.color = randomColor()
    el.style.fontFamily = randomFont()

    const baseFontSize = window.getComputedStyle(el).fontSize
    if (baseFontSize) {
      const numericSize = Number.parseFloat(baseFontSize)
      const newSize = Math.max(6, numericSize + randomOffset())
      el.style.fontSize = newSize + "px"
    }

    if (el.tagName === "IMG") {
      el.style.filter = `hue-rotate(${Math.floor(Math.random() * 360)}deg)`

      const baseWidth = el.offsetWidth
      const baseHeight = el.offsetHeight
      const widthOffset = randomOffset()
      const heightOffset = randomOffset()

      el.style.width = Math.max(10, baseWidth + widthOffset) + "px"
      el.style.height = Math.max(10, baseHeight + heightOffset) + "px"
    }
  })

  addLog("WTF mode activated! 🤪")
}
