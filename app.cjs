const createError = require('http-errors')
const express = require('express')
const path = require('path')
const cookieParser = require('cookie-parser')
const logger = require('morgan')

const dotenv = require('dotenv')
dotenv.config()

let app = express()

// view engine setup
app.set('views', path.join(__dirname, 'views'))
app.set('view engine', 'hbs')

app.use(logger('dev'))
app.use(express.json())
app.use(express.urlencoded({ extended: false }))
app.use(cookieParser())
app.use(express.static(path.join(__dirname, 'public')))

// live reload
if (process.env.NODE_ENV === 'development') {
    const livereload = require('livereload')
    const connectLiveReload = require('connect-livereload')

    let liveReloadServer = livereload.createServer()
    liveReloadServer.watch(path.join(__dirname, 'public'))
    liveReloadServer.server.once('connection', () => {
        setTimeout(() => {
            liveReloadServer.refresh('/')
        }, 100)
    })
    app.use(connectLiveReload())
}

// routes
let router = express.Router()
router.get('/', (_req, res, _next) => res.render('index', { title: 'Creative Projects' }))
router.get('/constellations', (_req, res, _next) => res.render('constellations', { title: 'Constellations' }))
app.use('/', router)

// catch 404 and forward to error handler
app.use(function (_req, _res, next) {
    next(createError(404))
})

// error handler
app.use(function (err, req, res, _next) {
    // set locals, only providing error in development
    res.locals.message = err.message
    res.locals.error = req.app.get('env') === 'development' ? err : {}

    // render the error page
    res.status(err.status || 500)
    res.render('error')
})

module.exports = app
