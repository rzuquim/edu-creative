/**
 * @typedef {Object} Particle
 * @property {number} x - horizontal position
 * @property {number} y - vertical position
 * @property {number } radius - particle size (default: 15)
 */

/**
 * @typedef {Object} ParticleOpts
 * @property {[number, number]} worldSize - x and y max coordinates in pixels
 * @property {number | undefined } radius - particle size (default: 15)
 */

/**
 * Creates a random particle bound by a canvas size
 * @param {ParticleOpts} opts - options to create a particle
 * @returns {Particle}
 **/
export function createParticle(opts) {
    let [maxX, maxY] = opts.worldSize

    return {
        x: Math.random() * maxX,
        y: Math.random() * maxY,
        radius: opts.radius || 15,
    }
}

/**
 * Draws a particle on a given canvas
 * @param {Particle} particle
 * @param {CanvasRenderingContext2D} canvasCtx
 * @returns {void}
 **/
export function drawParticle(particle, canvasCtx) {
    console.debug('drawing particle', particle, canvasCtx)
    canvasCtx.beginPath()
    canvasCtx.arc(particle.x, particle.y, particle.radius, 0, FULL_CIRCLE)
    canvasCtx.fill()
}

const FULL_CIRCLE = 2 * Math.PI
