import { createParticle } from './particles.js'

let canvas = document.getElementById('main-canvas')
canvas.width = window.innerWidth
canvas.height = window.innerHeight

let ctx = canvas.getContext('2d')
let particle = createParticle()

console.log(ctx, particle)
