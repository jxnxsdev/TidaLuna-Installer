/**
 * @fileoverview Release channel and version management
 */

import { setSelectedRelease, setSelectedVersion, getSelectedRelease } from "./app-state.js"
import { addLog } from "./logger.js"

/**
 * Renders release cards to the UI
 * @param {Array} releases - Array of release objects
 */
export function renderReleaseCards(releases) {
  const releaseCardsContainer = document.getElementById("release-cards")

  if (!releases || !releases.length) {
    addLog("No release channels available")
    releaseCardsContainer.innerHTML =
      '<div class="release-card"><div class="release-card-header"><h3>No releases found</h3></div></div>'
    return
  }

  releaseCardsContainer.innerHTML = ""

  releases.forEach((release) => {
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

    const selectContainer = document.createElement("div")
    selectContainer.className = "custom-select-container"

    const versionSelect = document.createElement("select")
    versionSelect.className = "version-select"
    versionSelect.id = `version-select-${release.id}`

    if (release.versions && release.versions.length > 0) {
      release.versions.forEach((version, index) => {
        const option = document.createElement("option")
        option.value = index
        option.textContent = version.version
        versionSelect.appendChild(option)
      })
    } else {
      const option = document.createElement("option")
      option.value = "-1"
      option.textContent = "No versions available"
      option.disabled = true
      versionSelect.appendChild(option)
      versionSelect.disabled = true
    }

    const dropdownArrow = document.createElement("div")
    dropdownArrow.className = "dropdown-arrow"
    dropdownArrow.innerHTML = "▼"

    selectContainer.appendChild(versionSelect)
    selectContainer.appendChild(dropdownArrow)

    selectContainer.addEventListener("click", (e) => {
      e.stopPropagation()
    })

    versionSelect.addEventListener("change", (e) => {
      e.stopPropagation()
      const versionIndex = Number.parseInt(e.target.value)
      if (getSelectedRelease() && getSelectedRelease().id === release.id) {
        selectVersion(versionIndex)
      }
    })

    cardBody.appendChild(selectContainer)

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

    card.addEventListener("click", () => {
      selectRelease(release)
    })

    wrapper.appendChild(card)
    releaseCardsContainer.appendChild(wrapper)
  })

  if (releases.length > 0) {
    selectRelease(releases[0])
  }
}

/**
 * Selects a specific version from a release
 * @param {number} versionIndex - The index of the version to select
 */
export function selectVersion(versionIndex) {
  const selectedRelease = getSelectedRelease()
  if (!selectedRelease || !selectedRelease.versions || versionIndex >= selectedRelease.versions.length) {
    return
  }

  const selectedVersion = selectedRelease.versions[versionIndex]
  setSelectedVersion(selectedVersion)
  addLog(`Selected version: ${selectedVersion.version}`)
}

/**
 * Selects a release channel
 * @param {Object} release - The release object to select
 */
export function selectRelease(release) {
  const existingGithubContainers = document.querySelectorAll(".github-container")
  existingGithubContainers.forEach((container) => container.remove())

  setSelectedRelease(release)

  const cards = document.querySelectorAll(".release-card")
  cards.forEach((card) => {
    if (card.dataset.id === release.id) {
      card.classList.add("selected")

      if (release.versions && release.versions.length > 0) {
        const versionSelect = document.getElementById(`version-select-${release.id}`)
        if (versionSelect) {
          versionSelect.selectedIndex = 0
          selectVersion(0)
        }
      }

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
      card.after(githubContainer)

      const cardParent = card.parentElement
      if (cardParent.classList.contains("release-cards-container")) {
        const wrapper = document.createElement("div")
        wrapper.className = "release-card-wrapper"

        cardParent.replaceChild(wrapper, card)
        wrapper.appendChild(card)
        wrapper.appendChild(githubContainer)
      }
    } else {
      card.classList.remove("selected")
    }
  })

  addLog(`Selected release: ${release.name}`)
}
