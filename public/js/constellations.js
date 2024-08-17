import { createWorld } from './world.js'

let canvas = document.getElementById('main-canvas')
canvas.width = window.innerWidth
canvas.height = window.innerHeight

let canvasCtx = canvas.getContext('2d')
canvasCtx.fillStyle = 'red'

let world = createWorld({ particles: 20, size: [canvas.width, canvas.height] })
world.draw(canvasCtx)

console.log(canvasCtx, world)
