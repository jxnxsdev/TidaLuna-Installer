/**
 * @fileoverview Theme management utilities
 */

/**
 * Initializes the application theme from localStorage or defaults to dark mode with 1/10 chance of spectrum
 */
export function initTheme() {
  const savedTheme = localStorage.getItem("theme")
  if (savedTheme) {
    applyTheme(savedTheme)
  } else {
    const randomChance = Math.random()
    const defaultTheme = randomChance < 0.1 ? "spectrum" : "dark"
    applyTheme(defaultTheme)
    localStorage.setItem("theme", defaultTheme)
  }
}

/**
 * Applies a theme to the document
 * @param {string} theme - The theme to apply ('light', 'dark', or 'spectrum')
 */
function applyTheme(theme) {
  document.body.classList.remove("dark-theme", "spectrum-theme")

  if (theme === "light") {
    // light theme has no classes
  } else if (theme === "spectrum") {
    document.body.classList.add("spectrum-theme")
  } else {
    document.body.classList.add("dark-theme")
  }
}

/**
 * Cycles through themes: light -> dark -> spectrum -> light
 */
export function toggleTheme() {
  const isDark = document.body.classList.contains("dark-theme")
  const isSpectrum = document.body.classList.contains("spectrum-theme")

  let nextTheme = "light"
  if (!isDark && !isSpectrum) {
    nextTheme = "dark"
  } else if (isDark) {
    nextTheme = "spectrum"
  }

  applyTheme(nextTheme)
  localStorage.setItem("theme", nextTheme)
}

/**
 * Checks if dark theme is currently active
 * @returns {boolean} True if dark theme is active
 */
export function isDarkTheme() {
  return document.body.classList.contains("dark-theme")
}

/**
 * Checks if spectrum theme is currently active
 * @returns {boolean} True if spectrum theme is active
 */
export function isSpectrumTheme() {
  return document.body.classList.contains("spectrum-theme")
}

/**
 * Gets the current active theme
 * @returns {string} The current theme ('light', 'dark', or 'spectrum')
 */
export function getCurrentTheme() {
  if (document.body.classList.contains("spectrum-theme")) {
    return "spectrum"
  }
  if (document.body.classList.contains("dark-theme")) {
    return "dark"
  }
  return "light"
}
