/**
 * @fileoverview Particle animation background
 */

import { isDarkTheme } from "./theme.js"

/**
 * Initializes the particle background animation
 */
export function initParticles() {
  const canvas = document.getElementById("particles")
  const ctx = canvas.getContext("2d")
  let particles = []

  /**
   * Resizes the canvas to match window dimensions
   */
  function resizeCanvas() {
    canvas.width = window.innerWidth
    canvas.height = window.innerHeight
  }

  /**
   * Creates particles for the animation
   */
  function createParticles() {
    particles = []
    const particleCount = Math.floor(window.innerWidth / 20)

    for (let i = 0; i < particleCount; i++) {
      particles.push({
        x: Math.random() * canvas.width,
        y: Math.random() * canvas.height,
        radius: Math.random() * 2 + 1,
        color: isDarkTheme() ? "#ffffff" : "#6366f1",
        speed: Math.random() * 0.5 + 0.1,
        direction: Math.random() * Math.PI * 2,
        opacity: Math.random() * 0.5 + 0.1,
      })
    }
  }

  /**
   * Draws particles and their connections
   */
  function drawParticles() {
    ctx.clearRect(0, 0, canvas.width, canvas.height)

    particles.forEach((particle) => {
      ctx.beginPath()
      ctx.arc(particle.x, particle.y, particle.radius, 0, Math.PI * 2)
      ctx.fillStyle = particle.color
      ctx.globalAlpha = particle.opacity
      ctx.fill()

      particle.x += Math.cos(particle.direction) * particle.speed
      particle.y += Math.sin(particle.direction) * particle.speed

      if (particle.x < 0) particle.x = canvas.width
      if (particle.x > canvas.width) particle.x = 0
      if (particle.y < 0) particle.y = canvas.height
      if (particle.y > canvas.height) particle.y = 0
    })

    connectParticles()
    requestAnimationFrame(drawParticles)
  }

  /**
   * Connects nearby particles with lines
   */
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
          ctx.strokeStyle = isDarkTheme() ? "#ffffff" : "#6366f1"
          ctx.globalAlpha = opacity * 0.2
          ctx.stroke()
        }
      }
    }
  }

  /**
   * Updates particle colors when theme changes
   */
  function updateParticleColors() {
    const color = isDarkTheme() ? "#ffffff" : "#6366f1"
    particles.forEach((particle) => {
      particle.color = color
    })
  }

  resizeCanvas()
  createParticles()
  drawParticles()

  window.addEventListener("resize", () => {
    resizeCanvas()
    createParticles()
  })

  const themeToggle = document.querySelector(".theme-toggle")
  themeToggle.addEventListener("click", updateParticleColors)
}
