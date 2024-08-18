import { createParticle, drawParticle, updateParticle } from './particles.js'

/**
 * @typedef {Object} World - main holds particles
 * @property {function(CanvasRenderingContext2D): void} draw - Draws a frame of the world moving particles around
 */

/**
 * @typedef {Object} WorldOpts
 * @property {number} particles - how many particles should be created
 * @property {[number, number]} size - the size of the world where the particle lives (in px)
 */

/**
 * Creates a world full of happy particles (also sets up the global canvas config)
 * @param {WorldOpts} opts - options to setup the world
 * @param {CanvasRenderingContext2D} canvasCtx
 * @returns {World}
 **/
export function createWorld(opts, canvasCtx) {
    let particles = []
    for (let i = 0; i < opts.particles; i++) {
        particles.push(createParticle({ worldSize: opts.size }))
    }

    canvasCtx.strokeStyle = 'white'
    canvasCtx.fillStyle = 'white'

    return {
        draw: function (canvasCtx) {
            drawConnections(particles, canvasCtx)
            for (let particle of particles) {
                drawParticle(particle, canvasCtx)
                updateParticle(particle, canvasCtx)
            }
        },
    }
}

/**
 * Draws lines between particles
 * @param {import('./particles.js').Particle[]} particles
 * @param {CanvasRenderingContext2D} canvasCtx
 * @returns {void}
 **/
function drawConnections(particles, canvasCtx) {
    // NOTE: since we are changing the globalAlpha when drawing the connection
    canvasCtx.save()
    for (let i = 0; i < particles.length - 1; i++) {
        for (let j = i + 1; j < particles.length; j++) {
            let a = particles[i]
            let b = particles[j]
            let dist = distance(a, b)
            if (dist <= CONNECTION_DISTANCE) {
                drawConnection(a, b, dist, canvasCtx)
            }
        }
    }
    canvasCtx.restore()
}

/** Geometric distance between two particles
 * @param {import('./particles.js').Particle} particleA
 * @param {import('./particles.js').Particle} particleB
 * @returns {number}
 **/
function distance(particleA, particleB) {
    return Math.sqrt(Math.pow(particleA.x - particleB.x, 2) + Math.pow(particleA.y - particleB.y, 2))
}

/** Draws a line between two particles
 * @param {import('./particles.js').Particle} a
 * @param {import('./particles.js').Particle} b
 * @param {number} dist
 * @param {CanvasRenderingContext2D} canvasCtx
 **/
function drawConnection(a, b, dist, canvasCtx) {
    let ratio = dist / CONNECTION_DISTANCE
    let opacity = 1 - ratio
    canvasCtx.lineWidth = LINE_WIDTH - (ratio * LINE_WIDTH)
    canvasCtx.globalAlpha = opacity
    canvasCtx.beginPath()
    canvasCtx.moveTo(a.x, a.y)
    canvasCtx.lineTo(b.x, b.y)
    canvasCtx.stroke()
}

const CONNECTION_DISTANCE = 80
const LINE_WIDTH = 5
