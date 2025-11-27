/**
 * @fileoverview Step management for installation progress display
 */

import { StepNames } from "./constants.js"
import { addCurrentStep, clearCurrentSteps } from "./app-state.js"

/**
 * Adds a step to the progress display
 * @param {string} step - The step identifier
 * @param {boolean} [isCurrent=false] - Whether this is the current step
 */
export function addStep(step, isCurrent = false) {
  if (document.getElementById(`step-${step}`)) {
    return
  }

  const stepsContainer = document.getElementById("steps-container")
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
  stepElement.scrollIntoView({ behavior: "smooth", block: "center" })

  addCurrentStep(step)
}

/**
 * Marks all steps as completed
 */
export function markStepsAsComplete() {
  const steps = document.querySelectorAll(".step")
  steps.forEach((step) => {
    step.classList.remove("current")
    step.classList.add("success")
  })
}

/**
 * Marks the current step as failed
 */
export function markCurrentStepAsError() {
  const steps = document.querySelectorAll(".step.current")
  steps.forEach((step) => {
    step.classList.remove("current")
    step.classList.add("error")
  })
}

/**
 * Clears all steps from the display
 */
export function clearSteps() {
  const stepsContainer = document.getElementById("steps-container")
  stepsContainer.innerHTML = ""
  clearCurrentSteps()
}
