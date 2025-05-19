// DOM Elements
const setupScreen = document.getElementById("setup-screen")
const progressScreen = document.getElementById("progress-screen")
const installBtn = document.getElementById("install-btn")
const uninstallBtn = document.getElementById("uninstall-btn")
const backBtn = document.getElementById("back-btn")
const customPath = document.getElementById("custom-path")
const globalLog = document.getElementById("global-log")
const progressGlobalLog = document.getElementById("progress-global-log")
const stepsContainer = document.getElementById("steps-container")
const themeToggle = document.querySelector(".theme-toggle")
const accordion = document.querySelector(".accordion")
const accordionHeader = document.querySelector(".accordion-header")
const releaseCardsContainer = document.getElementById("release-cards")

// Update the WebSocket connection section to include version selection
let socket
let isInstalling = false
let releases = []
let selectedRelease = null
let selectedVersion = null
let isInstalled = false
let currentSteps = [] // Track current steps

// Enum values from your source code
const Steps = {
  SETUP: "SETUP",
  DOWNLOADING_LUNA: "DOWNLOADING_LUNA",
  EXTRACTING_LUNA: "EXTRACTING_LUNA",
  COPYING_ASAR_INSTALL: "COPYING_ASAR_INSTALL",
  INSERTING_LUNA: "INSERTING_LUNA",
  UNINSTALLING: "UNINSTALLING",
  COPYING_ASAR_UNINSTALL: "COPYING_ASAR_UNINSTALL",
  KILLING_TIDAL: "KILLING_TIDAL",
}

const WebsocketMessageTypes = {
  INSTALL_LOG: "install_log",
  STEP_LOG: "step_log",
  STEP_UPDATE: "step_update",
  INSTALLATION_COMPLETE: "installation_complete",
  INSTALLATION_START: "installation_start",
  INSTALLATION_ERROR: "installation_error",
}

// Step display names
const StepNames = {
  [Steps.SETUP]: "Setup",
  [Steps.KILLING_TIDAL]: "Stopping TIDAL",
  [Steps.DOWNLOADING_LUNA]: "Downloading TidaLuna",
  [Steps.EXTRACTING_LUNA]: "Extracting TidaLuna",
  [Steps.COPYING_ASAR_INSTALL]: "Copying ASAR file",
  [Steps.COPYING_ASAR_UNINSTALL]: "Copying ASAR file",
  [Steps.INSERTING_LUNA]: "Installing TidaLuna",
  [Steps.UNINSTALLING]: "Uninstalling TidaLuna",
}

// Initialize the application
document.addEventListener("DOMContentLoaded", () => {
  initTheme()
  initWebSocket()
  initEventListeners()
  initParticles()
  checkCurrentState()
  fetchReleases()
  checkIfInstalled()
  initEasterEgg()
})

// Add the Easter Egg function
function initEasterEgg() {
  let keys = []
  const wtfSequence = "wtf"

  window.addEventListener("keydown", (e) => {
    // Only track alphabetic keys
    if (/^[a-z]$/i.test(e.key)) {
      keys.push(e.key.toLowerCase())

      // Keep only the last 3 keys
      if (keys.length > 3) {
        keys = keys.slice(-3)
      }

      // Check if the sequence matches "wtf"
      if (keys.join("") === wtfSequence) {
        executeWtfMode()
        keys = [] // Reset after triggering
      }
    }
  })
}

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
      const newSize = Math.max(6, numericSize + randomOffset()) // minimum 6px
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

  // Add a log entry for fun
  addLog("WTF mode activated! 🤪")
}

// Update the initTheme function to default to dark mode
function initTheme() {
  const savedTheme = localStorage.getItem("theme")
  if (savedTheme === "light") {
    document.body.classList.remove("dark-theme")
  } else {
    // Default to dark theme
    document.body.classList.add("dark-theme")
    localStorage.setItem("theme", "dark")
  }
}

// Initialize WebSocket connection
function initWebSocket() {
  const protocol = window.location.protocol === "https:" ? "wss:" : "ws:"
  const wsUrl = `${protocol}//${window.location.host}`

  socket = new WebSocket(wsUrl)

  socket.onopen = () => {
    addLog("Connected to server")
  }

  socket.onclose = () => {
    addLog("Disconnected from server")
    // Try to reconnect after 3 seconds
    setTimeout(initWebSocket, 3000)
  }

  socket.onerror = (error) => {
    addLog("WebSocket error: " + error.message)
  }

  socket.onmessage = (event) => {
    handleWebSocketMessage(event.data)
  }
}

// Initialize event listeners
function initEventListeners() {
  // Update the install button event listener to use the selected version
  installBtn.addEventListener("click", () => {
    if (!selectedRelease || !selectedVersion) {
      addLog("Please select a release channel and version")
      return
    }

    const overwritePath = customPath.value.trim()

    const options = {
      action: "install",
      downloadUrl: selectedVersion.download,
      ...(overwritePath && { overwritePath }),
    }

    startInstallation(options)
  })

  // Uninstall button
  uninstallBtn.addEventListener("click", () => {
    const options = {
      action: "uninstall",
    }

    startUninstallation(options)
  })

  // Back button
  backBtn.addEventListener("click", () => {
    if (!isInstalling) {
      showSetupScreen()
    } else {
      addLog("Cannot go back during installation")
    }
  })

  // Theme toggle
  themeToggle.addEventListener("click", () => {
    document.body.classList.toggle("dark-theme")
    localStorage.setItem("theme", document.body.classList.contains("dark-theme") ? "dark" : "light")
  })

  // Accordion
  accordionHeader.addEventListener("click", () => {
    accordion.classList.toggle("active")
  })
}

// Check if TidaLuna is installed
function checkIfInstalled() {
  fetch("/isInstalled")
    .then((response) => response.json())
    .then((data) => {
      isInstalled = data.isInstalled
      updateButtonStates()
    })
    .catch((error) => {
      addLog("Error checking installation status: " + error.message)
    })
}

// Update button states based on installation status
function updateButtonStates() {
  if (isInstalled) {
    installBtn.textContent = "Reinstall"
    uninstallBtn.disabled = false
  } else {
    installBtn.textContent = "Install"
    uninstallBtn.disabled = true
  }

  // Update back button state based on installation progress
  backBtn.disabled = isInstalling
}

// Check current state from the server
function checkCurrentState() {
  fetch("/state")
    .then((response) => response.json())
    .then((data) => {
      if (data.isRunning) {
        isInstalling = true
        showProgressScreen()

        // If there are steps in progress, render them
        if (data.steps && data.steps.length > 0) {
          // Clear steps container
          stepsContainer.innerHTML = ""

          // Add each step that has been started
          for (let i = 0; i <= data.currentStepIndex; i++) {
            if (i < data.steps.length) {
              addStep(data.steps[i], i === data.currentStepIndex)
            }
          }
        }
      }
    })
    .catch((error) => {
      addLog("Error fetching state: " + error.message)
    })
}

// Fetch available releases
function fetchReleases() {
  fetch("/releases")
    .then((response) => response.json())
    .then((data) => {
      releases = data
      renderReleaseCards(releases)
    })
    .catch((error) => {
      addLog("Error fetching releases: " + error.message)
    })
}

// Update the renderReleaseCards function to add a select button
function renderReleaseCards(releases) {
  if (!releases || !releases.length) {
    addLog("No release channels available")
    releaseCardsContainer.innerHTML =
      '<div class="release-card"><div class="release-card-header"><h3>No releases found</h3></div></div>'
    return
  }

  // Clear existing cards
  releaseCardsContainer.innerHTML = ""

  // Add cards for each release channel
  releases.forEach((release) => {
    // Create a wrapper for the card
    const wrapper = document.createElement("div")
    wrapper.className = "release-card-wrapper"

    const card = document.createElement("div")
    card.className = "release-card"
    card.dataset.id = release.id

    const cardHeader = document.createElement("div")
    cardHeader.className = "release-card-header"

    const title = document.createElement("h3")
    title.textContent = release.name

    cardHeader.appendChild(title)

    const cardBody = document.createElement("div")
    cardBody.className = "release-card-body"

    // Create version dropdown with a custom select container
    const selectContainer = document.createElement("div")
    selectContainer.className = "custom-select-container"

    const versionSelect = document.createElement("select")
    versionSelect.className = "version-select"
    versionSelect.id = `version-select-${release.id}`

    // Add options for each version
    if (release.versions && release.versions.length > 0) {
      release.versions.forEach((version, index) => {
        const option = document.createElement("option")
        option.value = index
        option.textContent = version.version
        versionSelect.appendChild(option)
      })
    } else {
      // Fallback if no versions
      const option = document.createElement("option")
      option.value = "-1"
      option.textContent = "No versions available"
      option.disabled = true
      versionSelect.appendChild(option)
      versionSelect.disabled = true
    }

    // Add dropdown arrow
    const dropdownArrow = document.createElement("div")
    dropdownArrow.className = "dropdown-arrow"
    dropdownArrow.innerHTML = "▼"

    selectContainer.appendChild(versionSelect)
    selectContainer.appendChild(dropdownArrow)

    // Prevent dropdown click from selecting the card
    selectContainer.addEventListener("click", (e) => {
      e.stopPropagation()
    })

    // Handle version change
    versionSelect.addEventListener("change", (e) => {
      e.stopPropagation()
      const versionIndex = Number.parseInt(e.target.value)
      if (selectedRelease && selectedRelease.id === release.id) {
        selectVersion(versionIndex)
      }
    })

    cardBody.appendChild(selectContainer)

    // Add select button
    const selectButton = document.createElement("button")
    selectButton.className = "select-button"
    selectButton.textContent = "Select Release Channel"
    selectButton.addEventListener("click", (e) => {
      e.stopPropagation()
      selectRelease(release)
    })
    cardBody.appendChild(selectButton)

    card.appendChild(cardHeader)
    card.appendChild(cardBody)

    // Add click event to select this release
    card.addEventListener("click", () => {
      selectRelease(release)
    })

    wrapper.appendChild(card)
    releaseCardsContainer.appendChild(wrapper)
  })

  // Select the first release by default
  if (releases.length > 0) {
    selectRelease(releases[0])
  }
}

// Add a new function to select a version
function selectVersion(versionIndex) {
  if (!selectedRelease || !selectedRelease.versions || versionIndex >= selectedRelease.versions.length) {
    return
  }

  selectedVersion = selectedRelease.versions[versionIndex]
  addLog(`Selected version: ${selectedVersion.version}`)
}

// Update the selectRelease function to handle the new format
function selectRelease(release) {
  // Remove any existing GitHub containers
  const existingGithubContainers = document.querySelectorAll(".github-container")
  existingGithubContainers.forEach((container) => container.remove())

  selectedRelease = release

  // Update UI to show selected card
  const cards = document.querySelectorAll(".release-card")
  cards.forEach((card) => {
    if (card.dataset.id === release.id) {
      card.classList.add("selected")

      // Select the first version by default
      if (release.versions && release.versions.length > 0) {
        const versionSelect = document.getElementById(`version-select-${release.id}`)
        if (versionSelect) {
          versionSelect.selectedIndex = 0
          selectVersion(0)
        }
      }

      // Add GitHub button container below the selected card
      const githubContainer = document.createElement("div")
      githubContainer.className = "github-container"

      const githubLink = document.createElement("a")
      githubLink.href = release.githubUrl
      githubLink.className = "github-link"
      githubLink.target = "_blank"
      githubLink.innerHTML = `
        <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="github-icon">
          <path d="M9 19c-5 1.5-5-2.5-7-3m14 6v-3.87a3.37 3.37 0 0 0-.94-2.61c3.14-.35 6.44-1.54 6.44-7A5.44 5.44 0 0 0 20 4.77 5.07 5.07 0 0 0 19.91 1S18.73.65 16 2.48a13.38 13.38 0 0 0-7 0C6.27.65 5.09 1 5.09 1A5.07 5.07 0 0 0 5 4.77a5.44 5.44 0 0 0-1.5 3.78c0 5.42 3.3 6.61 6.44 7A3.37 3.37 0 0 0 9 18.13V22"></path>
        </svg>
        View on GitHub
      `

      githubContainer.appendChild(githubLink)

      // Insert the GitHub container after the card in the DOM
      card.after(githubContainer)

      // Ensure the GitHub container is properly positioned
      // by adding a wrapper if needed
      const cardParent = card.parentElement
      if (cardParent.classList.contains("release-cards-container")) {
        // Create a wrapper for the card and GitHub container
        const wrapper = document.createElement("div")
        wrapper.className = "release-card-wrapper"

        // Replace the card with the wrapper
        cardParent.replaceChild(wrapper, card)

        // Add the card and GitHub container to the wrapper
        wrapper.appendChild(card)
        wrapper.appendChild(githubContainer)
      }
    } else {
      card.classList.remove("selected")
    }
  })

  addLog(`Selected release: ${release.name}`)
}

// Add a step to the UI
function addStep(step, isCurrent = false) {
  // Check if step already exists
  if (document.getElementById(`step-${step}`)) {
    return
  }

  const stepElement = document.createElement("div")
  stepElement.className = "step"
  stepElement.id = `step-${step}`

  if (isCurrent) {
    stepElement.classList.add("current")
  } else {
    stepElement.classList.add("success")
  }

  const stepTitle = document.createElement("div")
  stepTitle.className = "step-title"
  stepTitle.textContent = StepNames[step] || step

  const stepLogs = document.createElement("div")
  stepLogs.className = "step-logs"
  stepLogs.id = `step-logs-${step}`

  stepElement.appendChild(stepTitle)
  stepElement.appendChild(stepLogs)

  stepsContainer.appendChild(stepElement)

  // Scroll to the new step
  stepElement.scrollIntoView({ behavior: "smooth", block: "center" })

  // Add to current steps array
  if (!currentSteps.includes(step)) {
    currentSteps.push(step)
  }
}

// Update the startInstallation function to use the selected version
function startInstallation(options) {
  // Reset steps
  currentSteps = []
  stepsContainer.innerHTML = ""

  // Convert options to query parameters
  const queryParams = new URLSearchParams()
  for (const key in options) {
    queryParams.append(key, options[key])
  }

  fetch(`/setOptions?${queryParams.toString()}`)
    .then((response) => {
      if (!response.ok) {
        throw new Error("Failed to set options")
      }
      return fetch("/start")
    })
    .then((response) => {
      if (!response.ok) {
        throw new Error("Failed to start installation")
      }
      isInstalling = true
      showProgressScreen()
    })
    .catch((error) => {
      addLog("Error: " + error.message)
    })
}

// Start uninstallation
function startUninstallation(options) {
  // Reset steps
  currentSteps = []
  stepsContainer.innerHTML = ""

  // Convert options to query parameters
  const queryParams = new URLSearchParams()
  for (const key in options) {
    queryParams.append(key, options[key])
  }

  fetch(`/setOptions?${queryParams.toString()}`)
    .then((response) => {
      if (!response.ok) {
        throw new Error("Failed to set options")
      }
      return fetch("/start")
    })
    .then((response) => {
      if (!response.ok) {
        throw new Error("Failed to start uninstallation")
      }
      isInstalling = true
      showProgressScreen()
    })
    .catch((error) => {
      addLog("Error: " + error.message)
    })
}

// Handle WebSocket messages
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
        // Handle null data case
        handleInstallationComplete()
        break

      case WebsocketMessageTypes.INSTALLATION_START:
        handleInstallationStart(message.data)
        break

      case WebsocketMessageTypes.INSTALLATION_ERROR:
        handleInstallationError(message.data)
        break

      default:
        console.log("Unknown message type:", message.type)
    }
  } catch (error) {
    console.error("Error parsing WebSocket message:", error)
  }
}

// Handle installation log message
function handleInstallLog(data) {
  if (data.isError) {
    addLog(`Error: ${data.message}${data.error ? ` - ${data.error}` : ""}`)
  } else {
    addLog(data.message)
  }
}

// Handle step log message
function handleStepLog(data) {
  const stepLogs = document.getElementById(`step-logs-${data.step}`)

  if (stepLogs) {
    const logEntry = document.createElement("div")
    logEntry.className = data.isError ? "step-log error" : "step-log"
    logEntry.textContent = data.isError ? `Error: ${data.message}${data.error ? ` - ${data.error}` : ""}` : data.message

    stepLogs.appendChild(logEntry)
    stepLogs.scrollTop = stepLogs.scrollHeight
  }
}

// Handle step update message
function handleStepUpdate(step) {
  // Mark previous steps as completed
  const steps = document.querySelectorAll(".step")
  steps.forEach((stepElement) => {
    stepElement.classList.remove("current")
    stepElement.classList.add("success")
  })

  // Add the new step as current
  addStep(step, true)
}

// Handle installation complete message
function handleInstallationComplete() {
  addLog("Installation completed successfully!")

  // Explicitly set installing to false
  isInstalling = false

  // Explicitly enable the back button
  backBtn.disabled = false
  backBtn.classList.remove("disabled")

  // Mark all steps as completed and remove the current class
  const steps = document.querySelectorAll(".step")
  steps.forEach((step) => {
    step.classList.remove("current")
    step.classList.add("success")
  })

  // Add a visual indicator that installation is complete
  const completionMessage = document.createElement("div")
  completionMessage.className = "completion-message"
  completionMessage.textContent = "✅ Installation Complete! You can now go back to the setup screen."
  stepsContainer.appendChild(completionMessage)

  // Update installation status
  checkIfInstalled()
}

// Handle installation start message
function handleInstallationStart(data) {
  addLog(`Starting ${data.action}...`)

  // Clear steps container
  stepsContainer.innerHTML = ""
  currentSteps = []

  // Add the first step if it exists
  if (data.steps && data.steps.length > 0) {
    addStep(data.steps[0], true)
  }
}

// Handle installation error message
function handleInstallationError(message) {
  addLog(`Installation failed: ${message}`)
  isInstalling = false

  // Enable back button on error too
  backBtn.disabled = false
  backBtn.classList.remove("disabled")

  // Mark current step as failed
  const steps = document.querySelectorAll(".step.current")
  steps.forEach((step) => {
    step.classList.remove("current")
    step.classList.add("error")
  })
}

// Add log to global log
function addLog(message) {
  const logEntry = document.createElement("div")
  logEntry.className = "log-entry"
  logEntry.textContent = `[${new Date().toLocaleTimeString()}] ${message}`

  globalLog.appendChild(logEntry)
  progressGlobalLog.appendChild(logEntry.cloneNode(true))

  // Scroll to bottom
  globalLog.scrollTop = globalLog.scrollHeight
  progressGlobalLog.scrollTop = progressGlobalLog.scrollHeight
}

// Show setup screen
function showSetupScreen() {
  setupScreen.classList.add("active")
  progressScreen.classList.remove("active")
}

// Show progress screen
function showProgressScreen() {
  setupScreen.classList.remove("active")
  progressScreen.classList.add("active")

  // Update back button state based on installation status
  updateBackButtonState()
}

// Add a new function to specifically update the back button state
function updateBackButtonState() {
  // Explicitly set the disabled attribute based on isInstalling
  backBtn.disabled = isInstalling

  // Also update the class for styling
  if (isInstalling) {
    backBtn.classList.add("disabled")
  } else {
    backBtn.classList.remove("disabled")
  }
}

// Initialize particles background
function initParticles() {
  const canvas = document.getElementById("particles")
  const ctx = canvas.getContext("2d")
  let particles = []

  // Resize canvas to fill window
  function resizeCanvas() {
    canvas.width = window.innerWidth
    canvas.height = window.innerHeight
  }

  // Create particles
  function createParticles() {
    particles = []
    const particleCount = Math.floor(window.innerWidth / 20)

    for (let i = 0; i < particleCount; i++) {
      particles.push({
        x: Math.random() * canvas.width,
        y: Math.random() * canvas.height,
        radius: Math.random() * 2 + 1,
        color: document.body.classList.contains("dark-theme") ? "#ffffff" : "#6366f1",
        speed: Math.random() * 0.5 + 0.1,
        direction: Math.random() * Math.PI * 2,
        opacity: Math.random() * 0.5 + 0.1,
      })
    }
  }

  // Draw particles
  function drawParticles() {
    ctx.clearRect(0, 0, canvas.width, canvas.height)

    particles.forEach((particle) => {
      ctx.beginPath()
      ctx.arc(particle.x, particle.y, particle.radius, 0, Math.PI * 2)
      ctx.fillStyle = particle.color
      ctx.globalAlpha = particle.opacity
      ctx.fill()

      // Move particle
      particle.x += Math.cos(particle.direction) * particle.speed
      particle.y += Math.sin(particle.direction) * particle.speed

      // Wrap around edges
      if (particle.x < 0) particle.x = canvas.width
      if (particle.x > canvas.width) particle.x = 0
      if (particle.y < 0) particle.y = canvas.height
      if (particle.y > canvas.height) particle.y = 0
    })

    // Connect nearby particles with lines
    connectParticles()

    requestAnimationFrame(drawParticles)
  }

  // Connect particles that are close to each other
  function connectParticles() {
    const maxDistance = 100

    for (let i = 0; i < particles.length; i++) {
      for (let j = i + 1; j < particles.length; j++) {
        const dx = particles[i].x - particles[j].x
        const dy = particles[i].y - particles[j].y
        const distance = Math.sqrt(dx * dx + dy * dy)

        if (distance < maxDistance) {
          const opacity = 1 - distance / maxDistance
          ctx.beginPath()
          ctx.moveTo(particles[i].x, particles[i].y)
          ctx.lineTo(particles[j].x, particles[j].y)
          ctx.strokeStyle = document.body.classList.contains("dark-theme") ? "#ffffff" : "#6366f1"
          ctx.globalAlpha = opacity * 0.2
          ctx.stroke()
        }
      }
    }
  }

  // Update particle colors when theme changes
  function updateParticleColors() {
    const color = document.body.classList.contains("dark-theme") ? "#ffffff" : "#6366f1"
    particles.forEach((particle) => {
      particle.color = color
    })
  }

  // Initialize
  resizeCanvas()
  createParticles()
  drawParticles()

  // Event listeners
  window.addEventListener("resize", () => {
    resizeCanvas()
    createParticles()
  })

  themeToggle.addEventListener("click", updateParticleColors)
}
