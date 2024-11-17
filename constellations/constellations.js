import { createWorld } from './world.js'

let canvas = document.getElementById('main-canvas')
canvas.width = window.innerWidth
canvas.height = window.innerHeight

let canvasCtx = canvas.getContext('2d')

let world = createWorld(
    { particles: 300, size: [canvas.width, canvas.height] },
    canvasCtx,
)

function animate() {
    canvasCtx.clearRect(0, 0, canvas.width, canvas.height)
    world.draw(canvasCtx)
    requestAnimationFrame(animate)
}

animate()

