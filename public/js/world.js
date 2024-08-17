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

    canvasCtx.strokeStyle = 'black'
    canvasCtx.fillStyle = 'red'

    return {
        draw: function (ctx) {
            for (let particle of particles) {
                drawParticle(particle, ctx)
                updateParticle(particle, ctx)
            }
        },
    }
}
