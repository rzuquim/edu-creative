/**
 * @typedef {Object} Particle
 * @property {number} x - horizontal position of the center of the particle
 * @property {number} y - vertical position of the center of the particle
 * @property {number} maxX - horizontal maximum bound (world size)
 * @property {number} maxY - vertical maximum bound (world size)
 * @property {number} vx - horizontal velocity
 * @property {number} vy - verticla velocity
 * @property {number } radius - particle size (default: 15)
 */

/**
 * @typedef {Object} ParticleOpts
 * @property {[number, number]} worldSize - x and y max coordinates in pixels
 */

/**
 * Creates a random particle bound by a canvas size
 * @param {ParticleOpts} opts - options to create a particle
 * @returns {Particle}
 **/
export function createParticle(opts) {
    let [worldWidth, worldHeight] = opts.worldSize
    let radius = Math.random() * 10 + 3

    let maxX = worldWidth - radius
    let maxY = worldHeight - radius

    return {
        x: radius + Math.random() * (worldWidth - 2 * radius),
        y: radius + Math.random() * (worldHeight - 2 * radius),
        vx: Math.random() * 1 - 0.5,
        vy: Math.random() * 1 - 0.5,
        radius,
        maxX,
        maxY,
    }
}

/**
 * Draws a particle on a given canvas
 * @param {Particle} particle
 * @param {CanvasRenderingContext2D} canvasCtx
 * @returns {void}
 **/
export function drawParticle(particle, canvasCtx) {
    canvasCtx.beginPath()
    canvasCtx.arc(particle.x, particle.y, particle.radius, 0, FULL_CIRCLE)
    canvasCtx.fill()
}

/**
 * Updates the particle positions inside its world
 * @param {Particle} particle
 * @returns {void}
 **/
export function updateParticle(particle) {
    particle.x += particle.vx
    particle.y += particle.vy

    // keeping the particle inside the world bounds
    if (particle.x <= particle.radius || particle.x >= particle.maxX) {
        particle.vx *= -1
    }

    if (particle.y <= particle.radius || particle.y >= particle.maxY) {
        particle.vy *= -1
    }
}

const FULL_CIRCLE = 2 * Math.PI
