import { createParticle, drawParticle } from './particles.js'

/**
 * @typedef {Object} World - main holds particles
 * @property {function(CanvasRenderingContext2D): void} draw - Draws the world on a given canvas
 */

/**
 * @typedef {Object} WorldOpts
 * @property {number} particles - how many particles should be created
 * @property {[number, number]} size - the size of the world where the particle lives (in px)
 */

/**
 * Creates a world full of happy particles
 * @param {WorldOpts} opts - options to setup the world
 * @returns {World}
 **/
export function createWorld(opts) {
    let particles = []
    for (let i = 0; i < opts.particles; i++) {
        particles.push(createParticle({ worldSize: opts.size }))
    }

    return {
        draw: function (ctx) {
            for (let particle of particles) {
                drawParticle(particle, ctx)
            }
        },
    }
}
